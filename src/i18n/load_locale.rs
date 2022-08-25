use colored::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::process;

use crate::i18n::language;

use super::language::DiscordSupportedLanguage;
use super::locale::Locale;

pub fn load_locales() -> HashMap<DiscordSupportedLanguage, Locale> {
    let mut locales: HashMap<DiscordSupportedLanguage, Locale> = HashMap::new();

    let path: PathBuf = PathBuf::from("locales");
    let files = get_files_in_dir(&path);

    println!("Loading locale files...");

    if files.is_empty() {
        panic!("No locale found. Please add at least one locale file.")
    }

    for file in files {
        let json = fetch_locale_from_file(&file);

        if let Some(locale) = json {
            let file_name = file.file_stem().unwrap().to_str().unwrap();

            if let Some(lang_code) = language::try_get_lang_code(file_name) {
                println!("Loaded {}", format!("{}.json", file_name).yellow());
                locales.insert(lang_code, locale);
            } else {
                println!(
                    "{} Lang code \"{}\" from the translation file is not supported.",
                    "ERROR ".bright_red(),
                    &file_name
                );
                process::exit(1);
            }
        } else {
            println!(
                "{} Translation file \"{}\" is not matching locale format.",
                "ERROR ".bright_red(),
                &file.file_name().unwrap().to_str().unwrap()
            );
            process::exit(1);
        }
    }

    locales
}

fn get_files_in_dir(path: &PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    files
}

fn fetch_locale_from_file(path: &PathBuf) -> Option<Locale> {
    let file_name = path.file_name().unwrap().to_str().unwrap();

    if !file_name.ends_with(".json") {
        println!(
            "{} {} is not a json file.",
            "ERROR ".bright_red(),
            &file_name
        );
        process::exit(1);
    }

    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let locale_from_file: Result<Locale, serde_json::Error> = serde_json::from_reader(&mut reader);

    match locale_from_file {
        Ok(locale) => Some(locale),
        Err(_) => None,
    }
}
