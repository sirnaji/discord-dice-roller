use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Locale
{
    pub infos: Infos,
    pub translations: Translations,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Infos
{
    pub name: String,
    pub native_name: String,
    pub emoji: Emoji,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Emoji
{
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Translations
{
    pub general: General,
    pub commands: Commands,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct General
{
    pub guild_only_cmd: String,
    pub action_error_title: String,
    pub action_error_desc: String,
    pub error: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Commands
{
    pub roll: Roll,
    pub setlang: Setlang,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Roll
{
    pub name: String,
    pub description: String,
    pub dictionary: RollDictionary,
    pub roll_details: RollDetails,
    pub threshold: RollThreshold,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RollDictionary
{
    pub die: String,
    pub dice: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RollDetails
{
    pub rolling_multiple_dice: String,
    pub rolling_single_die: String,
    pub successful_dice_rolls: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RollThreshold
{
    pub critical_failure: String,
    pub critical_success: String,
    pub failure: String,
    pub success: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Setlang
{
    pub name: String,
    pub description: String,
    pub updated_to_language_title: String,
    pub updated_to_language_desc: String,
    pub select_new_language: String,
    pub errors: SetlangErrors,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetlangErrors
{
    pub threshold_needed_description: String,
    pub threshold_needed_footer: String,
}
