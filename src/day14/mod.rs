pub fn get_input() -> usize {
    704321
}

pub fn part1(input: usize) -> String {
    let mut recipes = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    while recipes.len() < input + 10 {
        let total = recipes[elf1] + recipes[elf2];
        if total >= 10 {
            recipes.push(total / 10);
        }
        recipes.push(total % 10);
        elf1 = (elf1 + recipes[elf1] + 1) % recipes.len();
        elf2 = (elf2 + recipes[elf2] + 1) % recipes.len();
    }
    recipes
        .iter()
        .skip(input % recipes.len())
        .take(10)
        .map(|&e| format!("{}", e))
        .collect()
}

pub fn part2(input: &str) -> usize {
    let parsed_input = &input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()[..];
    let mut recipes = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    let result;
    loop {
        let mut score = recipes[elf1] + recipes[elf2];
        let mut double = false;
        if score >= 10 {
            double = true;
            recipes.push(score / 10);
            score %= 10;
        }
        recipes.push(score);
        elf1 = (elf1 + 1 + recipes[elf1]) % recipes.len();
        elf2 = (elf2 + 1 + recipes[elf2]) % recipes.len();
        let rel_l = recipes.len();
        let in_l = parsed_input.len();
        if rel_l > in_l && &recipes[rel_l - in_l..rel_l] == parsed_input {
            result = rel_l - in_l;
            break;
        }
        if double && rel_l > in_l + 1 && &recipes[rel_l - 1 - in_l..rel_l - 1] == parsed_input {
            result = rel_l - 1 - in_l;
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(9), "5158916779");
        assert_eq!(part1(5), "0124515891");
        assert_eq!(part1(18), "9251071085");
        assert_eq!(part1(2018), "5941429882");
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(get_input()), "1741551073");
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("51589"), 9);
        assert_eq!(part2("01245"), 5);
        assert_eq!(part2("92510"), 18);
        assert_eq!(part2("59414"), 2018);
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&format!("{}", get_input())), 20322683);
    }
}
