use enum_iterator::{next_cycle, previous_cycle};

use crate::{
    enums::{Coordinate, Direction, Instruction, ParsingErr, RoverErr},
    parse::{coordinate, instruction_stream, starting_position},
};

#[derive(Debug, PartialEq)]
pub struct Rover {
    pub id: usize,
    pub x: isize,
    pub y: isize,
    pub facing: Direction,
}

impl Rover {
    pub fn new(id: usize, coordinates: Coordinate, facing: Direction) -> Self {
        Rover {
            id,
            x: coordinates.0,
            y: coordinates.1,
            facing,
        }
    }

    pub fn execute_commands(
        mut self,
        instructions: Vec<Instruction>,
        boundery: Option<Coordinate>,
    ) -> Result<Self, RoverErr> {
        for (i, instruction) in instructions.iter().enumerate() {
            match instruction {
                Instruction::Left => self.facing = previous_cycle(&self.facing).unwrap_or_default(),
                Instruction::Right => self.facing = next_cycle(&self.facing).unwrap_or_default(),
                Instruction::Move => match self.facing {
                    Direction::North => self.y += 1,
                    Direction::East => self.x += 1,
                    Direction::South => self.y -= 1,
                    Direction::West => self.x -= 1,
                },
            }

            if self.has_crossed_boundery(boundery) {
                return Err(RoverErr::Boundery(self, i));
            }
        }

        Ok(self)
    }

    pub fn has_crossed_boundery(&self, boundery: Option<Coordinate>) -> bool {
        match boundery {
            Some((x, y)) => self.x < 0 || self.y < 0 || self.x > x || self.y > y,
            None => false,
        }
    }
}

impl ToString for Rover {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.facing)
    }
}

pub struct RoverControlSatellite;

impl RoverControlSatellite {
    pub fn parse_and_execute_incoming_message(
        message: String,
        unbounded: bool,
    ) -> Result<Vec<Rover>, RoverErr> {
        let mut lines = message.lines().map(|line| line.trim()).enumerate();
        let bounderies = Self::parse_bounderies(lines.next())?;

        let mut instructions_and_positions = Vec::new();
        while let Some(entry) = Self::parse_instructions_and_position((lines.next(), lines.next()))?
        {
            instructions_and_positions.push(entry)
        }

        instructions_and_positions
            .into_iter()
            .enumerate()
            .map(|(index, ((coordinates, direction), instructions))| {
                Rover::new(index + 1, coordinates, direction)
                    .execute_commands(instructions, (!unbounded).then(|| bounderies))
            })
            .collect()
    }

    /// Get the bounderies of the plateau
    pub fn parse_bounderies(input: Option<(usize, &str)>) -> Result<Coordinate, RoverErr> {
        match input {
            Some((_, line)) => RoverErr::from_parse_result(coordinate(line), 0),
            None => Err(RoverErr::Parse(ParsingErr::MissingPlateauBounderies, 0)),
        }
    }

    /// Get the starting positions and instructions for a rover
    pub fn parse_instructions_and_position(
        input: (Option<(usize, &str)>, Option<(usize, &str)>),
    ) -> Result<Option<((Coordinate, Direction), Vec<Instruction>)>, RoverErr> {
        match input {
            (
                Some((starting_pos_index, starting_pos)),
                Some((instructions_index, instructions)),
            ) => Ok(Some((
                RoverErr::from_parse_result(starting_position(starting_pos), starting_pos_index)?,
                RoverErr::from_parse_result(instruction_stream(instructions), instructions_index)?,
            ))),
            // Catch when there is an uneven number of co-ordinate/instruction groupings
            (Some((previous_index, _)), None) => Err(RoverErr::Parse(
                ParsingErr::MissingInstructions,
                previous_index,
            )),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod rover_module {
    use super::*;

    #[cfg(test)]
    mod rover {
        use super::*;

        #[cfg(test)]
        mod has_crossed_boundery {
            use super::Rover;
            use crate::enums::Direction;

            #[test]
            fn none_input() {
                let rover = Rover::new(0, (-1, -1), Direction::North);
                assert!(!rover.has_crossed_boundery(None));
            }

            #[test]
            fn out_of_supplied_bounds() {
                let rover = Rover::new(0, (5, 5), Direction::North);
                assert!(rover.has_crossed_boundery(Some((2, 2))));
            }

            #[test]
            fn out_of_implied_bounds() {
                let rover = Rover::new(0, (-1, -1), Direction::North);
                assert!(rover.has_crossed_boundery(Some((2, 2))));
            }
        }

        #[cfg(test)]
        mod execute_commands {

            use super::Rover;
            use crate::enums::{Direction, Instruction};

            #[test]
            fn valid_input_with_all_directions() {
                let rover = Rover::new(0, (0, 0), Direction::North);
                let result = rover.execute_commands(
                    vec![
                        Instruction::Move,
                        Instruction::Right,
                        Instruction::Move,
                        Instruction::Right,
                        Instruction::Move,
                        Instruction::Right,
                        Instruction::Move,
                        Instruction::Right,
                    ],
                    None,
                );
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), Rover::new(0, (0, 0), Direction::North));
            }

            #[test]
            fn crosses_boundery() {
                let rover = Rover::new(0, (0, 0), Direction::North);
                let result = rover
                    .execute_commands(vec![Instruction::Left, Instruction::Move], Some((5, 5)));
                assert!(result.is_err());
            }
        }
    }

    mod rover_control_satelite {
        use super::*;

        #[cfg(test)]
        mod parse_bounderies {
            use super::RoverControlSatellite;

            #[test]
            fn valid_input() {
                let result = RoverControlSatellite::parse_bounderies(Some((0, "5 5")));
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), (5, 5))
            }

            #[test]
            fn alphabetic_input() {
                let result = RoverControlSatellite::parse_bounderies(Some((0, "foo bar")));
                assert!(result.is_err());
            }
        }

        #[cfg(test)]
        mod parse_instructions_and_position {
            use crate::enums::{Direction, Instruction};

            use super::RoverControlSatellite;

            #[test]
            fn valid_input() {
                let result = RoverControlSatellite::parse_instructions_and_position((
                    Some((0, "1 1 N")),
                    Some((0, "M")),
                ));
                assert!(result.is_ok());
                assert_eq!(
                    result.unwrap(),
                    Some((((1, 1), Direction::North), vec![Instruction::Move]))
                )
            }

            #[test]
            fn invalid_position() {
                let result = RoverControlSatellite::parse_instructions_and_position((
                    Some((0, "1 N")),
                    Some((0, "M")),
                ));
                assert!(result.is_err());
            }

            #[test]
            fn invalid_instructions() {
                let result = RoverControlSatellite::parse_instructions_and_position((
                    Some((0, "1 1 N")),
                    Some((0, "d")),
                ));
                assert!(result.is_err());
            }

            #[test]
            fn only_one_input() {
                let result = RoverControlSatellite::parse_instructions_and_position((
                    Some((0, "1 1 N")),
                    None,
                ));
                assert!(result.is_err());
            }

            #[test]
            fn no_inputs() {
                let result = RoverControlSatellite::parse_instructions_and_position((None, None));
                assert!(result.is_ok());
                assert!(result.unwrap().is_none())
            }
        }

        #[cfg(test)]
        mod parse_and_execute_incoming_message {
            use super::{Rover, RoverControlSatellite};

            #[test]
            fn valid_input() {
                let result = RoverControlSatellite::parse_and_execute_incoming_message(
                    r#"5 5
                    1 2 N
                    LMLMLMLMM
                    3 3 E
                    MMRMMRMRRM"#
                        .to_string(),
                    false,
                );
                assert!(result.is_ok());
                assert_eq!(
                    result.unwrap(),
                    vec![
                        Rover::new(1, (1, 3), crate::enums::Direction::North),
                        Rover::new(2, (5, 1), crate::enums::Direction::East)
                    ]
                )
            }

            #[test]
            fn crosses_boundery_unbounded() {
                let result = RoverControlSatellite::parse_and_execute_incoming_message(
                    r#"2 2
                    0 0 N
                    LM"#
                    .to_string(),
                    true,
                );
                assert!(result.is_ok());
                assert_eq!(
                    result.unwrap(),
                    vec![Rover::new(1, (-1, 0), crate::enums::Direction::West),]
                )
            }

            #[test]
            fn crosses_boundery_bounded() {
                let result = RoverControlSatellite::parse_and_execute_incoming_message(
                    r#"2 2
                    0 0 N
                    LM"#
                    .to_string(),
                    false,
                );
                assert!(result.is_err());
            }
        }
    }
}
