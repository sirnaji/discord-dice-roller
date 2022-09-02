use self::DiscordSupportedLanguage::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum DiscordSupportedLanguage {
    BG,
    CS,
    DA,
    DE,
    EL,
    EsEs,
    EnGb,
    EnUs,
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
    PtBr,
    RO,
    RU,
    SvSe,
    TH,
    TR,
    UK,
    VI,
    ZhCn,
    ZhTw,
}

impl DiscordSupportedLanguage {
    pub fn iter() -> impl Iterator<Item = DiscordSupportedLanguage> {
        [
            BG, CS, DA, DE, EL, EsEs, EnGb, EnUs, FI, FR, HI, HR, HU, IT, JA, KO, LT, NL, NO, PL,
            PtBr, RO, RU, SvSe, TH, TR, UK, VI, ZhCn, ZhTw,
        ]
        .iter()
        .copied()
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Self::BG => "bg",
            Self::CS => "cs",
            Self::DA => "da",
            Self::DE => "de",
            Self::EL => "el",
            Self::EsEs => "es-ES",
            Self::EnGb => "en-GB",
            Self::EnUs => "en-US",
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
            Self::PtBr => "pt-BR",
            Self::RO => "ro",
            Self::RU => "ru",
            Self::SvSe => "sv-SE",
            Self::TH => "th",
            Self::TR => "tr",
            Self::UK => "uk",
            Self::VI => "vi",
            Self::ZhCn => "zh-CN",
            Self::ZhTw => "zh-TW",
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