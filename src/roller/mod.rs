pub use {handler::*, parser::*, random::*};

mod handler;
mod parser;
mod random;

pub fn new(user_input: &str) -> Result<handler::RollResult, parser::ParseError> {
    let parsed_input = parser::parse_input(user_input);
    handler::handle_roll(parsed_input)
}
