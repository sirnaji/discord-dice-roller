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

pub fn try_get_lang_code(expected_lang_code: &str) -> Option<DiscordSupportedLanguage> {
    for lang_code in DiscordSupportedLanguage::iter() {
        if lang_code.to_str() == expected_lang_code {
            return Some(lang_code);
        }
    }

    None
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

    pub fn get_name(&self) -> &str {
        match self {
            Self::BG => "Bulgarian",
            Self::CS => "Czech",
            Self::DA => "Danish",
            Self::DE => "German",
            Self::EL => "Greek",
            Self::ESES => "Spanish",
            Self::ENGB => "English, UK",
            Self::ENUS => "English, US",
            Self::FI => "Finnish",
            Self::FR => "French",
            Self::HI => "Hindi",
            Self::HR => "Croatian",
            Self::HU => "Hungarian",
            Self::IT => "Italian",
            Self::JA => "Japanese",
            Self::KO => "Korean",
            Self::LT => "Lithuanian",
            Self::NL => "Dutch",
            Self::NO => "Norwegian",
            Self::PL => "Polish",
            Self::PTBR => "Portuguese, Brazilian",
            Self::RO => "Romanian, Romania",
            Self::RU => "Russian",
            Self::SVSE => "Swedish",
            Self::TH => "Thai",
            Self::TR => "Turkish",
            Self::UK => "Ukrainian",
            Self::VI => "Vietnamese",
            Self::ZHCN => "Chinese, China",
            Self::ZHTW => "Chinese, Taiwan",
        }
    }

    pub fn get_native_name(&self) -> &str {
        match self {
            Self::BG => "български",
            Self::CS => "Čeština",
            Self::DA => "Dansk",
            Self::DE => "Deutsch",
            Self::EL => "Ελληνικά",
            Self::ESES => "Español",
            Self::ENGB => "English, UK",
            Self::ENUS => "English, US",
            Self::FI => "Suomi",
            Self::FR => "Français",
            Self::HI => "हिन्दी",
            Self::HR => "Hrvatski",
            Self::HU => "Magyar",
            Self::IT => "Italiano",
            Self::JA => "日本語",
            Self::KO => "한국어",
            Self::LT => "Lietuviškai",
            Self::NL => "Nederlands",
            Self::NO => "Norsk",
            Self::PL => "Polski",
            Self::PTBR => "Português do Brasil",
            Self::RO => "Română",
            Self::RU => "Pусский",
            Self::SVSE => "Svenska",
            Self::TH => "ไทย",
            Self::TR => "Türkçe",
            Self::UK => "Українська",
            Self::VI => "Tiếng Việt",
            Self::ZHCN => "中文",
            Self::ZHTW => "繁體中文",
        }
    }

    pub fn get_flag_emoji(&self) -> &str {
        match self {
            Self::BG => ":flag_bg:",
            Self::CS => ":flag_cz:",
            Self::DA => ":flag_dk:",
            Self::DE => ":flag_de:",
            Self::EL => ":flag_gr:",
            Self::ESES => ":flag_es:",
            Self::ENGB => ":flag_gb:",
            Self::ENUS => ":flag_us:",
            Self::FI => ":flag_fi:",
            Self::FR => ":flag_fr:",
            Self::HI => ":flag_in:",
            Self::HR => ":flag_cr:",
            Self::HU => ":flag_hu:",
            Self::IT => ":flag_it:",
            Self::JA => ":flag_jp:",
            Self::KO => ":flag_kr:",
            Self::LT => ":flag_lt:",
            Self::NL => ":flag_nl:",
            Self::NO => ":flag_no:",
            Self::PL => ":flag_pl:",
            Self::PTBR => ":flag_br:",
            Self::RO => ":flag_ro:",
            Self::RU => ":flag_ru:",
            Self::SVSE => ":flag_sv:",
            Self::TH => ":flag_th:",
            Self::TR => ":flag_tr:",
            Self::UK => ":flag_ua:",
            Self::VI => ":flag_vn:",
            Self::ZHCN => ":flag_cn:",
            Self::ZHTW => ":flag_tw:",
        }
    }
}
