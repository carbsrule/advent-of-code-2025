mod args;
mod days;
mod lines;
mod num;

/**
 * Usage: cargo run DAY PART < INPUT_FILE
 * E.g.: cargo run 1 2 < day01_full.txt
 */

fn main() {
    let (day, part) = args::get_day_part();
    days::run(day, part);
}
