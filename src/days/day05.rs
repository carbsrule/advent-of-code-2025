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

fn load_ranges(lines: Vec<String>) -> Vec<(u64, u64)> {
    let mut fresh_ranges: Vec<(u64, u64)> = vec![];
    for line in lines {
        if line == "" {
            break;
        }
        let mut parts = line.split("-");
        let el1 = parts.next().unwrap();
        let el2 = parts.next();
        match el2 {
            None => continue,
            Some(value) => {
                let range_start: u64 = el1.parse().expect("Must be a number");
                let range_end: u64 = value.parse().expect("Must be a number");
                fresh_ranges.push((range_start, range_end));
            }
        }
    }
    return fresh_ranges;
}

fn ranges_overlap(range1: &(u64, u64), range2: &(u64, u64)) -> (bool, u64, u64) {
    let mut updated = false;
    let mut new_start = range1.0;
    let mut new_end = range1.1;

    // contiguous ranges
    if range1.1 + 1 == range2.0 {
        return (true, range1.0, range2.1);
    }

    // range2 overlaps range1 start
    if range2.0 <= range1.0 && range2.1 >= range1.0 {
        new_start = range2.0;
        updated = true;
    }

    // range2 overlaps range1 end (can coincide with overlapping start)
    if range2.1 >= range1.1 && range2.0 <= range1.1 {
        new_end = range2.1;
        updated = true;
    }
    return (updated, new_start, new_end);
}

pub fn part2(lines: Vec<String>) {
    let mut fresh_ranges: Vec<(u64, u64)> = load_ranges(lines);
    println!("Fresh ranges: {:?}", fresh_ranges);
    loop {
        let mut merged = vec![];
        for i in 0..fresh_ranges.len() {
            for j in 0..fresh_ranges.len() {
                if i == j {
                    continue;
                }
                let (overlap, new_start, new_end) =
                    ranges_overlap(&fresh_ranges[i], &fresh_ranges[j]);
                if overlap {
                    if DEBUG_OUTPUT {
                        println!("Merge {:?} and {:?}", fresh_ranges[i], fresh_ranges[j])
                    }
                    fresh_ranges[i].0 = new_start;
                    fresh_ranges[i].1 = new_end;
                    merged.push(j);
                }
            }
            if merged.len() > 0 {
                break;
            }
        }
        if merged.len() > 0 {
            merged.reverse();
            for i in merged {
                fresh_ranges.remove(i);
            }
        } else {
            break;
        }
    }
    println!("Fresh ingredient ranges: {:?}", fresh_ranges);
    let mut total_ids = 0;
    for range in fresh_ranges {
        total_ids += range.1 - range.0 + 1;
    }
    println!("Fresh ingredient IDs: {:?}", total_ids);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        assert_eq!(ranges_overlap(&(1, 2), &(3, 5)), (true, 1, 5)); // contiguous
        assert_eq!(ranges_overlap(&(3, 5), &(1, 3)), (true, 1, 5)); // equal start
        assert_eq!(ranges_overlap(&(3, 5), &(1, 4)), (true, 1, 5)); // overlap start
        assert_eq!(ranges_overlap(&(3, 5), &(5, 7)), (true, 3, 7)); // equal end
        assert_eq!(ranges_overlap(&(3, 5), &(4, 7)), (true, 3, 7)); // overlap end
        assert_eq!(ranges_overlap(&(3, 5), &(3, 5)), (true, 3, 5)); // matching
        assert_eq!(ranges_overlap(&(3, 5), &(2, 6)), (true, 2, 6)); // complete overlap
        assert_eq!(ranges_overlap(&(1, 3), &(5, 6)).0, false); // no overlap
        assert_eq!(ranges_overlap(&(5, 6), &(1, 3)).0, false); // no overlap
    }
}
