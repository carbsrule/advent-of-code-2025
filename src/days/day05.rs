const DEBUG_OUTPUT: bool = true;

fn is_fresh(ingredient: u64, fresh_ingredients: &Vec<(u64, u64)>) -> bool {
    if DEBUG_OUTPUT {
        print!("Ingredient {ingredient}");
    }
    let found = false;
    for fresh_range in fresh_ingredients {
        if ingredient >= fresh_range.0 && ingredient <= fresh_range.1 {
            if DEBUG_OUTPUT && !found {
                println!(": fresh");
            }
            return true;
        }
    }
    if DEBUG_OUTPUT && !found {
        println!(": stale");
    }
    return false;
}

pub fn part1(lines: Vec<String>) {
    let mut fresh_ingredients: Vec<(u64, u64)> = vec![];
    let mut num_fresh = 0;
    for line in lines {
        if line == "" {
            continue;
        }
        let mut parts = line.split("-");
        let el1 = parts.next().unwrap();
        let el2 = parts.next();

        match el2 {
            Some(el2_value) => {
                let range_start = el1.parse().expect("Must be a number");
                let range_end = el2_value.parse().expect("Must be a number");
                if DEBUG_OUTPUT {
                    println!("Fresh range: {range_start}-{range_end}")
                }
                fresh_ingredients.push((range_start, range_end));
            }
            None => {
                let ingredient: u64 = el1.parse().expect("Must be a number");
                if is_fresh(ingredient, &fresh_ingredients) {
                    num_fresh += 1;
                }
            }
        }
    }
    println!("Fresh ingredients: {num_fresh}");
}
