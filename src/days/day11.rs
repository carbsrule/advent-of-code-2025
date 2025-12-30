use std::collections::HashMap;

#[derive(Debug)]
struct Device {
    name: String,
    outputs: Vec<String>,
}
type DeviceMap = HashMap<String, Device>;

type ResultValue = u64;

#[derive(Clone, Debug)]
struct DescendentStats {
    both: ResultValue,
    fft: ResultValue,
    dac: ResultValue,
    neither: ResultValue,
}
type DeviceResultMap = HashMap<String, DescendentStats>;

impl DescendentStats {
    fn new() -> DescendentStats {
        return DescendentStats {
            both: 0,
            fft: 0,
            dac: 0,
            neither: 0,
        };
    }
}

fn read_devices(lines: Vec<String>) -> DeviceMap {
    let mut devices = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let name = parts[0].to_string();
        let mut device = Device {
            name: name.clone(),
            outputs: vec![],
        };
        for output in parts[1].trim().split(" ") {
            device.outputs.push(output.to_string());
        }
        devices.insert(name, device);
    }
    return devices;
}

fn solve(devices: &DeviceMap, device: &Device) -> u32 {
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

fn solve2(
    devices: &DeviceMap,
    device: &Device,
    seen: Vec<String>,
    known: &mut DeviceResultMap,
) -> DescendentStats {
    let known_result = known.get(&device.name);
    match known_result {
        None => (),
        Some(result) => return result.clone(),
    }

    println!("Device {} via path: {:?}", device.name, seen);
    if device.outputs.len() == 1 && device.outputs[0] == "out" {
        let mut result = DescendentStats::new();
        match device.name.clone() {
            val if val == "fft" => result.fft = 1,
            val if val == "dac" => result.dac = 1,
            _ => result.neither = 1,
        }
        known.insert(device.name.clone(), result.clone());
        return result;
    }

    let mut stats = DescendentStats::new();
    for i in 0..device.outputs.len() {
        let next_device_name = &device.outputs[i];
        let has_next_device = devices.get(next_device_name);

        match has_next_device {
            None => (),
            Some(next_device) => {
                if seen.contains(&next_device_name) {
                    panic!("Loop back to {next_device_name}");
                } else {
                    let mut new_seen = seen.clone();
                    new_seen.push(next_device_name.to_string());
                    let child_stats = solve2(devices, next_device, new_seen, known);
                    stats.both += child_stats.both;
                    stats.fft += child_stats.fft;
                    stats.dac += child_stats.dac;
                    // println!("stats.neither: {} + child_stats.neither {}", stats.neither, child_stats.neither);
                    stats.neither += child_stats.neither;
                }
            }
        }
    }
    if device.name == "fft" {
        stats.both += stats.dac;
        stats.dac = 0;
        stats.fft += stats.neither;
        stats.neither = 0;
    } else if device.name == "dac" {
        stats.both += stats.fft;
        stats.fft = 0;
        stats.dac += stats.neither;
        stats.neither = 0;
    }
    known.insert(device.name.clone(), stats.clone());
    return stats;
}

pub fn part1(lines: Vec<String>) {
    let devices = read_devices(lines);
    let num_paths = solve(&devices, &devices.get("you").unwrap());
    println!("Paths: {:?}", num_paths);
}

pub fn part2(lines: Vec<String>) {
    let devices = read_devices(lines);
    let mut known = DeviceResultMap::new();
    let start = "svr";
    let stats = solve2(
        &devices,
        &devices.get(start).unwrap(),
        vec![start.to_string()],
        &mut known,
    );
    println!("Stats: {:?}", stats);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve2() {
        let mut devices = DeviceMap::from([(
            "fft".to_string(),
            Device {
                name: "fft".to_string(),
                outputs: vec!["out".to_string()],
            },
        )]);
        let mut known = DeviceResultMap::new();
        let device = devices.get("fft").unwrap();
        let res = solve2(&devices, device, vec![], &mut known);
        assert_eq!(res.fft, 1);
    }
}
