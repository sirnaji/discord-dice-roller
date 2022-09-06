use super::locale::Locale;
use crate::utils::i18n::supported_language;
use colored::*;
use serde_json::Error;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::process;

pub fn load_locales() -> HashMap<String, Locale>
{
    let mut locales: HashMap<String, Locale> = HashMap::new();

    let path: PathBuf = PathBuf::from("locales");
    let files = get_files_in_dir(&path);

    println!("Loading locale files...");

    if files.is_empty()
    {
        println!(
            "{} No locale files found. The bot needs at least one locale file to load.",
            "ERROR ".bright_red()
        );
        process::exit(1);
    }

    for file in files
    {
        match fetch_locale_from_file(&file)
        {
            Ok(locale) =>
            {
                let file_name = file.file_stem().unwrap().to_str().unwrap();

                if let Some(lang_code) = supported_language::try_get_lang_code(file_name)
                {
                    println!("Loaded {}", format!("{}.json", file_name).yellow());
                    locales.insert(lang_code.to_str().to_string(), locale);
                }
                else
                {
                    println!(
                        "{} Lang code \"{}\" from the translation file is not supported.",
                        "ERROR ".bright_red(),
                        &file_name
                    );
                    process::exit(1);
                }
            }

            Err(err) =>
            {
                println!(
                    "{} Couldn't load \"{}\" : {}",
                    "ERROR ".bright_red(),
                    &file.file_name().unwrap().to_str().unwrap(),
                    err,
                );
                process::exit(1);
            }
        }
    }

    locales
}

fn get_files_in_dir(path: &PathBuf) -> Vec<PathBuf>
{
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(path).unwrap()
    {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file()
        {
            files.push(path);
        }
    }
    files
}

fn fetch_locale_from_file(path: &PathBuf) -> Result<Locale, Error>
{
    let file_name = path.file_name().unwrap().to_str().unwrap();

    if !file_name.ends_with(".json")
    {
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

    locale_from_file
}
