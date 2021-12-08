pub mod day01;
pub mod day06;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u8) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        6 => (day06::part1, day06::part2),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        },
    };
}
