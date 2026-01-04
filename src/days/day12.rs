mod present;
mod area;

use present::*;
use area::*;

const DISPLAY_AREAS: bool = false;

#[derive(PartialEq)]
enum Reading {
    Blank,
    Present,
}

fn load_area(line: String) -> Area {
    let parts: Vec<&str> = line.split(":").collect();
    let area_parts: Vec<&str> = parts[0].split("x").collect();
    let width: usize = area_parts[0].parse().expect("Width must be a number");
    let height: usize = area_parts[1].parse().expect("Height must be a number");
    let mut area = Area::new(height, width);

    // let mut total_qty: u16 = 0;
    let present_quantities = parts[1].trim().split(" ");
    for qty_str in present_quantities {
        let qty: u8 = qty_str.parse().expect("Quantity of presents must be a number");
        // total_qty += qty as u16;
        area.presents_reqd.push(qty);
    }
    // println!("Area has total qty: {total_qty}");

    return area;
}

fn load_data(lines: Vec<String>) -> (Vec<Present>, Vec<Area>) {
    let mut areas = vec![];
    let mut presents = vec![];
    let mut present_lines = vec![];
    let mut mode = Reading::Blank;
    let mut line_num = 0;
    for line in lines {
        line_num += 1;
        if line == "" && mode == Reading::Present {
            presents.push(Present::new(presents.len() as u8, present_lines));
            mode = Reading::Blank;
            present_lines = vec![];
            continue;
        }
        if line.contains("x") {
            if mode != Reading::Blank {
                panic!("How did we get here?");
            }
            areas.push(load_area(line));
            continue;
        }
        if mode == Reading::Blank && line.contains(":") {
            mode = Reading::Present;
            continue;
        }
        if mode == Reading::Present {
            present_lines.push(line);
            continue;
        }
        panic!("Failed reading input at line {line_num}");
    }

    return (presents, areas);
}


pub fn part1(lines: Vec<String>) {
    let (presents, mut areas) = load_data(lines);
    let mut sum_fillable = 0;
    let num_areas = areas.len();
    for i in 0..areas.len() {
        let area = &mut areas[i];
        println!("Area {}/{num_areas} ({}x{}):", i+1, area.width, area.height);
        if area.can_hold_presents(&presents) {
            sum_fillable += 1;
            println!("Fillable");
        } else {
            println!("Too full");
        }
        if DISPLAY_AREAS {
            area.print();
        }
        println!();
    }
    println!("Regions that can fit all presents: {sum_fillable}");
}
