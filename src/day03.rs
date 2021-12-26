use std::ops::Add;

pub fn part1(input: String) {
    let input = input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(2).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let totals = input.iter()
        .fold(
            vec![0; 12],
            |acc, i| acc.iter().zip(i).map(|p| p.0 + p.1).collect::<Vec<u32>>());

    let gamma_rate_binary = totals.iter().fold(String::new(), |acc, &t| if t > 500 as u32 {acc.add("1")} else { acc.add("0")});
    let gamma_rate = u32::from_str_radix(&gamma_rate_binary, 2).unwrap();
    let epsilon_rate_binary = gamma_rate_binary.chars().map(|c| if c == '0' {'1'} else {'0'}).collect::<String>();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate_binary, 2).unwrap();

    let power_consumption = gamma_rate * epsilon_rate;

    println!("Power Consumption: {}", power_consumption)
}