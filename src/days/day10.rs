use std::fmt;
use std::io;
use std::io::Write;

const DEBUG_OUTPUT: bool = false;

type JoltageLevel = u16;
type JoltageLevels = Vec<JoltageLevel>;

struct Machine {
    reqd_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    reqd_joltage: JoltageLevels,
    max_presses: JoltageLevel,
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut light_str = "".to_string();
        for i in &self.reqd_lights {
            let out = if *i { "#" } else { "." };
            light_str.push_str(out);
        }
        return write!(f, "[{light_str}]");
    }
}

fn flush() {
    io::stdout().flush().expect("Err on flush");
}

struct Incrementer {
    values: JoltageLevels,
    end_height: JoltageLevel,
    height: JoltageLevel,
    pos: usize,
    complete: bool,
}

impl Incrementer {
    pub fn new(end_height: JoltageLevel, width: usize) -> Incrementer {
        if width < 2 {
            panic!("Garbage in, garbage out");
        }
        let mut values = vec![];
        for _ in 0..width {
            values.push(0);
        }
        return Incrementer {
            values,
            end_height,
            height: 1,
            pos: width - 2,
            complete: false,
        };
    }

    pub fn next(&mut self) -> Vec<u16> {
        let width = self.values.len();
        if self.values[self.pos] < self.height {
            self.values[self.pos] += 1;
            if self.pos == 1
                && self.values[self.pos] == self.end_height
                && self.values[self.pos - 1] == self.end_height
            {
                self.complete = true;
            }
        } else {
            let mut reached_zero = true;
            for next_pos in (0..self.pos).rev() {
                if self.values[next_pos] < self.height {
                    self.values[next_pos] += 1;
                    for remaining in next_pos + 1..=width - 2 {
                        self.values[remaining] = 0;
                    }
                    self.pos = width - 2;
                    reached_zero = false;
                    break;
                }
            }
            if reached_zero && self.values[0] == self.height {
                for i in 0..width - 2 {
                    self.values[i] = 0;
                }
                self.values[width - 2] += 1;
                self.height += 1;
                self.pos = width - 3;
                if self.height > self.end_height {
                    self.complete = true;
                }
            }
        }

        return self.values.clone();
    }
}

fn new_machine() -> Machine {
    return Machine {
        reqd_lights: vec![],
        buttons: vec![],
        reqd_joltage: vec![],
        max_presses: 65535,
    };
}

fn read_lights(src: &str, machine: &mut Machine) {
    for i in 0..src.len() {
        match &src[i..i + 1] {
            "." => machine.reqd_lights.push(false),
            "#" => machine.reqd_lights.push(true),
            _ => (),
        }
    }
}

fn read_buttons(src: &str, machine: &mut Machine) {
    let parts: Vec<&str> = src.trim_matches([')', ' ']).split(") ").collect();
    for part in parts {
        let mut toggles = vec![];
        let nums = part.trim_matches(['(', ' ']).split(",");
        for num in nums {
            toggles.push(num.parse().expect("Must be a number"));
        }
        machine.buttons.push(toggles);
    }
}

fn read_joltage(src: &str, machine: &mut Machine) {
    let parts: Vec<&str> = src.trim_matches(['}', ' ']).split(",").collect();
    for part in parts {
        machine
            .reqd_joltage
            .push(part.parse().expect("Must be a number"));
    }
}

fn read_manual(lines: Vec<String>) -> Vec<Machine> {
    let mut machines = vec![];
    for line in lines {
        let mut machine = new_machine();
        let parts: Vec<&str> = line.split("]").collect();
        read_lights(parts[0], &mut machine);
        let parts: Vec<&str> = parts[1].trim().split("{").collect();
        read_buttons(parts[0], &mut machine);
        read_joltage(parts[1], &mut machine);
        machines.push(machine);
    }
    return machines;
}

struct ResultState {
    lights: Vec<bool>,
    presses: Vec<usize>,
}

fn toggle_lights(lights: &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
    let mut new_lights = lights.clone();
    for light_num in button {
        new_lights[*light_num] = !new_lights[*light_num];
    }
    return new_lights;
}

fn lights_match(lights: &Vec<bool>, reqd_lights: &Vec<bool>) -> bool {
    for i in 0..reqd_lights.len() {
        if lights[i] != reqd_lights[i] {
            return false;
        }
    }
    return true;
}

fn presses_to_on(machine: &Machine) -> u32 {
    let mut num_presses = 1;
    let mut press_results = vec![];
    let mut lights = vec![];
    for _ in 0..machine.reqd_lights.len() {
        lights.push(false);
    }
    press_results.push(ResultState {
        lights,
        presses: vec![],
    });
    loop {
        let mut new_results = vec![];
        for i in 0..machine.buttons.len() {
            let button = &machine.buttons[i];
            for state in &press_results {
                let new_lights = toggle_lights(&state.lights, button);
                if lights_match(&new_lights, &machine.reqd_lights) {
                    return num_presses;
                }
                let mut new_presses = state.presses.clone();
                new_presses.push(i);
                new_results.push(ResultState {
                    lights: new_lights,
                    presses: new_presses,
                })
            }
        }
        press_results = new_results;
        num_presses += 1;
        if num_presses > 100 {
            return 1_000_000;
        }
    }
}

pub fn part1(lines: Vec<String>) {
    let machines = read_manual(lines);
    let mut presses = 0;
    for machine in machines {
        presses += presses_to_on(&machine);
    }

    println!("Num presses: {presses}");
}

#[derive(Clone)]
struct ButtonPress {
    button_idx: usize,
    num_presses: JoltageLevel,
}
impl fmt::Debug for ButtonPress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}x{}", self.button_idx, self.num_presses);
    }
}

#[derive(Clone, Debug)]
struct JoltageResult {
    joltage: JoltageLevels,
    total_presses: u16,
    presses: Vec<ButtonPress>,
}

impl JoltageResult {
    fn new(machine: &Machine) -> JoltageResult {
        return JoltageResult {
            joltage: init_joltage(machine),
            total_presses: 0,
            presses: vec![],
        };
    }
}

fn calc_highest_joltage_pos(machine: &Machine) -> (usize) {
    let mut highest_joltage_pos = 0;
    let mut highest_joltage = 0;
    for i in 0..machine.reqd_joltage.len() {
        if machine.reqd_joltage[i] > highest_joltage {
            highest_joltage = machine.reqd_joltage[i];
            highest_joltage_pos = i;
        }
    }
    return highest_joltage_pos;
}

fn sort_buttons(buttons: &mut Vec<Vec<usize>>, highest_joltage_pos: usize) {
    buttons.sort_by(|a, b| {
        if a.len() > b.len() {
            return std::cmp::Ordering::Less;
        } else if a.len() > b.len() {
            return std::cmp::Ordering::Greater;
        } else if a.contains(&highest_joltage_pos) {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Equal;
    });
}

// Calculate the max number of times a button can be pressed without exceeding the required joltage
fn calc_max_button_presses(
    button_idx: usize,
    machine: &Machine,
    current_joltage: &JoltageLevels,
) -> u16 {
    let mut min_joltage = 65535;
    for joltage_idx in &machine.buttons[button_idx] {
        let joltage = machine.reqd_joltage[*joltage_idx] - current_joltage[*joltage_idx];
        if joltage < min_joltage {
            min_joltage = joltage;
        }
    }
    return min_joltage;
}

fn init_joltage(machine: &Machine) -> JoltageLevels {
    let mut initial_joltage = vec![];
    for _ in 0..machine.reqd_joltage.len() {
        initial_joltage.push(0);
    }
    return initial_joltage;
}

const INFINITE_PRESSES: u16 = 65535;

fn calc_joltage_presses_with_offset(machine: &Machine, offsets: &JoltageLevels) -> JoltageResult {
    let mut resultant_joltage = JoltageResult::new(machine);
    // println!("Pressing...");
    for button_idx in 0..machine.buttons.len() {
        let mut num_presses =
            calc_max_button_presses(button_idx, machine, &resultant_joltage.joltage);
        // println!("    Button {button_idx}:{:?} x{num_presses}", machine.buttons[button_idx]);

        if num_presses <= offsets[button_idx] {
            // Can't press the button at all
            num_presses = 0;
        } else {
            num_presses -= offsets[button_idx];
        }

        resultant_joltage.presses.push(ButtonPress {
            button_idx,
            num_presses,
        });
        resultant_joltage.total_presses += num_presses;

        for joltage_idx in &machine.buttons[button_idx] {
            resultant_joltage.joltage[*joltage_idx] += num_presses;
        }
    }
    if DEBUG_OUTPUT {
        println!(
            "Joltage: {:?} | Offsets: {:?} | Buttons pressed: {:?}",
            resultant_joltage.joltage, offsets, resultant_joltage.presses
        );
    }
    // println!("Return: {:?}", resultant_joltage);
    return resultant_joltage;
}

// TODO: rename since this isn't recursive now
fn recursive_calc_joltage_presses(machine: &Machine, offset_height: JoltageLevel) -> JoltageResult {
    let mut offsets: JoltageLevels = vec![];
    for _ in 0..machine.buttons.len() {
        offsets.push(0);
    }
    let res = calc_joltage_presses_with_offset(machine, &offsets);
    if res.joltage == machine.reqd_joltage {
        return res;
    }

    if offset_height > 0 {
        let mut inc = Incrementer::new(offset_height, machine.buttons.len());
        while !inc.complete {
            let offsets = inc.next();
            let res = calc_joltage_presses_with_offset(machine, &offsets);
            if res.joltage == machine.reqd_joltage {
                return res;
            }
        }
    }

    return res;
}

fn calc_joltage_from_buttons(machine: &mut Machine) -> Option<JoltageResult> {
    let initial_joltage = init_joltage(machine);
    // let mut button_max_presses = vec![];
    let mut max_presses = 0;
    for button_idx in 0..machine.buttons.len() {
        let presses = calc_max_button_presses(button_idx, machine, &initial_joltage);
        if presses > max_presses {
            max_presses = presses;
        }
        // button_max_presses.push(presses);
    }
    machine.max_presses = max_presses;
    println!("Max presses: {max_presses}");

    // for offset_height in 0..=machine.max_presses {
    //     offsets[0] = offset_height;
    let result = recursive_calc_joltage_presses(machine, machine.max_presses);
    if result.joltage == machine.reqd_joltage {
        println!("Result: {:?}", result);
        return Some(result);
    }
    // }

    return None;
}

fn calc_joltage_presses(mut machine: Machine) -> JoltageLevel {
    println!("Machine");
    let highest_joltage_pos = calc_highest_joltage_pos(&machine);
    // println!("Highest joltage: {highest_joltage} at {highest_joltage_pos}");
    sort_buttons(&mut machine.buttons, highest_joltage_pos);

    println!("    Buttons: {:?}", machine.buttons);
    println!("    Required joltage: {:?}", machine.reqd_joltage);
    // let joltage_affected_by_buttons = calc_joltage_by_buttons(&machine);
    // println!("    Joltage affected by buttons: {:?}", joltage_affected_by_buttons);

    let maybe_res = calc_joltage_from_buttons(&mut machine);
    match maybe_res {
        Some(res) => {
            println!("Final result: {:?}", res);
            return res.total_presses;
        }
        _ => return INFINITE_PRESSES,
    }
}

pub fn part2(lines: Vec<String>) {
    let machines = read_manual(lines);
    let mut total_presses = 0;
    let mut machine_num = 1;
    for machine in machines {
        let presses = calc_joltage_presses(machine);
        print!("Machine {machine_num}: {presses} presses");
        flush();
        total_presses += presses;
        println!(" (total: {total_presses})");
        machine_num += 1;
    }
    println!("Total presses: {total_presses}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lights_on() {
        let machine = Machine {
            reqd_lights: vec![true],
            buttons: vec![vec![0]],
            reqd_joltage: vec![],
            max_presses: 0,
        };
        assert_eq!(presses_to_on(&machine), 1);

        let machine = Machine {
            reqd_lights: vec![true, true],
            buttons: vec![vec![0], vec![1]],
            reqd_joltage: vec![],
            max_presses: 0,
        };
        assert_eq!(presses_to_on(&machine), 2);

        let machine = Machine {
            reqd_lights: vec![true, true, false],
            buttons: vec![vec![0], vec![1, 2], vec![2]],
            reqd_joltage: vec![],
            max_presses: 0,
        };
        assert_eq!(presses_to_on(&machine), 3);
    }

    #[test]
    fn test_joltage() {
        let machine = Machine {
            reqd_lights: vec![true, true],
            buttons: vec![vec![1]],
            reqd_joltage: vec![0, 1],
            max_presses: 0,
        };
        assert_eq!(calc_joltage_presses(machine), 1);

        let machine = Machine {
            reqd_lights: vec![true, true],
            buttons: vec![vec![1], vec![1, 2], vec![2]],
            reqd_joltage: vec![0, 2, 3],
            max_presses: 0,
        };
        assert_eq!(calc_joltage_presses(machine), 3);

        let machine = Machine {
            reqd_lights: vec![true, true],
            buttons: vec![vec![0, 1], vec![1, 2], vec![0, 2]],
            // presses: 2, 3, 7 = 12
            reqd_joltage: vec![5, 9, 10],
            max_presses: 0,
        };
        let presses = calc_max_button_presses(0, &machine, &vec![0, 0, 0]);
        assert_eq!(presses, 5);
        assert_eq!(calc_joltage_presses(machine), 12);
    }
}
