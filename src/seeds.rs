use std::{cmp, ops::RangeInclusive};

#[derive(Debug)]
struct Conversion {
    destination_first: u32,
    source_first: u32,
    range_len: u32,
}

impl Conversion {
    fn try_map(&self, value: u32) -> Option<u32> {
        if self.source_range().contains(&value) {
            Some(value - self.source_first + self.destination_first)
        } else {
            None
        }
    }

    fn map(&self, value: u32) -> u32 {
        assert!(self.source_range().contains(&value), "range = {:?}, value = {}", self.source_range(), value);
        value - self.source_first + self.destination_first
    }

    fn source_range(&self) -> RangeInclusive<u32> {
        self.source_first..=self.source_last()
    }

    fn source_last(&self) -> u32 {
        u32::saturating_add(self.source_first, self.range_len - 1)
    }

    fn destination_range(&self) -> RangeInclusive<u32> {
        self.destination_first..=self.destination_last()
    }

    fn destination_last(&self) -> u32 {
        u32::saturating_add(self.destination_first, self.range_len - 1)
    }
}

pub fn part_1_calculate_seed_locations(input: String) -> u32 {
    let (seeds, conversions_list) = read_input(input);

    let mut sorted_conversions_list: Vec<Vec<Conversion>> = Vec::new();
    for mut conversion_map in conversions_list {
        conversion_map.sort_unstable_by_key(|c| c.source_first);
        sorted_conversions_list.push(conversion_map)
    }

    let seed_locations: Vec<u32> = seeds
        .iter()
        .map(|seed| apply_conversions_on_seed(*seed, &sorted_conversions_list))
        .collect();

    seed_locations.into_iter().min().unwrap_or(0)
}

fn apply_conversions_on_seed(seed: u32, sorted_conversions_list: &Vec<Vec<Conversion>>) -> u32 {
    println!("Processing seed {seed}:");

    let mut current_value = seed;
    for conversions in sorted_conversions_list {
        let conversion_index = find_conversion_index(conversions, current_value);

        let maybe_new_value = conversions[conversion_index].try_map(current_value);
        current_value = match maybe_new_value {
            Some(new_value) => new_value,
            None => current_value,
        }
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
            destination_first: destination_start,
            source_first: source_start,
            range_len: cmp::min(range_len, max_range_len),
        })
    }

    println!("Done reading input");
    (seeds, conversions)
}

pub fn part_2_calculate_range_mapping(input: String) -> u32 {
    let (seeds, conversions_list) = read_input(input);

    let mut sorted_conversions_list: Vec<Vec<Conversion>> = Vec::new();
    for mut conversion_map in conversions_list {
        conversion_map.sort_unstable_by_key(|c| c.source_first);
        sorted_conversions_list.push(add_missing_ranges(conversion_map))
    }

    let mut ranges = Vec::new();
    for chunk in seeds.chunks(2) {
        assert_eq!(chunk.len(), 2);
        let first = chunk[0];
        let range = chunk[1];
        let last = first + range - 1;
        ranges.push(first..=last);
    }
    println!("Initial ranges: {:?} ({})", ranges, ranges.len());

    for sorted_conversions in sorted_conversions_list {
        ranges = apply_conversions_on_ranges(&sorted_conversions, ranges);
        // println!("Ranges: {:?}", ranges);
        println!("Number of ranges: {}", ranges.len());

        ranges = combine_adjacent(ranges);
        // println!("Ranges after combining: {:?}", ranges);
        println!("Number of ranges after combining: {}", ranges.len());
    }

    println!("Final number of ranges: {}", ranges.len());

    ranges
        .into_iter()
        .map(|r| r.start().to_owned())
        .min()
        .unwrap()
}

fn add_missing_ranges(sorted_conversions: Vec<Conversion>) -> Vec<Conversion> {
    let mut new_conversions: Vec<Conversion> = Vec::new();

    // add zero to first
    if let Some(first_conversion) = sorted_conversions.first() {
        if first_conversion.source_first > 0 {
            new_conversions.push(Conversion {
                destination_first: 0,
                source_first: 0,
                range_len: first_conversion.source_first,
            })
        }
    }

    for next_range in sorted_conversions {
        if let Some(prev_range) = new_conversions.last() {
            let prev_end = prev_range.source_last() + 1;
            let next_start = next_range.source_first;
            if prev_end < next_start {
                // insert missing range
                new_conversions.push(Conversion {
                    destination_first: prev_end,
                    source_first: prev_end,
                    range_len: (next_start - prev_end),
                });
            }
        }

        // add existing range
        new_conversions.push(next_range);
    }

    // add last to u32::MAX
    if let Some(last_conversion) = new_conversions.last() {
        let last_source_value = last_conversion.source_last();
        if last_source_value < u32::MAX {
            let new_source_first = last_source_value + 1;
            new_conversions.push(Conversion {
                destination_first: new_source_first,
                source_first: new_source_first,
                range_len: u32::MAX - last_source_value,
            })
        }
    }

    return new_conversions;
}

fn combine_adjacent(mut ranges: Vec<RangeInclusive<u32>>) -> Vec<RangeInclusive<u32>> {
    ranges.sort_by_key(|r| r.start().to_owned());
    combine_adjacent_sorted(ranges)
}

fn combine_adjacent_sorted(ranges: Vec<RangeInclusive<u32>>) -> Vec<RangeInclusive<u32>> {
    let mut new_ranges: Vec<RangeInclusive<u32>> = Vec::new();
    let mut num_combined = 0;
    for next_range in ranges {
        if let Some(prev_range) = new_ranges.pop() {
            let next_start = *next_range.start();
            let next_end = *next_range.end();
            let prev_start = *prev_range.start();
            let prev_end = *prev_range.end();

            // if (prev_end is adjacent to next_start), avoiding overflow if prev_end == u32::MAX
            if prev_end >= next_start - 1 {
                new_ranges.push(prev_start..=next_end);
                num_combined += 1;
            } else {
                new_ranges.push(prev_range);
                new_ranges.push(next_range);
            }
        } else {
            new_ranges.push(next_range);
        }
    }

    println!("combined {num_combined} ranges");
    return new_ranges;
}

fn apply_conversions_on_ranges(
    conversions: &Vec<Conversion>,
    ranges: Vec<RangeInclusive<u32>>,
) -> Vec<RangeInclusive<u32>> {
    let mut new_ranges = Vec::new();

    for r in ranges {
        let first_value = *r.start();
        let last_value = *r.end();

        let first_conversion_index = find_conversion_index(conversions, first_value);
        let last_conversion_index = find_conversion_index(conversions, last_value);

        if last_conversion_index == first_conversion_index {
            let conversion = &conversions[first_conversion_index];
            let new_first = conversion.map(first_value);
            let new_last = conversion.map(last_value);
            new_ranges.push(new_first..=new_last)
        } else {
            let new_first = {
                let conversion = &conversions[first_conversion_index];
                conversion.map(first_value)..=conversion.destination_last()
            };
            new_ranges.push(new_first);

            if last_conversion_index - first_conversion_index > 2 {
                // there are conversions between the start and end conversion
                for index in (first_conversion_index + 1)..(last_conversion_index - 1) {
                    new_ranges.push(conversions[index].destination_range());
                }
            }

            let new_last = {
                let conversion = &conversions[last_conversion_index];
                conversion.destination_first..=conversion.map(last_value)
            };
            new_ranges.push(new_last);
        }
    }

    return new_ranges;
}

fn find_conversion_index(conversions: &Vec<Conversion>, current_value: u32) -> usize {
    let search_result = conversions.binary_search_by_key(&current_value, |c| c.source_first);
    // we want the index that starts before or at the given value
    let conversion_index = match search_result {
        Ok(i) => i,
        Err(0) => 0,
        Err(i) => i - 1,
    };
    conversion_index
}
