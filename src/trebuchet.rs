const NUMBERS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

pub fn decode_calibration(input_text: String) -> u32 {
    let split = input_text.lines();
    let mut total = 0;

    for line in split {
        if line.is_empty() { continue; }

        let mut first = 0;
        for char_idx in 0..line.len() {
            let line_progress = &line[char_idx..];

            if let Some(num) = check_for_number(line_progress) {
                first = num;
                break;
            }
        }

        let mut second = 0;
        for char_idx in (0..line.len()).rev() {
            let line_progress = &line[char_idx..];

            if let Some(num) = check_for_number(line_progress) {
                second = num;
                break;
            }
        }
        println!("{line}: {first}, {second}");

        total += (first * 10) + second;
    }

    total
}

fn check_for_number(line_progress: &str) -> Option<u32> {
    let first_char = line_progress.chars().nth(0).unwrap();
    if first_char.is_numeric() {
        return Some(first_char.to_digit(10).unwrap());
    } else {
        // start at one, we skip "zero"
        for num_idx in 1..=9 {
            if line_progress.starts_with(NUMBERS[num_idx]) {
                return Some(num_idx as u32);
            }
        }
    }

    return None;
}
