use std::io::stdin;
use owo_colors::OwoColorize;

use atty::Stream;
use clap::Parser;

fn factor(mut input: u128) -> Vec<u128> {
    let mut factors = Vec::new();

    let mut candidates = 2..;

    while input > 1 {
        let x = candidates.next().unwrap();

        while input % x == 0 {
            input /= x;

            factors.push(x);
        }
    }

    factors
}

/// Gets the prime factors for any integer. It will use stdin if none are specified.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run interactively (meant for TTY)
    #[arg(short, long)]
    interactive: bool,
    /// List of numbers to factor.
    numbers: Option<Vec<u128>>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(numbers) = cli.numbers {
        for input in numbers {
            let factors = factor(input);

            println!(
                "{}",
                factors
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
        return;
    }
    

    if atty::is(Stream::Stdin) && !cli.interactive {
        eprintln!("{}", "non-tty stream expected\n".red().bold());
        eprintln!("Run with arguments [ {} ], or run interactively [ {} ].", "factorr 127 581".italic().green(), "factorr --interactive".italic().green());
        eprintln!("Run {} for more information.", "factorr --help".bold().yellow());
        return;
    }

    if cli.interactive {
        println!("Welcome to {}! Enter a number to factor, or enter: {}, {}, or type '{}' to quit.", "factorr".yellow(), "CTRL + D".green(), "CTRL + C".green(), "exit".red());
    }

    // Read from stdin
    for line in stdin().lines() {
        let line = line.unwrap();

        if cli.interactive && line == "exit" {
            break;
        }

        let input = line.parse::<u128>();

        match input {
            Ok(input) => {
                let factors = factor(input);

                println!(
                    "{} {}",
                    if cli.interactive { ">" } else { "" },
                    factors
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            },
            Err(_) => {
                if cli.interactive {
                    println!("{} {} {}", "Invalid input -- expected a number, got".red(), line.red().bold(), "instead.".red());
                } else {
                    eprintln!("Expected a number, got {} instead.", line);
                    return;
                }
            }
        }
    }
}
