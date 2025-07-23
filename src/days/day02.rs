pub fn run(input: &str) {
    let safe1_reports = input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            safe(&numbers)
        })
        .count();

    println!("Answer 1: {}", safe1_reports);

    let safe2_reports = input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            safe_allow_one_error(&numbers)
        })
        .count();

    println!("Answer 2: {}", safe2_reports);
}

fn safe_with_violation_idx(report: &[i32], jump: Option<usize>) -> Option<usize> {
    let mut prev_idx = 0;
    if let Some(j) = jump {
        if j >= report.len() {
            panic!(
                "jump index {} is out of bounds for report of length {}",
                j,
                report.len()
            );
        }
        if j == 0 {
            prev_idx = 1;
            if prev_idx >= report.len() {
                return None;
            }
        }
    }

    let mut prev = report[prev_idx];
    let mut expected_sign: Option<i32> = None;

    for i in (prev_idx + 1)..report.len() {
        if Some(i) == jump {
            continue;
        }

        let current = report[i];
        let diff = current - prev;

        if !(1..=3).contains(&diff.abs()) {
            return Some(i);
        }

        let current_sign = if diff > 0 { 1 } else { -1 };

        match expected_sign {
            Some(s) => {
                if s != current_sign {
                    return Some(i);
                }
            }
            None => {
                expected_sign = Some(current_sign);
            }
        }

        prev = current;
    }

    None
}

fn safe(numbers: &[i32]) -> bool {
    safe_with_violation_idx(numbers, None).is_none()
}

fn safe_allow_one_error(numbers: &[i32]) -> bool {
    if numbers.len() <= 2 {
        return true;
    }

    let first_violation_idx = match safe_with_violation_idx(&numbers, None) {
        Some(idx) => idx,
        None => return true,
    };

    if safe_with_violation_idx(numbers, Some(first_violation_idx)).is_none() {
        return true;
    }

    if first_violation_idx > 0 {
        if safe_with_violation_idx(numbers, Some(first_violation_idx - 1)).is_none() {
            return true;
        }
    }

    if first_violation_idx > 1 {
        if safe_with_violation_idx(numbers, Some(first_violation_idx - 2)).is_none() {
            return true;
        }
    }

    false
}
