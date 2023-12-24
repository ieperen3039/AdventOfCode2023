#[allow(dead_code)]
mod trebuchet;
mod cubes;
mod gears;
mod scratchcards;
mod seeds;

use std::path::PathBuf;

use clap::Parser;

/// All exercises for advent of code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CommandlineArguments {
    /// which puzzle to solve
    #[arg(short, long)]
    day: i32,
    /// input file
    #[arg(short, long)]
    input: PathBuf,
}

fn main() {
    // let args = CommandlineArguments::parse();
    let args = CommandlineArguments {
        day: 5,
        input: PathBuf::from("inputs/seeds.txt")
    };

    let input_text = {
        let file_contents = std::fs::read_to_string(&args.input);
        if file_contents.is_err() {
            panic!("File {} can not be opened", args.input.display())
        } else {
            file_contents.unwrap()
        }
    };

    match args.day {
        1 => {
            let value = trebuchet::decode_calibration(input_text);
            println!("{value}");
        },
        2 => {
            let value = cubes::filter_games(input_text);
            println!("{value}");
        },
        3 => {
            let value = gears::part_2_find_gear_ratios(input_text);
            println!("{value}");
        },
        4 => {
            let value = scratchcards::part_2_get_total_scratchcards(input_text);
            println!("{value}");
        },
        5 => {
            let value = seeds::part_2_calculate_range_mapping(input_text);
            println!("{value}");
        },
        _ => panic!("day {} not defined", args.day)
    }

}
