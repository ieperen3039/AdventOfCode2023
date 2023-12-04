use std::{
    cmp,
    collections::{hash_map, HashMap},
};

pub fn part_1_find_engine_parts(input: String) -> u32 {
    let rows: Vec<&str> = input.lines().filter(|&l| !l.is_empty()).collect();
    let rows_len = rows.len();
    let rows_end = rows_len - 1;

    let line_len = rows[0].len();
    let line_end = line_len - 1;

    assert!(rows.iter().all(|s| s.len() == line_len));

    let mut total = 0;

    for line_idx in 0..rows_len {
        let line = rows[line_idx];
        let char_list: Vec<_> = line.chars().collect();

        let mut next_char_idx = 0;
        while next_char_idx < line_len {
            // splitting this in two variables, because this is less bug-prone
            let char_idx = next_char_idx;
            next_char_idx += 1;

            let char = char_list[char_idx];
            if !char.is_ascii_digit() {
                continue;
            }

            let numeric_len = numeric_len(&char_list[char_idx..]);
            // number_end is the index of the last char
            let number_end = char_idx + numeric_len - 1;
            let number_str = &line[char_idx..=number_end];
            let number: u32 = number_str.parse().unwrap();

            // c_min = first index to searc
            let c_min = char_idx.checked_sub(1).unwrap_or(0);
            // c_max = last index to search
            let c_max = cmp::min(number_end + 1, line_end);

            if is_machine_part(char_list[c_min])
                || is_machine_part(char_list[c_max])
                || (line_idx > 0 && contains_machine_part(&rows[line_idx - 1][c_min..=c_max]))
                || (line_idx < rows_end
                    && contains_machine_part(&rows[line_idx + 1][c_min..=c_max]))
            {
                print!("added {number:<7}; ");
                total += number;
            } else {
                print!("ignored {number:<5}; ");
            }

            // could be number_end + 2, as we know the next char is not a number
            next_char_idx = number_end + 1;
        }

        println!("")
    }

    total
}

pub fn part_2_find_gear_ratios(input: String) -> u32 {
    let rows: Vec<&str> = input.lines().filter(|&l| !l.is_empty()).collect();
    let rows_len = rows.len();
    let rows_end = rows_len - 1;

    let character_matrix: Vec<Vec<char>> = rows.iter().map(|l| l.chars().collect()).collect();
    let line_len = rows[0].len();
    let line_end = line_len - 1;

    assert!(rows.iter().all(|s| s.len() == line_len));

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for line_idx in 0..rows_len {
        let char_list = &character_matrix[line_idx];

        let mut next_char_idx = 0;
        while next_char_idx < line_len {
            // splitting this in two variables, because this is less bug-prone
            let char_idx = next_char_idx;
            next_char_idx += 1;

            let char = char_list[char_idx];
            if !char.is_ascii_digit() {
                continue;
            }

            let numeric_len = numeric_len(&char_list[char_idx..]);
            // number_end is the index of the last char
            let number_end = char_idx + numeric_len - 1;
            let number_str = &rows[line_idx][char_idx..=number_end];
            let number: u32 = number_str.parse().unwrap();

            // c_min = first index to searc
            let c_min = char_idx.checked_sub(1).unwrap_or(0);
            // c_max = last index to search
            let c_max = cmp::min(number_end + 1, line_end);
            // l_min = first index to searc
            let l_min = line_idx.checked_sub(1).unwrap_or(0);
            // l_max = last index to search
            let l_max = cmp::min(line_idx + 1, rows_end);

            for line_idx in l_min..=l_max {
                for char_idx in c_min..=c_max {
                    if is_gear(character_matrix[line_idx][char_idx]) {
                        println!("{number:>5} for position ({line_idx:3},{char_idx:3}) |");
                        gears
                            .entry((line_idx, char_idx))
                            .or_insert(Vec::new())
                            .push(number);
                    }
                }
            }

            // could be number_end + 2, as we know the next char is not a number
            next_char_idx = number_end + 1;
        }
    }

    gears
        .values()
        .filter(|vec| vec.len() == 2)
        .map(|v| v[0] * v[1])
        .fold(0, |a, b| a + b)
}

// we don't accept the char_itr as &mut, because it would be hard to understand what state the iterator ends up in.
// in our case, it is right where we want, but I have standards
fn numeric_len(chars: &[char]) -> usize {
    let mut index = 0;
    for c in chars {
        if !c.is_ascii_digit() {
            return index;
        }
        index += 1;
    }

    return index;
}

fn contains_machine_part(line: &str) -> bool {
    line.chars().any(is_machine_part)
}

fn is_machine_part(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

// true iff c is a * symbol (may not be a true gear)
fn is_gear(c: char) -> bool {
    c == '*'
}
