extern crate regex;

use std::collections::{HashMap, HashSet};

pub fn get_input_part1() -> String {
    include_str!("input1.txt").to_owned()
}

pub fn get_input_part2() -> Vec<Vec<usize>> {
    include_str!("input2.txt")
        .lines()
        .map(|line| line_to_vec(line))
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Inst {
    /// add register
    Addr,
    /// add immediate
    Addi,
    /// multiply register
    Mulr,
    /// multiply immediate
    Muli,
    /// bitwise and register
    Banr,
    /// bitwise and immediate
    Bani,
    /// bitwise or register
    Borr,
    /// bitwise or immediate
    Bori,
    /// set register
    Setr,
    /// set immediate
    Seti,
    /// greater than immediate/register
    Gtir,
    /// greater than register/immediate
    Gtri,
    /// greater than register/register
    Gtrr,
    /// equal immediate/register
    Eqir,
    /// equal register/immediate
    Eqri,
    /// equal register/register
    Eqrr,
}

impl Inst {
    /// Converts opcode to instruction.
    pub fn from_usize(inst: usize) -> Inst {
        match inst {
            0x0 => Inst::Addr,
            0x1 => Inst::Addi,
            0x2 => Inst::Mulr,
            0x3 => Inst::Muli,
            0x4 => Inst::Banr,
            0x5 => Inst::Bani,
            0x6 => Inst::Borr,
            0x7 => Inst::Bori,
            0x8 => Inst::Setr,
            0x9 => Inst::Seti,
            0xa => Inst::Gtir,
            0xb => Inst::Gtri,
            0xc => Inst::Gtrr,
            0xd => Inst::Eqir,
            0xe => Inst::Eqri,
            0xf => Inst::Eqrr,
            _ => panic!("Bad opcode: {}", inst),
        }
    }
}

#[derive(Debug)]
pub struct Part1Input {
    /// The registers before executing the instruction.
    before: Vec<usize>,
    /// The instruction, in decimal format. (inst, input1, input2, output)
    inst: Vec<usize>,
    /// The registers after executing the instruction.
    after: Vec<usize>,
}

fn line_to_vec(input: &str) -> Vec<usize> {
    let match_nums = regex::Regex::new(".*?(\\d+).+?(\\d+).+?(\\d+).+?(\\d+).*").unwrap();
    let captures = match_nums.captures(input).unwrap();
    (1..captures.len())
        .map(|e| captures[e].parse().unwrap())
        .collect()
}

pub fn parse_input(input: &str) -> Vec<Part1Input> {
    let mut result = vec![];
    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        result.push(Part1Input {
            before: line_to_vec(lines.next().unwrap()),
            inst: line_to_vec(lines.next().unwrap()),
            after: line_to_vec(lines.next().unwrap()),
        });
        lines.next();
    }
    result
}

pub fn exec(opcode: Inst, in1: usize, in2: usize, out: usize, reg: &mut [usize]) {
    reg[out] = match opcode {
        Inst::Addr => reg[in1] + reg[in2],
        Inst::Addi => reg[in1] + in2,
        Inst::Mulr => reg[in1] * reg[in2],
        Inst::Muli => reg[in1] * in2,
        Inst::Banr => reg[in1] & reg[in2],
        Inst::Bani => reg[in1] & in2,
        Inst::Borr => reg[in1] | reg[in2],
        Inst::Bori => reg[in1] | in2,
        Inst::Setr => reg[in1],
        Inst::Seti => in1,
        Inst::Gtir => (in1 > reg[in2]) as usize,
        Inst::Gtri => (reg[in1] > in2) as usize,
        Inst::Gtrr => (reg[in1] > reg[in2]) as usize,
        Inst::Eqir => (in1 == reg[in2]) as usize,
        Inst::Eqri => (reg[in1] == in2) as usize,
        Inst::Eqrr => (reg[in1] == reg[in2]) as usize,
    }
}

pub fn part1(input: &mut [Part1Input]) -> usize {
    let mut result = 0;
    for elem in input {
        let mut count = 0;
        for opcode in 0x0..=0xf {
            let opcode = Inst::from_usize(opcode);
            let mut reg = elem.before.clone();
            exec(opcode, elem.inst[1], elem.inst[2], elem.inst[3], &mut reg);
            if reg == elem.after.as_slice() {
                count += 1;
            }
        }
        if count >= 3 {
            result += 1;
        }
    }
    result
}

/// Calculates the opcode <-> instruction mappings, based on the input.
pub fn calc_mapping(input: &mut [Part1Input]) -> HashMap<usize, Inst> {
    // Maps opcodes -> possible instructions.
    let mut possible_opcodes = HashMap::new();
    for elem in input {
        let mut possible = HashSet::new();
        for opcode in 0x0..=0xf {
            let opcode = Inst::from_usize(opcode);
            let mut reg = elem.before.clone();
            exec(opcode, elem.inst[1], elem.inst[2], elem.inst[3], &mut reg);
            if reg == elem.after.as_slice() {
                possible.insert(opcode);
            }
        }
        // Fetch the valid instructions set for the given opcode (all of them if it's the firt time).
        let mut value = possible_opcodes
            .entry(elem.inst[0])
            .or_insert_with(|| (0x0..=0xf).map(Inst::from_usize).collect::<HashSet<_>>());
        // Filter away any non-valid instructions for the given opcode.
        value.retain(|e| possible.contains(e));
    }

    // Maps specific opcodes -> instructions.
    let mut mapping = HashMap::new();
    // the instructions that have been mapped.
    let mut mapped_instructions = HashSet::new();
    loop {
        if possible_opcodes.is_empty() {
            break;
        }
        let mut matched = None;
        // Go looking for opcodes that point to a single instruction.
        for opcode in 0x0..=0xf {
            let mut insts = possible_opcodes.get_mut(&opcode);
            if insts.is_none() {
                continue;
            }
            let mut insts = insts.unwrap();
            if insts.len() == 1 {
                let inst = insts.iter().next().unwrap().to_owned();
                mapping.insert(opcode, inst);
                mapped_instructions.insert(inst);
                matched = Some((opcode, inst));
            }
        }
        // Remove the inst from the possible opcodes map.
        if matched.is_some() {
            let (opcode, inst) = matched.unwrap();
            possible_opcodes.remove(&opcode);
            for insts in possible_opcodes.values_mut() {
                insts.remove(&inst);
            }
        }
    }
    mapping
}

pub fn part2(input: &mut [Part1Input], input2: &[Vec<usize>]) -> usize {
    // Determine the opcode mapping.
    let mapping = calc_mapping(input);

    // Execute the instructions.
    let mut reg = vec![0; 4];
    for line in input2 {
        exec(mapping[&line[0]], line[1], line[2], line[3], &mut reg);
    }
    reg[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = include_str!("test1.txt");

    #[test]
    fn part1_examples() {
        let mut input = parse_input(TEST_INPUT1);
        assert_eq!(part1(&mut input), 1);
    }

    #[test]
    fn part1_result() {
        let mut input = parse_input(&get_input_part1());
        assert_eq!(part1(&mut input), 663);
    }

    #[test]
    fn part2_result() {
        assert_eq!(
            part2(&mut parse_input(&get_input_part1()), &get_input_part2()),
            525
        );
    }
}
