use std::{collections::HashMap, fmt};

type JoltageLevel = u16;
type JoltageLevels = Vec<JoltageLevel>;

struct Machine {
    reqd_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    reqd_joltage: JoltageLevels,
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

struct Incrementer2 {
    values: JoltageLevels,
    end_height: JoltageLevel,
    height: JoltageLevel,
    pos: usize,
    started: bool,
    complete: bool,
}

impl Incrementer2 {
    pub fn new(end_height: JoltageLevel, width: usize) -> Incrementer2 {
        if width < 1 {
            panic!("Garbage in, garbage out");
        }
        let mut values = vec![];
        for _ in 0..width {
            values.push(0);
        }
        return Incrementer2 {
            values,
            end_height,
            height: 1,
            pos: width - 1,
            started: false,
            complete: false,
        };
    }

    pub fn next(&mut self) -> Vec<JoltageLevel> {
        let width = self.values.len();

        // println!("Called next: height {}, pos {}, values {:?}", self.height, self.pos, self.values);

        if !self.started {
            self.started = true;
        } else if self.values[self.pos] < self.height {
            self.values[self.pos] += 1;
            if self.pos == 0 && self.values[0] == self.end_height {
                let mut complete = true;
                for val in &self.values {
                    if *val < self.end_height {
                        complete = false;
                        break;
                    }
                }
                if complete {
                    self.complete = true;
                }
            }
        } else {
            // N.B. self.values[self.pos] >= self.height (should be ==)
            let mut reached_zero = true;
            for next_pos in (0..self.pos).rev() {
                if self.values[next_pos] < self.height {
                    self.values[next_pos] += 1;
                    for remaining in next_pos + 1..=width - 1 {
                        self.values[remaining] = 0;
                    }
                    self.pos = width - 1;
                    reached_zero = false;
                    break;
                }
            }
            if reached_zero && self.values[0] == self.height {
                for i in 0..width - 1 {
                    self.values[i] = 0;
                }
                self.values[width - 1] += 1;
                self.height += 1;
                self.pos = width - 1;
                if self.height > self.end_height {
                    self.complete = true;
                }
            }
        }

        // println!("Gave {:?}, max: {}, complete? {}", self.values.clone(), self.end_height, self.complete);
        return self.values.clone();
    }
}

fn new_machine() -> Machine {
    return Machine {
        reqd_lights: vec![],
        buttons: vec![],
        reqd_joltage: vec![],
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

type MatrixElement = i16;
type Matrix = Vec<Vec<MatrixElement>>;

fn load_matrix(machine: &Machine) -> Matrix {
    // Final column of matrix = joltage
    // I.e. machine.reqd_joltage
    let mut matrix: Matrix = vec![];
    let mut row_num = 0;
    let num_buttons = machine.buttons.len();
    for joltage in &machine.reqd_joltage {
        matrix.push(vec![]);
        for _ in 0..num_buttons {
            matrix[row_num].push(0);
        }
        matrix[row_num].push(*joltage as MatrixElement);
        row_num += 1;
    }

    for col in 0..machine.buttons.len() {
        let buttons = &machine.buttons[col];
        for row in buttons {
            matrix[*row][col] = 1;
        }
    }

    return matrix;
}

fn render_matrix(matrix: &Matrix) -> String {
    let rows = matrix.len();
    let mut render = "".to_string();
    for row in 0..rows {
        for col in &matrix[row] {
            render.push_str(&format!("{:>2} ", col));
        }
        render.push('\n');
    }
    render.push('\n');
    return render;
}

fn print_matrix(matrix: &Matrix) {
    print!("{}", render_matrix(matrix));
}

fn swap_rows(row1: usize, row2: usize, matrix: &mut Matrix) {
    // println!("Swapping rows: {:?} and {:?}", matrix[row1], matrix[row2]);
    let new_row = matrix[row2].clone();
    matrix[row2] = matrix[row1].clone();
    matrix[row1] = new_row;
}

/**
 * Subtract row2 (* multiplier) from row1, put result in row1
 */
fn row_subtract(row1: usize, row2: usize, multiplier: MatrixElement, matrix: &mut Matrix) {
    // println!("Subtracting rows: {:?} - ({multiplier}x {:?})", matrix[row1], matrix[row2]);
    for col in 0..matrix[row2].len() {
        matrix[row1][col] = matrix[row1][col] - (matrix[row2][col] * multiplier);
    }
}

/**
 * Swaps two rows to ensure that the (n,n) position is a 1, if possible
 * Returns number of affected rows
 */
fn swap_for_one(row: usize, matrix: &mut Matrix) -> usize {
    let col = row;
    if matrix[row].len() <= col {
        return 0;
    }
    let current_value = matrix[row][col];
    if current_value == 1 {
        return 0;
    }
    for row2 in row + 1..matrix.len() {
        let new_value = matrix[row2][col];
        if new_value == 1 || (current_value == 0 && new_value != 0) {
            // println!("Swapping rows {row}: {:?} and {row2}: {:?}", matrix[row], matrix[row2]);
            swap_rows(row, row2, matrix);
            return 1;
        }
    }
    return 0;
}

fn multiply_row(row: usize, multiplier: MatrixElement, matrix: &mut Matrix) {
    for col in 0..matrix[row].len() {
        matrix[row][col] *= multiplier;
    }
}

fn clone_and_multiply_row(row: usize, multiplier: MatrixElement, matrix: &Matrix) -> Vec<MatrixElement> {
    let mut new_row = vec![];
    for col in 0..matrix[row].len() {
        new_row.push(matrix[row][col] * multiplier);
    }
    return new_row;
}

/**
 * Multiply by -1 to ensure a given value at is positive
 * The value is either the position (n,n), or the first non-zero value in a row
 * Return number of affected rows
 */
fn multiply_to_one(row: usize, col: usize, matrix: &mut Matrix) -> usize {
    if matrix[row].len() <= col {
        return 0;
    }
    if matrix[row][col] < 0 {
        multiply_row(row, -1, matrix);
        // println!("multiply_to_one (row {row})");
        // print_matrix(matrix);
        return 1;
    }
    return 0;
}

fn subtract_to_zero(row: usize, row2: usize, matrix: &mut Matrix) -> usize {
    let mut col = row;
    let width = matrix[0].len() - 1;

    // Move right if row,col is 0 (and not at solution column)
    while col < width && matrix[row][col] == 0 {
        col += 1;
    }
    if col >= width {
        return 0;
    }

    if matrix[row2][col] == 0 {
        return 0;
    }

    // First check if one is a multiple of the other
    let multiplier = matrix[row2][col] / matrix[row][col];

    // println!("row: {row}, row2: {row2}, col: {col}, multiplier {multiplier}");

    // Ensure multiplier is correct as int
    if matrix[row][col] * multiplier == matrix[row2][col] {
        // row2 = row2 - (row1 * X)
        row_subtract(row2, row, multiplier, matrix);
        return 1;
    }

    let row1_multiplier =  matrix[row2][col];
    let row2_multiplier =  matrix[row][col];

    let modified_row1 = clone_and_multiply_row(row, row1_multiplier, matrix);
    let modified_row2 = clone_and_multiply_row(row2, row2_multiplier, matrix);
    for col in 0..modified_row2.len() {
        matrix[row2][col] = modified_row2[col] - modified_row1[col];
    }

    return 1;
}

/**
 * Subtract rows to try to get to an identity(-like) matrix
 * The first non-zero column may end up being a number greater than 1
 * Return number of affected rows
 */
fn subtract_to_identity(matrix: &mut Matrix) -> usize {
    let mut modified_rows = 0;
    let num_cols = matrix[0].len() - 1;
    let num_rows = matrix.len();
    let max_dim = if num_cols < num_rows {
        num_cols
    } else {
        num_rows
    };

    for row in 0..max_dim {
        for col in 0..max_dim {
            if row == col {
                continue;
            }
            if matrix[row][col] != 0 && matrix[col][col] == 1 {
                let multiplier = matrix[row][col] / matrix[col][col];

                // Ensure multiplier is correct as int
                if matrix[col][col] * multiplier == matrix[row][col] {
                    row_subtract(row, col, multiplier, matrix);
                    modified_rows += 1;
                }
            }
        }
    }
    return modified_rows;
}

fn reduce_rows_by_common_denominator(matrix: &mut Matrix) -> usize {
    let mut modified_rows = 0;
    let width = matrix[0].len();
    for row in 0..matrix.len() {
        let mut lowest_val = MatrixElement::MAX;
        for col in 0..width {
            let val = matrix[row][col].abs();
            if val > 0 && val < lowest_val {
                lowest_val = val;
            }
        }
        // If row is all zeroes, it can't be reduced
        if lowest_val == MatrixElement::MAX {
            continue;
        }

        'lcd_loop: for lcd in (2..=lowest_val).rev() {
            for col in 0..width {
                if matrix[row][col] % lcd != 0 {
                    continue 'lcd_loop;
                }
            }
            print!("Reduced row {:?} -> ", matrix[row]);
            modified_rows += 1;
            for col in 0..width {
                let val = matrix[row][col];
                matrix[row][col] = val / lcd;
            }
            println!("{:?}", matrix[row]);
            break;
        }
    }
    return modified_rows;
}

// Do row ops on a matrix to try and get to an optimal state
fn do_row_ops(matrix: &mut Matrix) {
    let mut total_modified_rows = 1000;
    let mut runs = 0;

    while total_modified_rows > 0 {
        total_modified_rows = 0;

        // Repeat steps 1&2 for positions (0,0), (1,1), (2,2), ...
        // 1. Ensure (n,n) position has value 1 by multiplying or swapping rows
        for row in 0..matrix.len() {
            let modified_rows = multiply_to_one(row, row, matrix);
            total_modified_rows += modified_rows;

            let modified_rows = swap_for_one(row, matrix);
            total_modified_rows += modified_rows;
            if modified_rows > 0 {
                // println!("swap_for_one: {modified_rows}");
                // print_matrix(matrix);
            }

            let modified_rows = multiply_to_one(row, row, matrix);
            total_modified_rows += modified_rows;

            // 2. Do subtraction ops until n-th column is (0, 0, ..., 0,) 1, 0, 0, ..., 0
            for row2 in 0..matrix.len() {
                if row2 == row {
                    continue;
                }
                total_modified_rows += subtract_to_zero(row, row2, matrix);
            }
            // println!("subtract_to_zero: {total_modified_rows}");

            // 3. Somewhere along the line, do multiplication/division to convert values that aren't 0 or 1 to 1
            // if matrix[row][row] != 0 && matrix[row][row] != 1 {
            //     println!("Broke at this point:");
            //     print_matrix(matrix);
            //     panic!("Pos ({row},{row}) is neither 0 nor 1");
            // }
        }

        // println!("Checking for columns that don't match identity");
        // Ignore last column which is the solution column
        total_modified_rows += subtract_to_identity(matrix);
        // println!("subtract_to_identity: {total_modified_rows}");

        // Make all possible rows positive
        let solution_col = matrix[0].len() - 1;
        for row in 0..matrix.len() {
            for col in 0..solution_col {
                if matrix[row][col] != 0 {
                    total_modified_rows += multiply_to_one(row, col, matrix);
                    break;
                }
            }
        }

        total_modified_rows += reduce_rows_by_common_denominator(matrix);
        // println!("make rows positive: {total_modified_rows}");

        // Look for further reductions
        // total_modified_rows += subtract_via_first_nonzero(matrix);
        // println!("subtract_via_first_nonzero: {total_modified_rows}");

        runs += 1;
        println!("Run {runs}, affected rows: {total_modified_rows}");
    }
}

fn reduce_overdetermined_matrix(matrix: &mut Matrix) {
    // Exclude the result column when calculating the matrix size
    let matrix_width = matrix[0].len() - 1;
    let matrix_height = matrix.len();

    if matrix_height <= matrix_width {
        return;
    }
    println!("Matrix is too tall; mangling.");
    loop {
        let mut made_changes = false;
        for row in 0..matrix.len() {
            for row2 in 0..matrix.len() {
                if row == row2 {
                    continue;
                }
                if matrix[row] == matrix[row2] {
                    matrix.remove(row);
                    // println!("Removing duplicate row {row} (matches {row2})");
                    made_changes = true;
                    break;
                }
            }
            if made_changes {
                break;
            }
        }
        if matrix.len() + 1 == matrix[0].len() {
            break;
        }
        if !made_changes {
            break;
        }
    }
}

const INFINITE_PRESSES: JoltageLevel = JoltageLevel::MAX;

/**
 * If a column is all zeroes, that button is pressed zero times
 */
fn determine_zero_presses(matrix: &Matrix, button_presses: &mut Vec<Option<JoltageLevel>>) {
    let width = matrix[0].len() - 1;
    for col in 0..width {
        if button_presses[col] != None {
            continue;
        }
        let mut col_is_all_zeroes = true;
        for row in 0..matrix.len() {
            if matrix[row][col] != 0 {
                col_is_all_zeroes = false;
                break;
            }
        }
        if col_is_all_zeroes {
            button_presses[col] = Some(0);
        }
    }
}

fn determine_known_presses(
    matrix: &Matrix,
    known_presses: &Vec<Option<JoltageLevel>>,
) -> Vec<Option<JoltageLevel>> {
    // println!("determine_known_presses -- known_presses: {:?}", known_presses);
    let height = matrix.len();
    let width = matrix[0].len() - 1;
    let mut button_presses: Vec<Option<JoltageLevel>> = vec![];
    for i in 0..width {
        if i < known_presses.len() {
            button_presses.push(known_presses[i]);
        } else {
            button_presses.push(None);
        }
    }

    // println!("Button presses before zeroes: {:?}", button_presses);
    determine_zero_presses(matrix, &mut button_presses);
    // println!("Button presses after zeroes: {:?}", button_presses);

    for row in (0..height).rev() {
        let mut row_value = matrix[row][width];
        let mut unknown_els = vec![];
        for col in 0..width {
            if matrix[row][col] == 0 {
                continue;
            }
            match button_presses[col] {
                None => unknown_els.push(col),
                Some(value) => {
                    // println!("value as MatrixElement: {} * matrix[row][col]: {}", value, matrix[row][col]);
                    row_value -= (value as MatrixElement) * matrix[row][col];
                }
            }
        }
        if unknown_els.len() == 1 {
            let col = unknown_els.pop().unwrap();
            let overflow = row_value % matrix[row][col];
            if overflow != 0 {
                // println!(
                //     "Can't deal with non-int value at ({row},{col}): x * {} = {}; button presses: {:?}",
                //     matrix[row][col],
                //     row_value,
                //     fmt_optional_button_presses(&button_presses)
                // );
                button_presses[col] = Some(INFINITE_PRESSES);
                break;
            }
            let value = row_value / matrix[row][col];
            if value < 0 {
                // Can't press a button negative times
                // println!(
                //     "Can't press a button negative times at ({row},{col}): x * {} = {}; button presses: {:?}",
                //     matrix[row][col],
                //     row_value,
                //     fmt_optional_button_presses(&button_presses)
                // );
                button_presses[col] = Some(INFINITE_PRESSES);
                break;
            }

            let unsigned_val = value as u16;
            let resigned_val = unsigned_val as i16;
            if resigned_val != value {
                panic!("Overflow: {value} vs {resigned_val}");
            }
            button_presses[col] = Some(value as u16);
        }
    }

    return button_presses;
}

fn determine_all_known_presses(
    matrix: &Matrix,
    known_presses: &Vec<Option<JoltageLevel>>,
) -> Vec<Option<JoltageLevel>> {
    let mut known_presses = known_presses.clone();
    let mut num_known_presses = count_some(&known_presses);
    loop {
        let new_known = determine_known_presses(matrix, &known_presses);
        let num_new_known = count_some(&new_known);
        let mut has_infinite = false;
        for i in 0..new_known.len() {
            match new_known[i] {
                None => (),
                Some(val) => {
                    if val == INFINITE_PRESSES {
                        has_infinite = true;
                        break;
                    }
                }
            }
        }

        if num_new_known == num_known_presses || has_infinite {
            return new_known;
        }
        known_presses = new_known;
        num_known_presses = num_new_known;
    }
}

fn count_some<T>(options: &Vec<Option<T>>) -> usize {
    let mut sum_some = 0;
    for i in 0..options.len() {
        match options[i] {
            None => (),
            _ => {
                sum_some += 1;
            }
        }
    }
    return sum_some;
}

/**
 * Update matrix based on pressing a button a number of times
 */
fn press_matrix_button(matrix: &mut Matrix, button_idx: usize, num_presses: JoltageLevel) {
    let solution_col = matrix[0].len() - 1;
    for row in 0..matrix.len() {
        let val = matrix[row][button_idx];
        matrix[row][solution_col] -= val * (num_presses as MatrixElement);
        matrix[row][button_idx] = 0;
    }
}

fn determine_unknown_cols(known_presses: &Vec<Option<JoltageLevel>>) -> Vec<usize> {
    let mut unknown_columns = vec![];
    for i in 0..known_presses.len() {
        if known_presses[i] == None {
            unknown_columns.push(i);
        }
    }
    return unknown_columns;
}

/**
 * Determine key columns of matrix (the ones that matter for possible solutions)
 * Return vec of (col index, sum of absolute values in column)
 */
fn determine_key_cols(matrix: &Matrix, unknown_columns: &Vec<usize>) -> Vec<(usize, MatrixElement)> {
    let width = matrix[0].len() - 1;
    let mut key_cols = vec![];
    for col in 0..width {
        if !unknown_columns.contains(&col) {
            continue;
        }
        let (count_nonzero, sum_abs) = sum_col_abs(matrix, col);
        if count_nonzero > 1 {
            key_cols.push((col, sum_abs));
        }
    }
    return key_cols;
}

fn sum_col_abs(matrix: &Matrix, col: usize) -> (u8, MatrixElement) {
    let mut sum = 0;
    let mut count_non_zero = 0;
    for row in 0..matrix.len() {
        let val = matrix[row][col].abs();
        sum += val;
        if val > 0 {
            count_non_zero +=1;
        }
    }
    return (count_non_zero, sum);
}

fn solve_by_key_rows(matrix: &Matrix, known_presses: &Vec<Option<JoltageLevel>>) -> (JoltageLevel, Vec<JoltageLevel>) {
    let unknown_cols = determine_unknown_cols(known_presses);
    let key_cols = determine_key_cols(matrix, &unknown_cols);
    let mut max_presses = 1000;

    let mut product = max_presses as u64;
    for i in 0..key_cols.len() {
        product *= key_cols[i].1 as u64 + 1;
    }
    println!("Key columns: {:?}", key_cols);
    println!("Max presses: {max_presses}, number of possibilities: {product}");

    let mut inc = Incrementer2::new(max_presses, key_cols.len());
    let mut seen_options: HashMap<Vec<JoltageLevel>, ()> = HashMap::new();
    let mut best_solution = JoltageLevel::MAX;
    let mut best_solution_presses = vec![];
    'inc_loop: while !inc.complete {
        let key_press_options = inc.next();
        let press_option_sum: JoltageLevel = key_press_options.iter().sum();
        if press_option_sum > max_presses {
            // println!("{:?} (> max {max_presses} - ignore)", key_press_options);
            continue;
        }

        // TODO: fix the incrementer so this doesn't happen
        if seen_options.get(&key_press_options) != None {
            // println!("{:?} (repeat - ignore)", key_press_options);
            continue;
        }
        seen_options.insert(key_press_options.clone(), ());

        let mut press_options = known_presses.clone();
        for i in 0..key_cols.len() {
            let (col, _) = key_cols[i];
            press_options[col] = Some(key_press_options[i]);
        }
        press_options = determine_all_known_presses(matrix, &press_options);

        let mut has_none = false;
        let mut key_presses = vec![];
        for i in 0..press_options.len() {
            match press_options[i] {
                None => has_none = true,
                Some(num_presses) => {
                    if num_presses == INFINITE_PRESSES {
                        // Not solvable: move on to the next possibility
                        continue 'inc_loop;
                    }
                    key_presses.push(num_presses);
                }
            }
        }
        if has_none {
            // If presses couldn't all be determined, it should mean there was an unsolvable row,
            // with INFINITE_PRESSES. If not, something has gone wrong, and the key_presses won't
            // align with the columns
            panic!("Key press option with no value");
        }

        let mut sum_presses = 0;
        for i in 0..key_presses.len() {
            sum_presses += key_presses[i];
        }

        println!();
        println!("Key column presses: {:?}", key_press_options);
        println!("Possible solution: {:?} (num presses: {sum_presses}, max presses: {max_presses})", key_presses);

        if sum_presses < best_solution {
            best_solution = sum_presses;
            best_solution_presses = key_presses;
            if inc.height > sum_presses {
                inc.height = sum_presses;
            }
            if inc.end_height > sum_presses {
                inc.end_height = sum_presses;
            }
            if max_presses > sum_presses {
                max_presses = sum_presses;
            }
        }
    }

    return (best_solution, best_solution_presses);
}

fn convert_options_to_presses(options: Vec<Option<JoltageLevel>>) -> Vec<JoltageLevel> {
    let mut presses = vec![];
    for i in 0..options.len() {
        let val = options[i].unwrap_or(INFINITE_PRESSES);
        presses.push(val);
    }
    return presses;
}

fn solve_matrix(matrix: &Matrix, depth: usize) -> (JoltageLevel, Vec<JoltageLevel>) {
    // Check for any immediately solvable values
    let known_presses = determine_all_known_presses(matrix, &vec![]);
    let num_known_presses = count_some(&known_presses);

    // println!("Known presses: {:?}", known_presses);
    // print_matrix(matrix);

    // SOLVED
    if num_known_presses == known_presses.len() {
        let mut num_presses_required = 0;

        // Check for infinite presses in any slot first
        // In case another slot has that minus one, which would cause an overflow
        for i in 0..known_presses.len() {
            match known_presses[i] {
                None => (),
                Some(value) => {
                    if value == INFINITE_PRESSES {
                        return (INFINITE_PRESSES, convert_options_to_presses(known_presses));
                    }
                }
            }
        }

        for i in 0..known_presses.len() {
            match known_presses[i] {
                None => (),
                Some(value) => {
                    // println!("Adding {value} to num_presses_required: {num_presses_required}");
                    num_presses_required += value;
                }
            }
        }
        return (num_presses_required, convert_options_to_presses(known_presses));
    }

    if depth == 0 {
        println!("*** CAN'T SOLVE WITHOUT BRUTE FORCE: {:?} ***", known_presses);
    }

    // Apply known presses so brute force becomes easier
    let mut matrix = matrix.clone();
    for button_idx in 0..known_presses.len() {
        if known_presses[button_idx] != None {
            let num_presses = known_presses[button_idx].unwrap();
            if num_presses == INFINITE_PRESSES {
                return (num_presses, vec![]);
            }
            press_matrix_button(&mut matrix, button_idx, num_presses);
        }
    }

    let (num_presses, mut final_presses) = solve_by_key_rows(&matrix, &known_presses);
    if num_presses == INFINITE_PRESSES {
        return (num_presses, final_presses);
    }

    for i in 0..final_presses.len() {
        if final_presses[i] == 0 {
            final_presses[i] = known_presses[i].unwrap_or(0);
        }
    }

    println!("Returning num_presses {num_presses}: {:?}", final_presses);
    return (num_presses, final_presses);
}

fn solve_machine(machine: &Machine) -> (JoltageLevel, Vec<JoltageLevel>) {
    let mut matrix = load_matrix(&machine);
    println!("Original matrix:");
    print_matrix(&matrix);
    // println!();

    reduce_overdetermined_matrix(&mut matrix);
    println!("After reducing overdetermined matrix:");
    print_matrix(&matrix);

    do_row_ops(&mut matrix);
    println!("After row ops:");
    print_matrix(&matrix);

    return solve_matrix(&matrix, 0);
}

pub fn part2(lines: Vec<String>) {
    let machines = read_manual(lines);
    let mut total_presses = 0;
    let mut machine_num = 1;
    for machine in machines {
        println!("Machine {machine_num}:");
        let (lowest_sum, chosen_presses) = solve_machine(&machine);
        println!("Adding {lowest_sum} presses...");

        println!("Best solution for machine {machine_num}: {lowest_sum} from presses {:?}", chosen_presses);
        if lowest_sum > 60_000 {
            panic!("Invalid value; check machine {machine_num}");
        }
        total_presses += lowest_sum;
        println!("    (total: {total_presses})\n");
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
        };
        assert_eq!(presses_to_on(&machine), 1);

        let machine = Machine {
            reqd_lights: vec![true, true],
            buttons: vec![vec![0], vec![1]],
            reqd_joltage: vec![],
        };
        assert_eq!(presses_to_on(&machine), 2);

        let machine = Machine {
            reqd_lights: vec![true, true, false],
            buttons: vec![vec![0], vec![1, 2], vec![2]],
            reqd_joltage: vec![],
        };
        assert_eq!(presses_to_on(&machine), 3);
    }

    #[test]
    fn test_solve_machine() {
        let machine = Machine {
            reqd_lights: vec![true, true],
            buttons: vec![vec![1]],
            reqd_joltage: vec![0, 1],
        };
        assert_eq!(solve_machine(&machine).0, 1);

        let machine = Machine {
            reqd_lights: vec![true, true],
            buttons: vec![vec![1], vec![1, 2], vec![2]],
            reqd_joltage: vec![0, 2, 3],
        };
        assert_eq!(solve_machine(&machine).0, 3);

        let machine = Machine {
            reqd_lights: vec![true, true],
            buttons: vec![vec![0, 1], vec![1, 2], vec![0, 2]],
            // presses: 2, 3, 7 = 12
            reqd_joltage: vec![5, 9, 10],
        };
        assert_eq!(solve_machine(&machine).0, 12);
    }

    #[test]
    fn test_subtract_to_identity() {
        let mut matrix: Matrix = vec![
            vec![1, 1, 10],
            vec![0, 1, 3],
        ];
        let rows_modified = subtract_to_identity(&mut matrix);
        assert_eq!(rows_modified, 1);
        assert_eq!(matrix, vec![
            vec![1, 0, 7],
            vec![0, 1, 3],
        ]);

        let mut matrix: Matrix = vec![
            vec![1, 2, 10],
            vec![0, 1, 3],
        ];
        let rows_modified = subtract_to_identity(&mut matrix);
        assert_eq!(rows_modified, 1);
        assert_eq!(matrix, vec![
            vec![1, 0, 4],
            vec![0, 1, 3],
        ]);
    }

    #[test]
    fn test_subtract_to_zero() {
        let mut matrix: Matrix = vec![
            vec![1, 0, 2, 1, 12],
            vec![1, 0, 1, 2,  7],
        ];

        for row in 0..matrix.len() {
            for row2 in 0..matrix.len() {
                if row == row2 {
                    continue;
                }
                subtract_to_zero(row, row2, &mut matrix);
            }
        }

        assert_eq!(matrix, vec![
            // After first round
            // vec![1, 0, 2,  1, 12],
            // vec![0, 0, -1, 1, -5],

            // After 2nd round
            vec![1, 0,  0, 3,  2],
            vec![0, 0, -1, 1, -5],
        ]);
    }

    #[test]
    fn test_subtract_to_zero_with_both_two_plus() {
        let mut matrix: Matrix = vec![
            vec![1, 3, 1, 10],
            vec![0, 2, 1,  5],
        ];

        for row in 0..matrix.len() {
            for row2 in 0..matrix.len() {
                if row == row2 {
                    continue;
                }
                subtract_to_zero(row, row2, &mut matrix);
            }
        }

        assert_eq!(matrix, vec![
            vec![2, 0, -1, 5], // 2r1 - 3r2 -> r1
            vec![0, 2,  1, 5],
        ]);
    }

    #[test]
    fn test_press_matrix_button() {
        let mut matrix: Matrix = vec![
            vec![1, 3, 1, 10],
            vec![0, 2, 1,  5],
        ];
        press_matrix_button(&mut matrix, 1, 2);
        assert_eq!(matrix, vec![
            vec![1, 0, 1, 4],
            vec![0, 0, 1, 1],
        ]);

        let mut matrix: Matrix = vec![
            vec![1, -3, 1, 10],
            vec![0, -2, 1,  5],
        ];
        press_matrix_button(&mut matrix, 1, 2);
        assert_eq!(matrix, vec![
            vec![1, 0, 1, 16],
            vec![0, 0, 1,  9],
        ]);
    }
}
