pub fn part_1(input: String) {
    let input = parse_input(&input);

    let translations = input.iter()
        .map(|&i| match i.0 {
            "forward" => (i.1, 0),
            "up" => (0, i.1 * -1),
            "down" => (0, i.1),
            _ => panic!("Unknown instruction")
        });

    let final_location = translations.fold((0, 0), |location, translation| (location.0 + translation.0, location.1 + translation.1));

    println!("Depth: {}\nForward Progression: {}\nTotal Distance: {}",
             final_location.0,
             final_location.1,
             final_location.0 * final_location.1);
}

pub fn part_2(input: String) {
    let input = parse_input(&input);

    let operations: Vec<Box<dyn Fn((i32, i32, i32)) -> (i32, i32, i32)>> = input.iter()
        .map(|&i| match i.0 {
            "up" => Box::new(move |state: (i32, i32, i32)| (state.0, state.1, state.2 + (i.1 * -1))) as Box<dyn Fn((i32, i32, i32)) -> (i32, i32, i32)>,
            "down" => Box::new(move |state: (i32, i32, i32)| (state.0, state.1, state.2 + i.1)),
            "forward" => Box::new(move |state: (i32, i32, i32)| (state.0 + i.1, state.1 + (i.1 * state.2), state.2)),
            _ => panic!("Unknown instruction")
        }).collect();

    let final_state = operations.iter().fold((0, 0, 0), |location, operation| operation(location));

    println!("Depth: {}\nForward Progression: {}\nTotal Distance: {}",
             final_state.0,
             final_state.1,
             final_state.0 * final_state.1);
}

fn parse_input(input: &str) -> Vec<(&str, i32)> {
    input.lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|p| (p[0], p[1].parse::<i32>().unwrap()))
        .collect::<Vec<(&str, i32)>>()
}

