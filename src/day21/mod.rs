use super::day16::exec;
use super::day19::Instruction;
use std::collections::HashSet;

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn part1(ipr: usize, instructions: &[Instruction]) -> usize {
    let mut regs = [0; 6];
    let mut ip = 0;
    while ip < instructions.len() {
        let inst = &instructions[ip];
        exec(inst.inst, inst.in1, inst.in2, inst.out, &mut regs);
        if ip == 28 {
            return regs[3];
        }
        regs[ipr] += 1;
        ip = regs[ipr] as usize;
    }
    unreachable!();
}

pub fn part2(ipr: usize, instructions: &[Instruction]) -> usize {
    let mut regs = [0; 6];
    let mut ip = 0;
    let mut prev = 0;
    let mut seen = HashSet::new();
    while ip < instructions.len() {
        let inst = &instructions[ip];
        exec(inst.inst, inst.in1, inst.in2, inst.out, &mut regs);
        if ip == 28 {
            let r3 = regs[3];
            if !seen.insert(r3) {
                break;
            }
            prev = r3;
        }
        regs[ipr] += 1;
        ip = regs[ipr] as usize;
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;
    use day19::parse_input;

    #[test]
    fn part1_result() {
        let (ip, insts) = parse_input(&get_input());
        assert_eq!(part1(ip, &insts), 13_270_004);
    }

    #[test]
    fn part2_result() {
        let (ip, insts) = parse_input(&get_input());
        assert_eq!(part2(ip, &insts), 12_879_142);
    }
}
