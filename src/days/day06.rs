const DEBUG_OUTPUT: bool = true;

enum NumOrOp {
    Number(u64),
    Op(char),
}

enum OpType {
    Add,
    Multiply,
}

fn load_sheet(lines: Vec<String>) -> Vec<Vec<NumOrOp>> {
    let mut sheet: Vec<Vec<NumOrOp>> = vec![];
    let mut row: usize = 0;
    for line in lines {
        let parts = line.split_ascii_whitespace();
        let mut col = 0;
        for part in parts {
            // Ignore whitespace at start and end of line
            if part == "" {
                continue;
            }

            if row == 0 {
                sheet.push(vec![]);
            }
            if part == "*" || part == "+" {
                sheet[col].push(NumOrOp::Op(part.chars().nth(0).unwrap()));
            } else {
                let num = part.parse().expect("Must be a number");
                sheet[col].push(NumOrOp::Number(num));
            }
            col += 1;
        }
        row += 1;
    }
    return sheet;
}

// Add up numbers in columns
fn col_sum(op: char, col: &[NumOrOp]) -> u64 {
    let mut sum = match col[0] {
        NumOrOp::Number(num) => num,
        _ => panic!("Must be a number"),
    };

    for i in 1..col.len() {
        let num = match col[i] {
            NumOrOp::Number(x) => x,
            _ => panic!("Must be a number"),
        };
        if op == '*' {
            sum *= num;
        } else {
            sum += num;
        }
    }

    if DEBUG_OUTPUT {
        println!("Col sum: {sum}");
    }

    return sum;
}

fn load_sheet2(lines: Vec<String>) -> Vec<(OpType, Vec<u64>)> {
    let mut columns = vec![];
    let last_line = &lines[lines.len() - 1];
    let mut col_starts = vec![];
    let mut idx = 0;
    for char in last_line.chars() {
        let op_type;
        match char {
            '*' => {
                op_type = OpType::Multiply;
                col_starts.push(idx);
                columns.push((op_type, vec![]));
            }
            '+' => {
                op_type = OpType::Add;
                col_starts.push(idx);
                columns.push((op_type, vec![]));
            }
            ' ' => {}
            _ => panic!("Invalid character in op row"),
        }
        idx += 1;
    }
    let num_cols = columns.len();
    let line_len = last_line.len();

    let mut col_strings = vec![];
    for _ in 0..=line_len - col_starts.len() {
        col_strings.push("".to_string());
    }

    if DEBUG_OUTPUT {
        println!("Last line: '{last_line}'");
        println!("Line len: {line_len}, num cols: {num_cols}");
    }

    for line_num in 0..lines.len() - 1 {
        let line = &lines[line_num];

        let mut col_num = col_strings.len() - 1;
        let mut line_pos = line_len;
        let mut skip_col = false;
        for char in line.chars().rev() {
            line_pos -= 1;
            if skip_col {
                skip_col = false;
                continue;
            }
            col_strings[col_num].push(char);
            if col_starts.contains(&line_pos) {
                skip_col = true;
            }
            if col_num == 0 {
                break;
            }
            col_num -= 1;
        }
    }

    if DEBUG_OUTPUT {
        println!("Col strings: {:?}", col_strings);
    }

    let mut offset = 0;
    for col_num in 0..num_cols {
        let col_start = col_starts[col_num];
        let col_end: usize = if col_num < num_cols - 1 {
            col_starts[col_num + 1] - 1
        } else {
            line_len
        };
        let col_width = col_end - col_start;
        for col_pos in 0..col_width {
            let pos = offset + col_pos;
            let val: u64 = col_strings[pos]
                .trim()
                .parse()
                .expect("Failed to parse number");
            columns[col_num].1.push(val);
        }
        offset += col_width;
    }

    if DEBUG_OUTPUT {
        println!("Columns:");
        for column in &columns {
            let op = match column.0 {
                OpType::Add => '+',
                OpType::Multiply => '*',
            };
            println!("    {}: {:?}", op, column.1);
        }
    }

    return columns;
}

// Add up numbers in cephalopod columns
fn col_sum2(op: OpType, col: Vec<u64>) -> u64 {
    let mut result = col[0];
    for i in 1..col.len() {
        match op {
            OpType::Add => result += col[i],
            OpType::Multiply => result *= col[i],
        }
    }
    return result;
}

pub fn part1(lines: Vec<String>) {
    let sheet = load_sheet(lines);
    let mut total_sum = 0;
    for col in sheet {
        let op = col.last().unwrap();
        match op {
            NumOrOp::Op(ch) => {
                total_sum += col_sum(*ch, &col[0..col.len() - 1]);
            }
            _ => panic!("Last row must be operation"),
        }
    }
    println!("Total sum: {total_sum}");
}

pub fn part2(lines: Vec<String>) {
    let sheet = load_sheet2(lines);
    let mut total_sum = 0;
    for (op, col) in sheet {
        total_sum += col_sum2(op, col);
    }
    println!("Total sum: {total_sum}");
}
