use std::{
    fmt::{self, Display},
    io,
};

use enum_iterator::Sequence;
use nom::IResult;

use crate::rover::Rover;

#[derive(Debug)]
pub enum RoverErr {
    Opening(io::Error),
    Reading(io::Error),
    Saving(io::Error),
    Parse(ParsingErr, usize),
    Boundery(Rover, usize),
}

impl RoverErr {
    // Convienience helper for converting between result types
    pub fn from_parse_result<T>(input: IResult<&str, T>, line_index: usize) -> Result<T, RoverErr> {
        match input {
            // returns ok if there are no characters left in the string
            Ok((s, t)) if s.is_empty() => Ok(t),
            _ => Err(RoverErr::Parse(ParsingErr::UnexpectedToken, line_index)), // TODO: improve error by displaying the position of the unexpected token
        }
    }
}

#[derive(Debug)]
pub enum ParsingErr {
    MissingPlateauBounderies,
    MissingInstructions,
    UnexpectedToken,
}

impl Display for ParsingErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParsingErr::MissingPlateauBounderies => "Missing plateau bounderies",
                ParsingErr::MissingInstructions => "Missing instructions for rover",
                ParsingErr::UnexpectedToken => "Unexpected token encountered",
            }
        )
    }
}

impl Display for RoverErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Generate a pretty error message for the different errors that can occur.
        let (msg, e) = match self {
            RoverErr::Opening(e) => ("opening the instructions file", e),
            RoverErr::Reading(e) => ("reading in the instructions file", e),
            RoverErr::Saving(e) => ("saving the output file", e),
            RoverErr::Parse(e, index) => {
                return write!(
                f,
                "Rover Error 🤖 - Issue whilst parsing instructions file: {}, At line: {}",
                e,
                index + 1
            )
            }
            RoverErr::Boundery(rover, instruction) => {
                return write!(
                    f,
                    "Rover Error 🤖 - Rover {} crossed the plateau's boundery at position ({}, {}): Instruction {}, At Line: {}.\n\nPlease send help! 😞",
                    rover.id,
                    rover.x,
                    rover.y,
                    instruction + 1,
                    (rover.id * 2) + 1
                )
            }
        };

        write!(f, "Rover Error 🤖 - Issue whilst {msg}: {}", e)
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Instruction {
    #[default]
    Move,
    Left,
    Right,
}

impl From<&str> for Instruction {
    fn from(c: &str) -> Self {
        match c {
            "M" | "m" => Instruction::Move,
            "L" | "l" => Instruction::Left,
            "R" | "r" => Instruction::Right,
            _ => Instruction::default(),
        }
    }
}

#[derive(Debug, Default, Sequence, PartialEq)]
pub enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "N" | "n" => Direction::North,
            "E" | "e" => Direction::East,
            "S" | "s" => Direction::South,
            "W" | "w" => Direction::West,
            _ => Direction::default(),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "N",
                Direction::East => "E",
                Direction::South => "S",
                Direction::West => "W",
            }
        )
    }
}

pub type Coordinate = (isize, isize);
