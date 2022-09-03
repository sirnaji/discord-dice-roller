use regex::Regex;
use std::fmt;

pub enum ParseError
{
    DiceAmount,
    Threshold,
    DiceSize,
    Modifier,
    Input,
}

impl fmt::Debug for ParseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            ParseError::DiceAmount => write!(f, "Invalid dice amount"),
            ParseError::Threshold => write!(f, "Invalid threshold"),
            ParseError::DiceSize => write!(f, "Invalid dice size"),
            ParseError::Modifier => write!(f, "Invalid modifier"),
            ParseError::Input => write!(f, "Invalid input"),
        }
    }
}

#[derive(Debug)]
pub struct ParsedRoll
{
    pub dice_amount: Option<u8>,
    pub dice_size: Option<u8>,
    pub threshold: Option<u8>,
    pub threshold_modifier: Option<TresholdModifier>,
}

#[derive(Debug)]
pub struct TresholdModifier
{
    pub is_positive: bool,
    pub value: u8,
}

pub fn parse_input(input: &str) -> Result<ParsedRoll, ParseError>
{
    let input_validator = Regex::new(r"((?P<dice_amount>\d*)?[dD](?P<dice_size>\d+)?)?(?P<threshold>([<](?P<threshold_value>\d+))(?P<modifier>(?P<modifier_sign>[+-])(?P<modifier_value>\d+))?)?").unwrap();

    let captures = input_validator.captures(input).ok_or(ParseError::Input)?;

    let dice_amount = {
        if captures.name("dice_amount").is_some()
        {
            let amount = captures
                .name("dice_amount")
                .ok_or(ParseError::Input)?
                .as_str()
                .parse::<u8>()
                .map_err(|_| ParseError::DiceAmount);

            match amount
            {
                Ok(amount) => Some(amount),
                Err(_) => None,
            }
        }
        else
        {
            Some(1)
        }
    };

    let dice_size = {
        if captures.name("dice_size").is_some()
        {
            let size = captures
                .name("dice_size")
                .ok_or(ParseError::Input)?
                .as_str()
                .parse::<u8>()
                .map_err(|_| ParseError::DiceSize);

            match size
            {
                Ok(size) => Some(size),
                Err(_) => None,
            }
        }
        else
        {
            Some(20)
        }
    };

    let threshold = {
        if captures.name("threshold").is_some()
        {
            let threshold = captures
                .name("threshold_value")
                .ok_or(ParseError::Input)?
                .as_str()
                .parse::<u8>()
                .map_err(|_| ParseError::Threshold);

            match threshold
            {
                Ok(threshold) => Some(threshold),
                Err(_) => None,
            }
        }
        else
        {
            None
        }
    };

    let threshold_modifier = {
        if captures.name("modifier").is_some()
        {
            let modifier_sign = captures
                .name("modifier_sign")
                .ok_or(ParseError::Input)?
                .as_str()
                .parse::<char>()
                .map_err(|_| ParseError::Modifier)?;

            let modifier_value = captures
                .name("modifier_value")
                .ok_or(ParseError::Input)?
                .as_str()
                .parse::<u8>()
                .map_err(|_| ParseError::Modifier)?;

            let is_positive = modifier_sign == '+';
            let value = modifier_value;

            Some(TresholdModifier { is_positive, value })
        }
        else
        {
            None
        }
    };

    Ok(ParsedRoll {
        dice_amount,
        dice_size,
        threshold,
        threshold_modifier,
    })
}
