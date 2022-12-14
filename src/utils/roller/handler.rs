use crate::utils::i18n::locale::Locale;

use super::parser;
use super::random;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Threshold
{
    CriticalSuccess,
    CriticalFailure,
    Success,
    Failure,
    None,
}

impl Threshold
{
    pub fn get_color_emoji(&self) -> String
    {
        match self
        {
            Threshold::CriticalSuccess => ":blue_circle:".to_string(),
            Threshold::CriticalFailure => ":red_circle:".to_string(),
            Threshold::Success => ":green_circle:".to_string(),
            Threshold::Failure => ":yellow_circle:".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn get_emote_emoji(&self) -> String
    {
        match self
        {
            Threshold::CriticalSuccess => ":heart_eyes:".to_string(),
            Threshold::CriticalFailure => ":scream:".to_string(),
            Threshold::Success => ":innocent:".to_string(),
            Threshold::Failure => ":pleading_face:".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn get_text(&self, locale: &Locale) -> String
    {
        match self
        {
            Threshold::CriticalSuccess => locale
                .translations
                .commands
                .roll
                .threshold
                .critical_success
                .clone(),
            Threshold::CriticalFailure => locale
                .translations
                .commands
                .roll
                .threshold
                .critical_failure
                .clone(),
            Threshold::Success => locale.translations.commands.roll.threshold.success.clone(),
            Threshold::Failure => locale.translations.commands.roll.threshold.failure.clone(),
            _ => "".to_string(),
        }
    }

    pub fn get_color(&self) -> u32
    {
        match self
        {
            Threshold::CriticalSuccess => 0x55ACEE,
            Threshold::CriticalFailure => 0xDD2E44,
            Threshold::Success => 0x78B159,
            Threshold::Failure => 0xFF9800,
            _ => 0x2F3136,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Roll
{
    pub value: u8,
    pub threshold: Threshold,
}

#[derive(Debug)]
pub struct RollResult
{
    pub roll_command: parser::ParsedRoll,
    pub rolls: Vec<Roll>,
    pub threshold_value: Option<u8>,
}

pub fn handle_roll(
    roll_command: Result<parser::ParsedRoll, parser::ParseError>,
) -> Result<RollResult, parser::ParseError>
{
    match roll_command
    {
        Ok(roll_command) =>
        {
            let dice_amount: u8 = roll_command.dice_amount.unwrap_or(1);
            let dice_size: u8 = roll_command.dice_size.unwrap_or(20);

            let raw_rolls = random::roll(dice_amount, dice_size);

            let mut threshold_value: Option<u8> = None;

            if roll_command.threshold.is_some()
            {
                if roll_command.threshold_modifier.is_some()
                {
                    let modifier = roll_command.threshold_modifier.as_ref().unwrap();
                    let modifier_is_positive = modifier.is_positive;
                    let modifier_value: u8 = modifier.value;

                    if modifier_is_positive
                    {
                        if roll_command
                            .threshold
                            .unwrap()
                            .checked_add(modifier_value)
                            .is_none()
                        {
                            return Err(parser::ParseError::Modifier);
                        }
                        threshold_value = Some(roll_command.threshold.unwrap() + modifier_value);
                    }
                    else
                    {
                        if roll_command
                            .threshold
                            .unwrap()
                            .checked_sub(modifier_value)
                            .is_none()
                        {
                            return Err(parser::ParseError::Modifier);
                        }
                        threshold_value = Some(roll_command.threshold.unwrap() - modifier_value);
                    }
                }
                else
                {
                    threshold_value = Some(roll_command.threshold.unwrap());
                }
            }

            let rolls: Vec<Roll> = raw_rolls
                .iter()
                .map(|raw_roll| Roll {
                    value: raw_roll.clone(),
                    threshold: match threshold_value
                    {
                        Some(threshold) =>
                        {
                            // If die is 20-sided
                            if dice_size == 20
                            {
                                // 20 is always a critical failure
                                if *raw_roll == 20
                                {
                                    Threshold::CriticalFailure
                                }
                                else
                                {
                                    // When the threshold is greater than 20,
                                    // the critical success levels are calculated
                                    if threshold > 20
                                    {
                                        let critical_threshold = threshold - 20;

                                        // If roll value is in the critical threshold range,
                                        // Its a critical success
                                        if *raw_roll <= critical_threshold
                                        {
                                            Threshold::CriticalSuccess
                                        }
                                        else
                                        {
                                            Threshold::Success
                                        }
                                    }
                                    else
                                    {
                                        // Critical Success if roll value equals to threshold
                                        if *raw_roll == threshold
                                        {
                                            Threshold::CriticalSuccess
                                        }
                                        // Success if roll value less or equals to threshold
                                        else if *raw_roll <= threshold
                                        {
                                            Threshold::Success
                                        }
                                        // Failure
                                        else
                                        {
                                            Threshold::Failure
                                        }
                                    }
                                }
                            }
                            else
                            {
                                if *raw_roll <= threshold
                                {
                                    Threshold::Success
                                }
                                else
                                {
                                    Threshold::Failure
                                }
                            }
                        }

                        None => Threshold::None,
                    },
                })
                .collect();

            Ok(RollResult {
                roll_command,
                rolls,
                threshold_value,
            })
        }

        Err(err) => Err(err),
    }
}
