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
    pub commands: Commands,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Commands
{
    pub roll: Roll,
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
