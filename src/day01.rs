pub fn part1(input: String) {
    let input = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<i32>>();

    let increase_count = input.iter()
        .zip(&input[1..])
        .fold(0, |acc, pair| if pair.1 > pair.0 {acc + 1} else {acc});

    println!("Total Increases: {}", increase_count);
}
