pub fn run(input: &str) {
    let safe_reports = input
        .lines()
        .filter(|line| {
            let numbers = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok());
            safe(numbers)
        })
        .count();

    println!("Answer 2: {}", safe_reports);
}

fn safe(mut report: impl Iterator<Item = i32>) -> bool {
    let mut prev = match report.next() {
        Some(v) => v,
        None => return true,
    };

    let mut sign: Option<i32> = None;

    for current in report {
        let diff = current - prev;

        if sign.is_none() {
            if diff == 0 {
                return false;
            }
            sign = Some(if diff > 0 { 1 } else { -1 });
        }

        let sign = sign.unwrap();
        let normalized_diff = diff * sign;
        if !(1..=3).contains(&normalized_diff) {
            return false;
        }
        prev = current;
    }

    true
}
