use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufReader};
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use sudoku::Sudoku;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(author = "DimitriLW <5898931+dimitrilw@users.noreply.github.com>")]
#[command(about = "Solves sudoku puzzles.")]
struct Cli {
    /// File containing one or more sudoku puzzles.
    file: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// No purpose to this sub-command; just playing with clap crate.
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // block from clap tutorial
    // You can see how many times a particular flag or argument occurred.
    // Note, only flags can have multiple occurrences.
    let caveat = "Note: Debug mode has no purpose other than testing the clap crate.\n";
    match cli.debug {
        0 => {},
        1 => println!("{caveat}Debug mode is kind of on (1)"),
        2 => println!("{caveat}Debug mode is on (2)"),
        _ => println!("{caveat}Don't be crazy (3+)"),
    }

    // block from clap tutorial
    // You can check for the existence of subcommands, and if found, 
    // then use their matches just as you would the top level cmd.
    match &cli.command {
        Some(Commands::Test { list }) => {
            println!("Note: Test command has no purpose other than playing with clap crate.");
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    if let Some(file_path) = cli.file.as_path().to_str() {
        let f_in = BufReader::new(File::open(file_path)?);

        let f_out_path = format!("{}.solved.txt", file_path);
        if Path::new(&f_out_path).exists() {
            std::fs::remove_file(&f_out_path).unwrap();
        }
        let mut f_out = OpenOptions::new()
            .create(true)
            .append(true)
            .open(f_out_path)
            .unwrap();

        let mut i = 0;
        for line in f_in.lines() {
            i += 1;
            let sudoku = Sudoku::from_str_line(&line?).unwrap();
            let num_solutions = sudoku.count_at_most(9);
            match num_solutions {
                0 => writeln!(f_out, "Puzzle #{i} is unsolvable.")?,
                1 => writeln!(f_out, "Puzzle #{i} has 1 solution. This is it:")?,
                9 => writeln!(f_out, "Puzzle #{i} has 9-or-more solutions. This is one:")?,
                _ => writeln!(f_out, "Puzzle #{i} has {num_solutions} solutions. This is one:")?,
            }
            if num_solutions > 0 {
                writeln!(
                    f_out,
                    "{}\n",
                    sudoku.solve_one().expect("should have been able to solve"),
                )?;
            }
        }
    }
    Ok(())
}