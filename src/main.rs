use std::io::stdin;
use owo_colors::OwoColorize;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use atty::Stream;
use clap::Parser;
use primefactor::PrimeFactors;

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

fn factor(num: u128) -> Vec<u128> {
    let factors = PrimeFactors::from(num);
    if factors.is_prime() {
        vec![num]
    } else {
        factors.to_vec()
    }
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

        let mut rl = Editor::<()>::new().expect("Could not create interactive session. Make sure this is a TTY stream!");
        loop {
            let readline = rl.readline(format!("{} ", ">>".black()).as_str());
            match readline {
                Ok(line) => {

                    if line.to_lowercase() == "exit" {
                        break;
                    }

                    rl.add_history_entry(line.as_str());
                    let input = line.trim().parse::<u128>();

                    match input {
                        Ok(input) => {
                            let factors = factor(input);

                            println!(
                                "{}",
                                factors
                                    .iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(" ")
                            );
                        },
                        Err(_) => {
                            eprintln!("{} Expected a number, got '{}' instead.", "error:".red(), line);
                            continue;
                        }
                    }
                },
                Err(ReadlineError::Interrupted) => {
                    break
                },
                Err(ReadlineError::Eof) => {
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }

        return;
    }

    // Read from stdin
    for line in stdin().lines() {
        let line = line.unwrap();

        let input = line.parse::<u128>();

        match input {
            Ok(input) => {
                let factors = factor(input);

                println!(
                    "{}",
                    factors
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            },
            Err(_) => {
                eprintln!("Expected a number, got {} instead.", line);
                return;
            }
        }
    }
}
