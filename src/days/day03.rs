pub fn run(input: &str) {
    let chars: Vec<char> = input.chars().collect();
    println!("Answer 1: {}", total_sum(&chars));
    println!("Answer 2: {}", trigger_total_sum(&chars));
}

fn total_sum(chars: &[char]) -> i32 {
    let mut total_sum = 0;
    let mut i = 0;

    while i < chars.len() {
        // Look for "mul("
        if i + 3 < chars.len()
            && chars[i] == 'm'
            && chars[i + 1] == 'u'
            && chars[i + 2] == 'l'
            && chars[i + 3] == '('
        {
            i += 4; // Move past "mul("

            // Parse first number (1-3 digits)
            let mut num1_str = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() && num1_str.len() < 3 {
                num1_str.push(chars[i]);
                i += 1;
            }

            // Check for comma
            if i < chars.len() && chars[i] == ',' && !num1_str.is_empty() {
                i += 1; // Move past comma

                // Parse second number (1-3 digits)
                let mut num2_str = String::new();
                while i < chars.len() && chars[i].is_ascii_digit() && num2_str.len() < 3 {
                    num2_str.push(chars[i]);
                    i += 1;
                }

                // Check for closing parenthesis
                if i < chars.len() && chars[i] == ')' && !num2_str.is_empty() {
                    // Valid mul instruction found
                    if let (Ok(num1), Ok(num2)) = (num1_str.parse::<i32>(), num2_str.parse::<i32>())
                    {
                        total_sum += num1 * num2;
                    }
                    i += 1; // Move past closing parenthesis
                } else {
                    // Invalid format, continue searching
                    continue;
                }
            } else {
                // Invalid format, continue searching
                continue;
            }
        } else {
            i += 1;
        }
    }
    total_sum
}

fn trigger_total_sum(chars: &[char]) -> i32 {
    let mut total_sum = 0;
    let mut i = 0;
    let mut mul_enabled = true; // mul instructions are enabled by default

    while i < chars.len() {
        if mul_enabled {
            // When mul is enabled, look for mul( or don't()
            if i + 3 < chars.len()
                && chars[i] == 'm'
                && chars[i + 1] == 'u'
                && chars[i + 2] == 'l'
                && chars[i + 3] == '('
            {
                let start_mul_instruction = i;
                i += 4; // Move past "mul("

                // Parse first number (1-3 digits)
                let mut num1_str = String::new();
                while i < chars.len() && chars[i].is_ascii_digit() && num1_str.len() < 3 {
                    num1_str.push(chars[i]);
                    i += 1;
                }

                // Check for comma
                if i < chars.len() && chars[i] == ',' && !num1_str.is_empty() {
                    i += 1; // Move past comma

                    // Parse second number (1-3 digits)
                    let mut num2_str = String::new();
                    while i < chars.len() && chars[i].is_ascii_digit() && num2_str.len() < 3 {
                        num2_str.push(chars[i]);
                        i += 1;
                    }

                    // Check for closing parenthesis
                    if i < chars.len() && chars[i] == ')' && !num2_str.is_empty() {
                        // Valid mul instruction found
                        if let (Ok(num1), Ok(num2)) =
                            (num1_str.parse::<i32>(), num2_str.parse::<i32>())
                        {
                            total_sum += num1 * num2;
                        }
                        i += 1; // Move past closing parenthesis
                    } else {
                        // Invalid format, reset i to after 'mul(' and continue searching
                        i = start_mul_instruction + 4;
                        continue;
                    }
                } else {
                    // Invalid format, reset i to after 'mul(' and continue searching
                    i = start_mul_instruction + 4;
                    continue;
                }
            }
            // Check for "don't()"
            else if i + 6 < chars.len()
                && chars[i] == 'd'
                && chars[i + 1] == 'o'
                && chars[i + 2] == 'n'
                && chars[i + 3] == '\''
                && chars[i + 4] == 't'
                && chars[i + 5] == '('
                && chars[i + 6] == ')'
            {
                mul_enabled = false;
                i += 7; // Move past "don't()"
            } else {
                i += 1;
            }
        } else {
            // When mul is disabled, only look for do()
            if i + 3 < chars.len()
                && chars[i] == 'd'
                && chars[i + 1] == 'o'
                && chars[i + 2] == '('
                && chars[i + 3] == ')'
            {
                mul_enabled = true;
                i += 4; // Move past "do()"
            } else {
                i += 1;
            }
        }
    }

    total_sum
}
