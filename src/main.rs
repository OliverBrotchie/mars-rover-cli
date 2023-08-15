pub mod enums;
pub mod parse;
pub mod rover;

use std::{fs, path::PathBuf, process::ExitCode};

use clap::Parser;
use enums::RoverErr;

use crate::rover::RoverControlSatellite;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the instructions file.
    input_path: PathBuf,

    /// Return an error if the rover exits plateau.
    #[arg(short, long)]
    unbounded: bool,

    /// A path to save the output a a file. By default, the output will be printed to stdout.
    #[clap(short = 'o', long = "output")]
    output: Option<PathBuf>,
}

/// Cli wrapper function
fn main() -> ExitCode {
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = parse_input_and_output_result(args) {
        eprintln!("{err}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[inline]
fn parse_input_and_output_result(args: Args) -> Result<(), RoverErr> {
    // Open instructions file
    let file = fs::read_to_string(args.input_path).map_err(RoverErr::Opening)?;

    let rovers = RoverControlSatellite::parse_and_execute_incoming_message(file, args.unbounded)?;
    let output = rovers
        .into_iter()
        .map(|rover| rover.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    // Output the result
    if let Some(output_path) = args.output {
        fs::write(output_path, output).map_err(RoverErr::Saving)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}
