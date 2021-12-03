// use Regex::{Captures, Regex};
// const pattern: &str = r"^$";

// Probably usize, String, or Regex::Captures
type Item = String;

fn main() {
    // let re = Regex::new(pattern).unwrap();

    let input = std::fs::read_to_string("./input.txt")
        .unwrap()
        .split('\n')
        .filter_map(|s| s.parse::<Item>().ok())
        // .flat_map(|s| re.captures(s))
        .filter(|s| !s.is_empty())
        .collect::<Vec<Item>>();

    // println!("{:#?}", input);

    solution1(&input);
    solution2(&input);
}

fn solution1(lines: &[Item]) {}

fn solution2(lines: &[Item]) {}
