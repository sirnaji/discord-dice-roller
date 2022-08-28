use self::DiscordSupportedLanguage::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum DiscordSupportedLanguage {
    BG,
    CS,
    DA,
    DE,
    EL,
    ESES,
    ENGB,
    ENUS,
    FI,
    FR,
    HI,
    HR,
    HU,
    IT,
    JA,
    KO,
    LT,
    NL,
    NO,
    PL,
    PTBR,
    RO,
    RU,
    SVSE,
    TH,
    TR,
    UK,
    VI,
    ZHCN,
    ZHTW,
}

impl DiscordSupportedLanguage {
    pub fn iter() -> impl Iterator<Item = DiscordSupportedLanguage> {
        [
            BG, CS, DA, DE, EL, ESES, ENGB, ENUS, FI, FR, HI, HR, HU, IT, JA, KO, LT, NL, NO, PL,
            PTBR, RO, RU, SVSE, TH, TR, UK, VI, ZHCN, ZHTW,
        ]
        .iter()
        .copied()
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::BG => "bg",
            Self::CS => "cs",
            Self::DA => "da",
            Self::DE => "de",
            Self::EL => "el",
            Self::ESES => "es-ES",
            Self::ENGB => "en-GB",
            Self::ENUS => "en-US",
            Self::FI => "fi",
            Self::FR => "fr",
            Self::HI => "hi",
            Self::HR => "hr",
            Self::HU => "hu",
            Self::IT => "it",
            Self::JA => "ja",
            Self::KO => "ko",
            Self::LT => "lt",
            Self::NL => "nl",
            Self::NO => "no",
            Self::PL => "pl",
            Self::PTBR => "pt-BR",
            Self::RO => "ro",
            Self::RU => "ru",
            Self::SVSE => "sv-SE",
            Self::TH => "th",
            Self::TR => "tr",
            Self::UK => "uk",
            Self::VI => "vi",
            Self::ZHCN => "zh-CN",
            Self::ZHTW => "zh-TW",
        }
    }
}

pub fn try_get_lang_code(expected_lang_code: &str) -> Option<DiscordSupportedLanguage> {
    for lang_code in DiscordSupportedLanguage::iter() {
        if lang_code.to_str() == expected_lang_code {
            return Some(lang_code);
        }
    }

    None
}