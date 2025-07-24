pub fn run(input: &str) {
    let chars: Vec<char> = input.chars().collect();
    let total_sum = total_sum(&chars);
    println!("Answer 1: {}", total_sum);
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
