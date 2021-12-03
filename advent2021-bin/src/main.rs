use color_eyre::Report;
use colored::*;
use structopt::StructOpt;

use advent2021_lib::get_days;
use advent2021_lib::get_input;
use advent2021_lib::DayOutput;

#[derive(StructOpt)]
struct Cli {
    puzzle: Option<usize>,

    #[structopt(long)]
    all: bool,

    #[structopt(long)]
    parallel: bool,
}

fn print_day(day_num: usize, day: DayOutput) {
    println!("Day {}", day_num);
    println!("Part 1: {}", day.display.0);
    println!("Part 2: {}", day.display.1);
    println!();
}

fn print_day_visual(day_num: usize, day: DayOutput) {
    println!("Day {}", day_num);
    println!();
    if let Some(s) = day.visual {
        println!("{}", s);
        println!();
    }
    println!("Part 1: {}", day.display.0);
    println!("Part 2: {}", day.display.1);
    println!();
}

fn main() -> Result<(), Report> {
    setup()?;

    println!("{}", "Advent Of Code 2020".bold().blue());
    println!();

    let args = Cli::from_args();

    if args.all {
        for (day_num, day) in get_days().into_iter() {
            print_day(day_num, (day.calc)(get_input(day_num)));
        }
    }

    if args.parallel {
        let threads = get_days().into_iter().map(|(day_num, day)| {
            println!("Spawn day {}", day_num);
            std::thread::spawn(move || (day.calc)(get_input(day_num)))
        });
        std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_millis(50));
        println!();
        for (idx, thread) in threads.into_iter().enumerate() {
            print_day(idx + 1, thread.join().unwrap());
        }
    }

    if !(args.all || args.parallel) {
        let days = get_days();
        match args.puzzle {
            None => {
                let (&day_num, day) = days.iter().next_back().unwrap();
                print_day(day_num, (day.calc)(get_input(day_num)));
            }
            Some(day_num) => {
                let day = days.get(&day_num).expect("invalid day");
                print_day_visual(day_num, (day.calc)(get_input(day_num)));
            }
        };
    }

    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1")
    }
    color_eyre::install()?;

    Ok(())
}
