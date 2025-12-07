const DEBUG_OUTPUT: bool = true;

enum NumOrOp {
    Number(u64),
    Op(char),
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
