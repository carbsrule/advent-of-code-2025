use std::collections::HashMap;

#[derive(Debug)]
struct Device {
    outputs: Vec<String>,
}
type DeviceMap = HashMap<String, Device>;

fn read_devices(lines: Vec<String>) -> DeviceMap {
    let mut devices = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let mut device = Device { outputs: vec![] };
        for output in parts[1].trim().split(" ") {
            device.outputs.push(output.to_string());
        }
        devices.insert(parts[0].to_string(), device);
    }
    return devices;
}

fn solve(devices: &DeviceMap, device: &Device) -> u32 {
    let mut paths = 0;

    if device.outputs.len() == 1 && device.outputs[0] == "out" {
        return 1;
    }

    let mut sum = 0;
    for i in 0..device.outputs.len() {
        let next_device_name = &device.outputs[i];
        let has_next_device = devices.get(next_device_name);
        match has_next_device {
            None => (),
            Some(next_device) => {
                sum += solve(devices, next_device);
            }
        }
    }
    return sum;
}

pub fn part1(lines: Vec<String>) {
    let devices = read_devices(lines);
    let num_paths = solve(&devices, &devices.get("you").unwrap());
    println!("Paths: {:?}", num_paths);
}
