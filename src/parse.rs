use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{map_res, recognize},
    error::context,
    multi::{many0, many1},
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::enums::{Coordinate, Direction, Instruction};

pub fn decimal(input: &str) -> IResult<&str, isize> {
    map_res(
        context(
            "decimal",
            recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        ),
        |s: &str| s.parse::<isize>(),
    )(input)
}

pub fn coordinate(input: &str) -> IResult<&str, (isize, isize)> {
    separated_pair(decimal, multispace1, decimal)(input)
}

pub fn direction(input: &str) -> IResult<&str, Direction> {
    context(
        "direction",
        alt((
            tag_no_case("N"),
            tag_no_case("E"),
            tag_no_case("S"),
            tag_no_case("W"),
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn instruction(input: &str) -> IResult<&str, Instruction> {
    context(
        "instruction",
        alt((tag_no_case("M"), tag_no_case("L"), tag_no_case("R"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn starting_position(input: &str) -> IResult<&str, (Coordinate, Direction)> {
    separated_pair(coordinate, multispace0, direction)(input)
}

pub fn instruction_stream(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(terminated(instruction, multispace0))(input)
}

#[cfg(test)]
mod parse_module {
    use super::*;

    #[cfg(test)]
    mod decimal {
        use super::decimal;

        #[test]
        fn valid_input() {
            let result = decimal("123");
            assert_eq!(result, Ok(("", 123)));
        }

        #[test]
        fn alphabetic_characters() {
            let result = decimal("abc");
            assert!(result.is_err());
        }

        #[test]
        fn negative_number() {
            let result = decimal("-1");
            assert!(result.is_err());
        }
    }

    #[cfg(test)]
    mod coordinate {
        use super::coordinate;

        #[test]
        fn valid_input() {
            let result = coordinate("123   456");
            assert_eq!(result, Ok(("", (123, 456))));
        }

        #[test]
        fn external_whitespace() {
            let result = coordinate("  123   456  ");
            assert!(result.is_err());
        }
    }

    #[cfg(test)]
    mod direction {
        use crate::enums::Direction;

        use super::direction;

        #[test]
        fn valid_input() {
            let results = (direction("N"), direction("n"));
            assert_eq!(results.0, Ok(("", Direction::North)));
            assert_eq!(results.1, Ok(("", Direction::North)));
        }

        #[test]
        fn invalid_tag() {
            let result = direction("c");
            assert!(result.is_err());
        }
    }

    #[cfg(test)]
    mod instruction {
        use crate::enums::Instruction;

        use super::instruction;

        #[test]
        fn valid_input() {
            let results = (instruction("M"), instruction("m"));
            assert_eq!(results.0, Ok(("", Instruction::Move)));
            assert_eq!(results.1, Ok(("", Instruction::Move)));
        }

        #[test]
        fn invalid_tag() {
            let result = instruction("c");
            assert!(result.is_err());
        }
    }

    #[cfg(test)]
    mod starting_position {
        use crate::enums::Direction;

        use super::starting_position;

        #[test]
        fn valid_input() {
            let result = starting_position("0 0 E");
            assert_eq!(result, Ok(("", ((0, 0), Direction::East))));
        }

        #[test]
        fn missing_direction() {
            let result = starting_position("2 2");
            assert!(result.is_err())
        }
    }

    #[cfg(test)]
    mod instruction_stream {
        use crate::enums::Instruction;

        use super::instruction_stream;

        #[test]
        fn valid_input() {
            let result = instruction_stream("LMR");
            assert_eq!(
                result,
                Ok((
                    "",
                    vec![Instruction::Left, Instruction::Move, Instruction::Right]
                ))
            );
        }

        #[test]
        fn valid_input_with_spaces() {
            let result = instruction_stream("L M  R   ");
            assert_eq!(
                result,
                Ok((
                    "",
                    vec![Instruction::Left, Instruction::Move, Instruction::Right]
                ))
            );
        }
    }
}
