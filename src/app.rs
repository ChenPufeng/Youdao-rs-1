extern crate percent_encoding;

use self::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use std::fmt;
use util::*;

#[derive(Debug, PartialEq)]
pub struct App {
    pub words: Vec<String>,
    config: Config,
}

impl App {
    pub fn new(words: Vec<String>, is_voice: bool, is_more: bool) -> Self {
        App {
            words,
            config: Config { is_voice, is_more },
        }
    }

    pub fn query_string(&self) -> String {
        self.words.join(" ")
    }

    pub fn query_sentence_url(&self) -> String {
        format!(
            "{}{}",
            String::from("http://dict.youdao.com/example/blng/eng/"),
            utf8_percent_encode(&self.words.join("_")[..], DEFAULT_ENCODE_SET).to_string()
        )
    }

    pub fn query_url(&self) -> String {
        if is_chinese(&self.words.concat()[..]) {
            format!(
                "{}{}",
                String::from("http://dict.youdao.com/w/eng/"),
                utf8_percent_encode(&self.words.join(" ")[..], DEFAULT_ENCODE_SET).to_string()
            )
        } else {
            format!(
                "{}{}",
                String::from("http://dict.youdao.com/w/"),
                utf8_percent_encode(&self.words.join(" ")[..], DEFAULT_ENCODE_SET).to_string()
            )
        }
    }

    pub fn voice_url(&self) -> String {
        format!(
            "{}{}{}",
            String::from("https://dict.youdao.com/dictvoice?audio="),
            utf8_percent_encode(&self.words.join("+")[..], DEFAULT_ENCODE_SET).to_string(),
            String::from("&type=2")
        )
    }

    pub fn is_voice(&self) -> bool {
        self.config.is_voice
    }

    pub fn is_more(&self) -> bool {
        self.config.is_more
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "query words: {:?}, isMore: {}, isVoice: {}",
            self.words, self.config.is_more, self.config.is_voice,
        )
    }
}

#[derive(Debug, PartialEq)]
struct Config {
    is_voice: bool,
    is_more: bool,
}

#[test]
fn test_url_encoded() {
    let input = "hello world";
    let output = utf8_percent_encode(input, DEFAULT_ENCODE_SET).to_string();
    assert_eq!(output, String::from("hello%20world"));
}
