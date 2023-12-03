pub fn filter_games(input_text: String) -> u32 {
    let mut total = 0;

    let split = input_text.split("\r\n");
    let mut game_number = 0;
    for line in split {
        if line.is_empty() { continue; }

        game_number = game_number + 1;
        let header = format!("Game {game_number}:");
        assert!(line.starts_with(&header), "line = '{line}'");

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let line_input = &line[header.len()..];
        for elt in line_input.split(";") {
            for value in elt.split(",") {
                assert_eq!(value.chars().rev().position(char::is_alphabetic), Some(0), "line = '{line}', value = '{value}'");

                let mut char_iter = value.chars();
                let number_from = char_iter.position(char::is_numeric).unwrap();
                let number_until = number_from + 1 + char_iter.position(|c| !c.is_numeric()).unwrap();
                let amount = value[number_from..number_until].parse().unwrap();

                if value.ends_with("red") {
                    if amount > max_red { max_red = amount; }
                } else if value.ends_with("green") {
                    if amount > max_green { max_green = amount; }
                } else if value.ends_with("blue") {
                    if amount > max_blue { max_blue = amount; }
                } else {
                    panic!("not red, green or blue, but \"{value}\"")
                }
            }
        }

        println!("{max_red:3}, {max_green:3}, {max_blue:3} from input '{line_input:30}'");
        total += max_red * max_green * max_blue;
    }

    total
}