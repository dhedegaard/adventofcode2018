#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DmgType {
    Cold,
    Fire,
    Radiation,
    Slashing,
    Bludgeoning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    Immune,
    Infection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Army {
    team: Team,
    units: isize,
    hp: isize,
    weak: Vec<DmgType>,
    immune: Vec<DmgType>,
    ap: isize,
    att_type: DmgType,
    initiative: isize,
}

impl Army {
    fn ep(&self) -> isize {
        self.units * self.ap
    }

    fn calculate_damage(&self, power: isize, dmg_type: DmgType) -> isize {
        let mul = if self.immune.contains(&dmg_type) {
            0
        } else if self.weak.contains(&dmg_type) {
            2
        } else {
            1
        };
        mul * power
    }

    fn attack(&mut self, power: isize, dmg_type: DmgType) -> isize {
        let damage = self.calculate_damage(power, dmg_type);
        if damage == 0 {
            return 0;
        }
        let killed = (damage / self.hp).min(self.units);
        self.units -= killed;
        killed
    }
}

fn parse_dmg_type(input: &str) -> DmgType {
    match input.to_ascii_lowercase().as_ref() {
        "cold" => DmgType::Cold,
        "fire" => DmgType::Fire,
        "radiation" => DmgType::Radiation,
        "slashing" => DmgType::Slashing,
        "bludgeoning" => DmgType::Bludgeoning,
        _ => unreachable!("input: {}", input),
    }
}

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn parse_input(input: &str) -> Vec<Army> {
    let m_army = regex::Regex::new(r"(\d+) units each with (\d+) hit points \(?(.*?)\)? ?with an attack that does (\d+) (.*?) damage at initiative (\d+)").unwrap();
    let mut result = vec![];
    let mut team = Team::Immune;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.ends_with(":") {
            team = match line {
                "Immune System:" => Team::Immune,
                "Infection:" => Team::Infection,
                _ => unreachable!(line),
            };
            continue;
        }

        let captures = m_army.captures(line).unwrap();
        let modifiers = captures[3]
            .split(|e| e == ',' || e == ';')
            .map(|e| e.trim());
        let mut weaknesses = vec![];
        let mut immunities = vec![];
        let mut parsing_immunities = true;
        for elem in modifiers {
            let elems = elem.split_whitespace().collect::<Vec<_>>();
            if elems.len() > 1 && elems[1] == "to" {
                parsing_immunities = match elems[0] {
                    "weak" => false,
                    "immune" => true,
                    _ => unreachable!(),
                }
            }
            if !elems.is_empty() {
                let dmg_type = parse_dmg_type(elems[elems.len() - 1]);
                if parsing_immunities {
                    immunities.push(dmg_type);
                } else {
                    weaknesses.push(dmg_type);
                }
            }
        }

        result.push(Army {
            team,
            units: captures[1].parse().unwrap(),
            hp: captures[2].parse().unwrap(),
            weak: weaknesses,
            immune: immunities,
            ap: captures[4].parse().unwrap(),
            att_type: parse_dmg_type(&captures[5]),
            initiative: captures[6].parse().unwrap(),
        })
    }
    result
}

fn battle(armies: &[Army]) -> (Option<Team>, isize) {
    let mut armies = armies.to_vec();
    loop {
        let mut attackers = (0..armies.len()).collect::<Vec<usize>>();
        attackers.sort_by_key(|&idx| (-armies[idx].ep(), -armies[idx].initiative));
        let mut targets = vec![None; armies.len()];
        for idx_atk in attackers {
            let atk = &armies[idx_atk];
            if atk.units <= 0 { continue; }
            targets[idx_atk] = armies.iter().enumerate()
                .filter(|(idx, def)| def.team != atk.team && def.units > 0 && !targets.contains(&Some(*idx)))
                .max_by_key(|(_idx, def)| {
                    (def.calculate_damage(atk.ep(), atk.att_type), def.ep(), def.initiative)
                })
                .map(|(idx, _def)| idx);
        }
        let mut attackers = (0..armies.len()).collect::<Vec<usize>>();
        attackers.sort_by_key(|&idx| -armies[idx].initiative);
        let mut total_killed = 0;
        for idx_atk in attackers {
            if armies[idx_atk].units <= 0 { continue; }
            if let Some(idx_def) = targets[idx_atk] {
                let (ap, dt) = {
                    let atk = &armies[idx_atk];
                    (atk.ep(), atk.att_type)
                };
                total_killed += armies[idx_def].attack(ap, dt);
            }
        }

        // Check for draw or statemate (ie immune vs immune).
        if total_killed == 0 {
            return (None, 0);
        }

        // Otherwise determine the winner, if one of the teams have been defeated.
        let alive_infections = armies.iter()
            .filter(|&e| e.team == Team::Infection)
            .map(|e| e.units).sum();
        let alive_immunes = armies.iter()
            .filter(|&e| e.team == Team::Immune)
            .map(|e| e.units).sum();
        if alive_infections == 0 {
            return (Some(Team::Immune), alive_immunes);
        }
        if alive_immunes == 0 {
            return (Some(Team::Infection), alive_infections);
        }
    }
}

pub fn part1(input: &[Army]) -> isize {
    let (_team, part1) = battle(input);
    part1
}

pub fn part2(input: &[Army]) -> isize {
    (0..)
        .filter_map(|boost| {
            let mut armies = input.to_vec();
            for army in armies.iter_mut().filter(|army| army.team == Team::Immune) {
                army.ap += boost;
            }
            let (team, rest) = battle(&armies);
            if team.is_some() && team.unwrap() == Team::Immune {
                Some(rest)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_parse() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(input.len(), 4);
        assert_eq!(
            input[0],
            Army {
                team: Team::Immune,
                units: 17,
                hp: 5390,
                weak: vec![DmgType::Radiation, DmgType::Bludgeoning],
                immune: vec![],
                ap: 4507,
                att_type: DmgType::Fire,
                initiative: 2,
            }
        );
        assert_eq!(
            input[1],
            Army {
                team: Team::Immune,
                units: 989,
                hp: 1274,
                weak: vec![DmgType::Bludgeoning, DmgType::Slashing],
                immune: vec![DmgType::Fire],
                ap: 25,
                att_type: DmgType::Slashing,
                initiative: 3,
            }
        );
        assert_eq!(
            input[2],
            Army {
                team: Team::Infection,
                units: 801,
                hp: 4706,
                weak: vec![DmgType::Radiation],
                immune: vec![],
                ap: 116,
                att_type: DmgType::Bludgeoning,
                initiative: 1,
            }
        );
    }

    #[test]
    fn part1_examples() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 5216);
    }

    #[test]
    fn part1_result() {
        let input = parse_input(&get_input());
        assert_eq!(part1(&input), 14799);
    }

    #[test]
    fn part2_examples() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 51);
    }
}
