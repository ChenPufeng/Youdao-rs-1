extern crate ansi_term;
extern crate reqwest;
extern crate scraper;
extern crate tempfile;

use self::scraper::Html;
use ansi_term::Colour;

use std::env;
use std::env::Args;
use std::io::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

mod app;
mod parser;
mod util;

use app::*;
use parser::*;

fn run() {
    if let Some(app) = parse_arguments(env::args()) {
        let mut body = reqwest::get(app.query_url().as_str())
            .unwrap()
            .text()
            .unwrap();
        let document = Html::parse_document(&body);

        if let Err(e) = parse_and_print(&document, &app.query_string(), app.words.len() > 1) {
            panic!(format!("{:?}", e));
        }

        let mut sentence_body = reqwest::get(app.query_sentence_url().as_str())
            .unwrap()
            .text()
            .unwrap();
        let sentence_document = Html::parse_document(&sentence_body);
        if let Err(e) = query_sentences(&sentence_document, app.is_more()) {
            panic!(format!("{:?}", e));
        }

        if app.is_voice() {
            let _ = play_sound(&app);
        }
    } else {
        display_usage();
    }
}

fn main() {
    run();
}

fn parse_arguments(args: Args) -> Option<App> {
    let mut args: Vec<_> = args.collect();
    if args.len() == 1 {
        return None;
    }
    let mut is_voice = false;
    let mut is_more = false;
    let mut words = Vec::new();
    let _ = args.remove(0);
    while let Some(arg) = args.pop() {
        if arg == "-v" {
            is_voice = true;
            continue;
        }
        if arg == "-m" {
            is_more = true;
            continue;
        }
        words.push(arg)
    }
    let app = App::new(words.into_iter().rev().collect(), is_voice, is_more);
    Some(app)
}

fn display_usage() {
    println!("{}", Colour::Blue.paint("Usage:"));
    println!(
        "{}",
        Colour::Blue.paint("YoudaoCLI <word(s) to query>        Query the word(s)")
    );
    println!(
        "{}",
        Colour::Blue.paint("YoudaoCLI <word(s) to query> -v     Query with speech")
    );
    println!(
        "{}",
        Colour::Blue.paint("YoudaoCLI <word(s) to query> -v     Query with speech")
    );
}

fn play_sound(app: &App) -> Result<(), String> {
    let mut voice_response = reqwest::get(app.voice_url().as_str()).map_err(|e| format!("{}", e))?;
    let mut buf: Vec<u8> = vec![];
    voice_response
        .copy_to(&mut buf)
        .map_err(|e| format!("{}", e))?;
    let mut file = NamedTempFile::new().map_err(|e| format!("{}", e))?;
    file.write_all(&buf).map_err(|e| format!("{}", e))?;
    let _ = Command::new("mpg123")
        .arg(file.path().as_os_str())
        .output()
        .map_err(|e| format!("{}", e))?;
    Ok(())
}
