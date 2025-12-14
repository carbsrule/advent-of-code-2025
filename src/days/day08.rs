use std::fmt;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

struct PointDistance {
    p1: Point,
    p2: Point,
    dist: f64,
}

impl fmt::Debug for PointDistance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}]-[{}, {}, {}]: {}",
            self.p1.x, self.p1.y, self.p1.z, self.p2.x, self.p2.y, self.p2.z, self.dist,
        )
    }
}

fn load_points(lines: Vec<String>) -> Vec<Point> {
    let mut points = vec![];
    for line in lines {
        if line == "" {
            continue;
        }
        let parts = line.split(",");
        let mut part_num = 0;
        let mut point = Point { x: 0, y: 0, z: 0 };
        for part in parts {
            match part_num {
                0 => {
                    point.x = part.parse().expect("Must be a number");
                }
                1 => {
                    point.y = part.parse().expect("Must be a number");
                }
                2 => {
                    point.z = part.parse().expect("Must be a number");
                }
                _ => panic!("Failure"),
            }
            part_num += 1;
        }
        points.push(point);
    }
    return points;
}

fn calc_distance(p1: Point, p2: Point) -> PointDistance {
    let x_dist: f64 = if p2.x > p1.x {
        (p2.x - p1.x).into()
    } else {
        (p1.x - p2.x).into()
    };
    let y_dist: f64 = if p2.y > p1.y {
        (p2.y - p1.y).into()
    } else {
        (p1.y - p2.y).into()
    };
    let z_dist: f64 = if p2.z > p1.z {
        (p2.z - p1.z).into()
    } else {
        (p1.z - p2.z).into()
    };
    let dist = f64::sqrt((x_dist * x_dist) + (y_dist * y_dist) + (z_dist * z_dist));
    return PointDistance { p1, p2, dist };
}

fn shortest_distances(points: Vec<Point>) -> Vec<PointDistance> {
    let mut distances = vec![];
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = calc_distance(points[i], points[j]);
            distances.push(dist);
        }
    }

    // Reorder distances from min - max
    distances.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

    // println!("Distances:");
    // for dist in &distances {
    //     println!("{:?}", dist);
    // }
    return distances;
}

fn reassign(dist: PointDistance, circuits: &mut Vec<Vec<Point>>) -> u8 {
    const UNASSIGNED: usize = 1_000_000;
    let mut circuit1 = UNASSIGNED;
    let mut circuit2 = UNASSIGNED;
    for circuit_num in 0..circuits.len() {
        let existing_circuit = &circuits[circuit_num];
        if existing_circuit.contains(&dist.p1) {
            circuit1 = circuit_num;
        }
        if existing_circuit.contains(&dist.p2) {
            circuit2 = circuit_num;
        }
    }
    if circuit1 == UNASSIGNED && circuit2 == UNASSIGNED {
        // println!("Assigned {:?} and {:?} to new circuit ({})", dist.p1, dist.p2, circuits.len());
        let mut new_circuit = vec![];
        new_circuit.push(dist.p1);
        new_circuit.push(dist.p2);
        circuits.push(new_circuit);
        return 2;
    } else if circuit1 == circuit2 {
        // println!("Junction boxes {:?} and {:?} are already assigned to circuit ({circuit1}): {:?}", dist.p1, dist.p2, circuits[circuit1]);
        // OK, nothing to do here
    } else {
        if circuit1 == UNASSIGNED {
            // println!("Assigned {:?} to existing circuit ({}): {:?}", dist.p1, circuit2, circuits[circuit2]);
            circuits[circuit2].push(dist.p1);
            return 1;
        } else if circuit2 == UNASSIGNED {
            // println!("Assigned {:?} to existing circuit ({}): {:?}", dist.p2, circuit1, circuits[circuit1]);
            circuits[circuit1].push(dist.p2);
            return 1;
        } else {
            // println!("Joining boxes assigned to different existing circuits {:?} in {}, {:?} in {}", dist.p1, circuit1, dist.p2, circuit2);
            for junction_box_idx in 0..circuits[circuit2].len() {
                let junction_box = circuits[circuit2][junction_box_idx];
                circuits[circuit1].push(junction_box);
            }
            circuits.remove(circuit2);
            return 0;
        }
    }
    return 0;
}

pub fn part1(lines: Vec<String>) {
    let junction_boxes = load_points(lines);
    // println!("Points loaded: {:?}", junction_boxes);

    let mut circuits: Vec<Vec<Point>> = vec![];
    let distances = shortest_distances(junction_boxes);
    let mut num_assigned = 0;
    for dist in distances {
        reassign(dist, &mut circuits);
        // Only connect first 1000 pairs
        num_assigned += 1;
        if num_assigned >= 1000 {
            break;
        }
    }

    let mut circuit_lengths = vec![];
    for circuit in circuits {
        circuit_lengths.push(circuit.len());
    }
    circuit_lengths.sort();

    let mut longest_three = vec![];
    for i in (0..circuit_lengths.len()).rev() {
        longest_three.push(circuit_lengths[i]);
        if longest_three.len() == 3 {
            break;
        }
    }

    println!("Circuit lengths: {:?}", longest_three);

    let mut product = 1;
    for len in longest_three {
        product *= len;
    }

    println!("Product: {product}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(
            calc_distance(Point { x: 0, y: 0, z: 0 }, Point { x: 0, y: 0, z: 0 }).dist,
            0.0
        );
        assert_eq!(
            calc_distance(Point { x: 0, y: 0, z: 0 }, Point { x: 1, y: 0, z: 0 }).dist,
            1.0
        );
        assert_eq!(
            calc_distance(Point { x: 1, y: 1, z: 1 }, Point { x: 2, y: 3, z: 4 }).dist,
            f64::sqrt(1.0 + 4.0 + 9.0)
        );
        assert_eq!(
            calc_distance(Point { x: 2, y: 3, z: 4 }, Point { x: 1, y: 1, z: 1 }).dist,
            f64::sqrt(1.0 + 4.0 + 9.0)
        );
    }
}
