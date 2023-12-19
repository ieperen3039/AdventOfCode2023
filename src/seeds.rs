use std::cmp;



#[derive(Debug)]
struct Conversion {
    destination_start: u32,
    source_start: u32,
    range_len: u32,
}

impl Conversion {
    fn map(&self, value: u32) -> Option<u32> {
        if value < self.source_start {
            return None;
        }
        if value >= self.source_start + self.range_len {
            return None;
        }
        return Some(value - self.source_start + self.destination_start);
    }
}

pub fn part_1_calculate_seed_locations(input: String) -> u32 {
    let (seeds, conversions_list) = read_input(input);

    let mut sorted_conversions_list: Vec<Vec<Conversion>> = Vec::new();
    for mut conversion_map in conversions_list {
        conversion_map.sort_unstable_by_key(|c| c.source_start);
        sorted_conversions_list.push(conversion_map)
    }

    let seed_locations: Vec<u32> = seeds
        .iter()
        .map(|seed| apply_conversions(*seed, &sorted_conversions_list))
        .collect();

    seed_locations.into_iter().min().unwrap_or(0)
}

fn apply_conversions(seed: u32, sorted_conversions_list: &Vec<Vec<Conversion>>) -> u32 {
    println!("Processing seed {seed}:");
    let mut current_value = seed;
    for conversions in sorted_conversions_list {
        let search_result = conversions.binary_search_by_key(&current_value, |c| c.source_start);
        // we want the index that starts before or at the given value
        let conversion_index = match search_result {
            Ok(i) => i,
            Err(0) => 0,
            Err(i) => i - 1,
        };

        let maybe_new_value = conversions[conversion_index].map(current_value);

        println!("Value {current_value} maps to {:?}", maybe_new_value);

        current_value = match maybe_new_value {
            Some(new_value) => new_value,
            None => current_value,
        };
    }

    println!("Seed {seed} maps to location {current_value}");
    current_value
}

fn read_input(input: String) -> (Vec<u32>, Vec<Vec<Conversion>>) {
    let mut lines = input.lines();
    let seeds_str = lines.next().unwrap();
    let seeds_str = seeds_str.split(':').nth(1).unwrap();
    let seeds: Vec<u32> = seeds_str
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    // skip 2 elements (to the start of seed-to-soil map)
    lines.nth(1);

    let mut conversions: Vec<Vec<Conversion>> = Vec::new();
    conversions.push(Vec::new());

    let mut super_index = 0;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            // end of this map, start with next map
            let header = lines.next();
            if header.is_none() {
                // happens if input has trailing whitespace
                break;
            }
            println!("Start {}", header.unwrap());

            super_index += 1;
            conversions.push(Vec::new());
            continue;
        }

        let elements: Vec<u32> = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        assert_eq!(elements.len(), 3);

        let destination_start = elements[0];
        let source_start = elements[1];
        let range_len = elements[2];
        let max_range_len = u32::MAX - source_start;
        
        conversions[super_index].push(Conversion {
            destination_start,
            source_start,
            range_len : cmp::min(range_len, max_range_len),
        })
    }

    println!("Done reading input");
    (seeds, conversions)
}
