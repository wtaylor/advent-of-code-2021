pub fn part1(input: String) {
    let input = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<i32>>();

    let increase_count = input.iter()
        .zip(&input[1..])
        .fold(0, |acc, pair| if pair.1 > pair.0 {acc + 1} else {acc});

    println!("Total Increases: {}", increase_count);
}

pub fn part2(input: String) {
    let input = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<i32>>();

    let window_totals = (0..input.len()-2)
        .map(|i| &input[i..i+3])
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>();

    let increase_count = window_totals.iter()
        .zip(&window_totals[1..])
        .fold(0, |acc, pair| if pair.1 > pair.0 {acc+1} else {acc});

    println!("Total Increases: {}", increase_count)
}
