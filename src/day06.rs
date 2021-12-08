pub fn part1(input: String) {
    let input = input.trim().split(',').map(|l| l.parse().unwrap()).collect::<Vec<i8>>();
    let day_indexed_input= index_input(&input);

    let simulation_result = simulation(&day_indexed_input, 80);

    println!("Total Fish: {}", simulation_result.iter().sum::<u64>());
}

pub fn part2(input: String) {
    let input = input.trim().split(',').map(|l| l.parse().unwrap()).collect::<Vec<i8>>();
    let day_indexed_map= index_input(&input);

    let simulation_result = simulation(&day_indexed_map, 256);

    println!("Total Fish: {}", simulation_result.iter().sum::<u64>());
}

fn index_input(input: &Vec<i8>) -> Vec<u64> {
    let mut day_indexed_map = vec![0; 9];
    for i in 0..9 {
        day_indexed_map[i] = input.iter().filter(|&&f| f == i as i8).count() as u64
    }
    day_indexed_map
}

fn simulation(input_state: &Vec<u64>, total_iterations: u64) -> Vec<u64> {
    let mut current_state = input_state.clone();
    for _ in 0..total_iterations {
        current_state = simulate_day(&current_state);
    }

    current_state
}

fn simulate_day(start_of_day_state: &Vec<u64>) -> Vec<u64> {
    let mut end_of_day_state = start_of_day_state.clone();
    let spawned_fish = start_of_day_state[0];
    end_of_day_state.rotate_left(1);
    end_of_day_state[6] = end_of_day_state[6] + spawned_fish;

    end_of_day_state
}