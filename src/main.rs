fn main() {
    let input = std::fs::read_to_string("./")
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // println!("{:?}", input);

    solution1(&input);
    solution2(&input);
}

fn solution1(lines: &[_]) {}

fn solution2(lines: &[_]) {}
