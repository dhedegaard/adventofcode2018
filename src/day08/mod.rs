#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    pub fn sum_metadata(&self) -> usize {
        self.metadata.iter().sum::<usize>() + self.children.iter().map(|c| part1(&c)).sum::<usize>()
    }

    pub fn value(&self) -> usize {
        // No children, sum the metadata.
        if self.children.is_empty() {
            return self.metadata.iter().sum();
        }

        // Otherwise, use metadata as index in children, and sum the selected children.
        self.metadata
            .iter()
            .filter(|&e| e > &0 && e - 1 < self.children.len())
            .map(|&e| self.children[e - 1].value())
            .sum()
    }
}

pub fn get_input() -> String {
    include_str!("input.txt").to_owned()
}

pub fn parse_input(input: &str) -> Node {
    parse_node(&mut input.split_whitespace().map(|e| e.parse().unwrap()))
}

fn parse_node(iter: &mut impl Iterator<Item = usize>) -> Node {
    let child_nodes = iter.next().unwrap();
    let meta_nodes = iter.next().unwrap();

    Node {
        children: (0..child_nodes).map(|_| parse_node(iter)).collect(),
        metadata: iter.take(meta_nodes).collect(),
    }
}

pub fn part1(root: &Node) -> usize {
    root.sum_metadata()
}

pub fn part2(root: &Node) -> usize {
    root.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 138);
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&parse_input(&get_input())), 43825);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 66);
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&parse_input(&get_input())), 19276);
    }
}
