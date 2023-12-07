pub fn part_1_get_total_score(input: String) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let input = line.split(':').nth(1).unwrap();
        let mut part_itr = input.split('|');
        let winning_number_string = part_itr.next().unwrap().trim();
        let our_number_string = part_itr.next().unwrap().trim();

        let winning_numbers: Vec<u32> = winning_number_string
            .split(' ')
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let our_numbers: Vec<u32> = our_number_string
            .split(' ')
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let mut score = 1;

        for num in our_numbers {
            if winning_numbers.contains(&num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        total += score;
    }

    total
}

pub fn part_2_get_total_scratchcards(input: String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    // initially one of every card
    let mut quantity_of_cards: Vec<u32> = vec![1; lines.len()];

    for card_idx in 0..lines.len() {
        let input = lines[card_idx].split(':').nth(1).unwrap();
        let mut part_itr = input.split('|');
        let winning_number_string = part_itr.next().unwrap().trim();
        let our_number_string = part_itr.next().unwrap().trim();

        let winning_numbers: Vec<u32> = winning_number_string
            .split(' ')
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let our_numbers: Vec<u32> = our_number_string
            .split(' ')
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let mut num_winning_numbers = 0;

        for num in our_numbers {
            if winning_numbers.contains(&num) {
                num_winning_numbers += 1;
            }
        }

        println!(
            "Card {}: num copies = {}, num_winning_numbers = {num_winning_numbers}",
            card_idx + 1,
            quantity_of_cards[card_idx]
        );

        for offset in 0..num_winning_numbers {
            let num_copies_of_this = quantity_of_cards[card_idx];
            quantity_of_cards[card_idx + 1 + offset] += num_copies_of_this;
        }
    }

    // println!("{:?}", quantity_of_cards);
    quantity_of_cards.into_iter().fold(0, |a, b| a + b)
}
