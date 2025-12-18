use std::fmt;

struct Machine {
    reqd_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u32>,
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

fn new_machine() -> Machine {
    return Machine {
        reqd_lights: vec![],
        buttons: vec![],
        joltage: vec![],
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
            .joltage
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lights_on() {
        let machine = Machine {
            reqd_lights: vec![true],
            buttons: vec![vec![0]],
            joltage: vec![],
        };
        assert_eq!(presses_to_on(&machine), 1);

        let machine = Machine {
            reqd_lights: vec![true, true],
            buttons: vec![vec![0], vec![1]],
            joltage: vec![],
        };
        assert_eq!(presses_to_on(&machine), 2);

        let machine = Machine {
            reqd_lights: vec![true, true, false],
            buttons: vec![vec![0], vec![1, 2], vec![2]],
            joltage: vec![],
        };
        assert_eq!(presses_to_on(&machine), 3);
    }
}
