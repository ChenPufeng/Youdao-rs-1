extern crate ansi_term;
extern crate itertools;
extern crate reqwest;
extern crate scraper;

use self::itertools::join;
use self::scraper::element_ref::Select;
use self::scraper::{Html, Selector};
use ansi_term::Colour;
use util::*;


pub enum ParseError {
    
}

pub fn parse_and_print(fragment: &Html, query: &str, is_multi: bool) -> Result<(), Box<::std::error::Error>> {
    if is_chinese(query) {
        basic_query_chn(&fragment)?;
    } else {
        if hint_eng(&fragment, query) {
            return Ok(());
        }
        basic_query_eng(&fragment, is_multi)?;
    }
    Ok(())
}

pub fn query_sentences(fragment: &Html, is_more: bool) {
    let sentences_selector = Selector::parse("div#bilingual > ul > li").unwrap();
    let sentences = fragment.select(&sentences_selector);
    let mut count = 1;
    println!("");
    for s in sentences {
        let p_selector = Selector::parse("p").unwrap();
        let ps = s.select(&p_selector);
        for (i, p) in ps.enumerate() {
            if i == 0 {
                print!(
                    "{}",
                    Colour::Green.paint(format!(
                        "  {}. {}",
                        count,
                        join(&p.text().collect::<Vec<_>>(), "").trim()
                    ))
                )
            }
            if i == 1 {
                print!(
                    "     {}",
                    Colour::Blue.paint(join(&p.text().collect::<Vec<_>>(), "").trim())
                );
                count += 1
            }
            println!("");
            if count == 4 && !is_more {
                return;
            }
        }
    }
}

fn hint_eng(fragment: &Html, query: &str) -> bool {
    let typo_selector = Selector::parse(".typo-rel").unwrap();
    let typos = fragment.select(&typo_selector).collect::<Vec<_>>();
    if typos.len() == 0 {
        return false;
    }

    println!("");
    println!(
        "{}",
        Colour::Blue.paint(format!("     word(s) '{}' not found, do you mean?", query))
    );
    println!("");

    for t in typos {
        let word_selector = Selector::parse("a").unwrap();
        let words = match t.select(&word_selector).next() {
            Some(w) => w,
            None => return false,
        };
        println!("     {}", Colour::Green.paint(words.text().next().unwrap()));
        println!(
            "     {}",
            Colour::Yellow.paint(t.text().last().unwrap().trim())
        );
    }
    true
}

fn basic_query_chn(fragment: &Html) -> Result<(), Box<::std::error::Error>> {
    println!("");
    let chn_selector = Selector::parse(".trans-container > ul > p").unwrap();
    let chn = fragment.select(&chn_selector);
    let mut meanings = Vec::new();
    for c in chn {
        let search_seletor = Selector::parse(".contentTitle > .search-js").unwrap();
        let search_content = c.select(&search_seletor);
        for s in search_content {
            let t = s.text();
            t.for_each(|s| meanings.push(s));
        }
        print!(
            "       {}",
            Colour::Blue.paint(c.text().skip(1).next().unwrap().trim())
        );
        let joined_meaning = join(&meanings, ";");
        print!("  {}", Colour::Yellow.paint(joined_meaning));
        
    }

    Ok(())
}

fn basic_query_eng(fragment: &Html, is_multi: bool) -> Result<(), Box<::std::error::Error>> {
    println!("");
    if !is_multi {
        let pronounce_selector = Selector::parse("div.baav > span.pronounce").unwrap();
        let pronounce = fragment.select(&pronounce_selector);
        for (i, n) in pronounce.enumerate() {
            let phonetic_selector = Selector::parse("span.phonetic").unwrap();
            let phonetic = n.select(&phonetic_selector);
            if i == 0 {
                print!("    {} ", Colour::Yellow.bold().paint("英："));
                pronounce_output_select(phonetic);
            } else {
                print!("{} ", Colour::Blue.bold().paint("美："));
                pronounce_output_select(phonetic);
            }
        }
    }

    println!("");
    println!("");
    //means
    let means_selector = Selector::parse("div#phrsListTab > div.trans-container > ul").unwrap();
    let means = fragment.select(&means_selector);
    for m in means {
        println!(
            "  {}",
            Colour::Blue.paint(join(&m.text().collect::<Vec<_>>(), ""))
        );
    }
    Ok(())
}

fn pronounce_output_select(select: Select) {

    let t_vec = select
        .collect::<Vec<scraper::ElementRef>>()
        .first()
        .unwrap()
        .text()
        .collect::<Vec<_>>();

    let text = t_vec.first().unwrap();

    print!("{}   ", Colour::Blue.paint(*text));
}
