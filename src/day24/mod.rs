use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DmgType {
    None,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Army {
    team: Team,
    units: isize,
    hp: isize,
    weak: [DmgType; 4],
    immune: [DmgType; 4],
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
            weak: vec_to_arr(&weaknesses),
            immune: vec_to_arr(&immunities),
            ap: captures[4].parse().unwrap(),
            att_type: parse_dmg_type(&captures[5]),
            initiative: captures[6].parse().unwrap(),
        })
    }
    result
}

/// Converts a slice of DmgType to an array with 4 DmgTypes, filled with None's at the end.
fn vec_to_arr(input: &[DmgType]) -> [DmgType; 4] {
    let mut input = input.to_vec();
    while input.len() < 4 {
        input.push(DmgType::None);
    }
    let mut result = [DmgType::None; 4];
    result.copy_from_slice(&input);
    result
}

fn battle(armies: &[Army]) -> (Option<Team>, isize) {
    let mut armies = armies.to_vec();
    loop {
        armies.sort_by_key(|v| Reverse((v.units * v.ap, v.initiative)));
        let mut targets: Vec<Option<usize>> = vec![None; armies.len()];
        for (j, u) in armies.iter().enumerate() {
            let mut best = 0;
            for (i, v) in armies.iter().enumerate() {
                if u.team == v.team || targets.contains(&Some(i)) || v.units == 0 {
                    continue;
                }
                if v.calculate_damage(u.ep(), u.att_type) > best {
                    best = v.calculate_damage(u.ep(), u.att_type);
                    targets[j] = Some(i);
                };
            }
        }
        let mut attackers = (0..armies.len()).collect::<Vec<_>>();
        attackers.sort_by_key(|&idx| Reverse(armies[idx].initiative));
        let mut any_die = false;
        for atk_idx in attackers {
            if armies[atk_idx].units == 0 {
                continue;
            }
            if let Some(j) = targets[atk_idx] {
                let atk = armies[atk_idx];
                let mut def = armies[j];
                let killed = def.attack(atk.ep(), atk.att_type);
                if killed > 0 {
                    any_die = true;
                }
                armies[j] = def;
            }
        }

        if !any_die {
            return (None, 0);
        }

        let alive = armies.iter().fold((0, 0), |mut teams, army| {
            if army.team == Team::Immune {
                teams.0 += army.units;
            } else {
                teams.1 += army.units;
            }
            teams
        });
        if alive == (0, 0) {
            return (None, 0);
        } else if alive.0 == 0 {
            return (Some(Team::Infection), alive.1);
        } else if alive.1 == 0 {
            return (Some(Team::Immune), alive.0);
        }
    }
}

pub fn part1(input: &[Army]) -> isize {
    let (_team, part1) = battle(input);
    part1
}

pub fn part2(input: &[Army]) -> isize {
    let armies = input.to_vec();
    (1..)
        .filter_map(|b| {
            let mut armies = armies.clone();
            armies
                .iter_mut()
                .filter(|u| u.team == Team::Immune)
                .for_each(|u| u.ap += b);
            match battle(&armies) {
                (Some(Team::Immune), rem) => Some(rem),
                _ => None,
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
                weak: vec_to_arr(&vec![DmgType::Radiation, DmgType::Bludgeoning]),
                immune: vec_to_arr(&vec![]),
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
                weak: vec_to_arr(&vec![DmgType::Bludgeoning, DmgType::Slashing]),
                immune: vec_to_arr(&vec![DmgType::Fire]),
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
                weak: vec_to_arr(&vec![DmgType::Radiation]),
                immune: vec_to_arr(&vec![]),
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

    #[test]
    fn part2_result() {
        let input = parse_input(&get_input());
        assert_eq!(part2(&input), 4428);
    }
}
