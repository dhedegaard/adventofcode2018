use super::day16::{exec, Inst};

#[derive(Debug)]
pub struct Instruction {
    pub inst: Inst,
    pub in1: usize,
    pub in2: usize,
    pub out: usize,
}

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn parse_input(input: &str) -> (usize, Vec<Instruction>) {
    let mut lines = input.lines();
    let ip = lines
        .next()
        .unwrap()
        .split_whitespace()
        .rev()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let instructions = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            Instruction {
                inst: match parts.next().unwrap() {
                    "addr" => Inst::Addr,
                    "addi" => Inst::Addi,
                    "mulr" => Inst::Mulr,
                    "muli" => Inst::Muli,
                    "banr" => Inst::Banr,
                    "bani" => Inst::Bani,
                    "borr" => Inst::Borr,
                    "bori" => Inst::Bori,
                    "setr" => Inst::Setr,
                    "seti" => Inst::Seti,
                    "gtir" => Inst::Gtir,
                    "gtrr" => Inst::Gtrr,
                    "eqir" => Inst::Eqir,
                    "eqri" => Inst::Eqri,
                    "eqrr" => Inst::Eqrr,
                    _ => unreachable!(),
                },
                in1: parts.next().unwrap().parse().unwrap(),
                in2: parts.next().unwrap().parse().unwrap(),
                out: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect();
    (ip, instructions)
}

pub fn part1(ip: usize, instructions: &[Instruction]) -> usize {
    let mut reg = vec![0; 6];
    while reg[ip] < instructions.len() {
        let inst = &instructions[reg[ip]];
        exec(inst.inst, inst.in1, inst.in2, inst.out, &mut reg);
        reg[ip] += 1;
    }
    reg[0]
}

pub fn part2(ip: usize, instructions: &[Instruction]) -> usize {
    let mut reg = vec![1, 0, 0, 0, 0, 0];
    while reg[ip] != 1 {
        let inst = &instructions[reg[ip]];
        exec(inst.inst, inst.in1, inst.in2, inst.out, &mut reg);
        reg[ip] += 1;
    }
    // Expect all the inner loops to do the same thing, sum() the difference in the outer loops and
    // return the sum.
    let seed = *reg.iter().max().unwrap();
    (1..=seed).filter(|e| seed % e == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn part1_examples() {
        let (ip, instructions) = parse_input(TEST_INPUT);
        assert_eq!(part1(ip, &instructions), 7);
    }

    #[test]
    fn part1_result() {
        let (ip, instructions) = parse_input(&get_input());
        assert_eq!(part1(ip, &instructions), 912);
    }

    #[test]
    fn part2_result() {
        let (ip, instructions) = parse_input(&get_input());
        assert_eq!(part2(ip, &instructions), 10576224);
    }
}
