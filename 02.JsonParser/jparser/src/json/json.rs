use std::collections::{HashMap, VecDeque};
use std::io::{Error, ErrorKind};

trait Json {
    fn build(tokens: &mut VecDeque<char>) -> Result<Box<Self>, Error>;
}

struct JsonKey {
    key: String,
}

impl Json for JsonKey {
    fn build(tokens: &mut VecDeque<char>) -> Result<Box<Self>, Error> {
        let mut key = String::new();

        let mut is_backslash = false;

        while let Some(c) = tokens.pop_front() {
            if c == '\\' && !is_backslash {
                is_backslash = true;
                continue;
            } else if c == '"' && !is_backslash {
                break;
            }
            key.push(c);
            is_backslash = false;
        }
        Ok(Box::new(JsonKey { key }))
    }
}

struct JsonValue<T: Json> {
    value: T,
}

impl<T: Json> Json for JsonValue<T> {
    fn build(mut tokens: VecDeque<char>) -> Result<Box<Self>, Error> {
        while let Some(c) = tokens.pop_front() {
            if c == '"' {}
        }
    }
}

struct JsonArray<T: Json> {
    array: Vec<T>,
}

impl<T: Json> Json for JsonArray<T> {
    fn build(mut tokens: VecDeque<char>) -> (Result<Box<Self>, Error>, VecDeque<char>) {
        let mut array = Vec::new();
        while let Some(c) = tokens.pop_front() {
            if c == ']' {
                break;
            }
            let (val, tokens) = T::build(tokens)?;
            array.push(val);
        }
        Ok((Ok(Box::new(JsonArray { array })), tokens))
    }
}

struct JSonObject<T: Json> {
    object: HashMap<String, T>,
}

impl<T: Json> Json for JSonObject<T> {
    fn build(mut tokens: VecDeque<char>) -> (Result<Box<Self>, Error>, VecDeque<char>) {
        let mut object = HashMap::new();
        while let Some(c) = tokens.pop_front() {
            if c == '}' {
                break;
            }
            let (key, tokens) = T::build(tokens)?;
            let (val, tokens) = T::build(tokens)?;
            object.insert(key, val);
        }
        Ok((Ok(Box::new(JSonObject { object })), tokens))
    }
}

pub enum JsonState {
    Key,
    Value,
    Array,
}

#[derive(Debug)]
pub struct JsonParser {
    tokens: VecDeque<String>,
    temp_json: Vec<char>,
    count_bracket: i32,
    count_brace: i32,
}

impl JsonParser {
    pub fn new() -> Self {
        Self {
            tokens: VecDeque::new(),
            temp_json: Vec::new(),
            count_bracket: 0,
            count_brace: 0,
        }
    }

    fn push(&mut self, s: String) {
        if s.is_empty() {
            return;
        } else if s == "{" {
            self.count_bracket += 1;
        } else if s == "}" {
            self.count_bracket -= 1;
        } else if s == "[" {
            self.count_brace += 1;
        } else if s == "]" {
            self.count_brace -= 1;
        }
        self.tokens.push_back(s);
    }

    fn push_and_clear(&mut self, s: String) {
        if !self.temp_json.is_empty() {
            self.push(self.temp_json.iter().collect::<String>());
            self.temp_json.clear();
        }
        self.push(s);
    }

    pub fn build_json_object(&mut self, content: String) {
        self.tokens.clear();
        self.temp_json.clear();
        self.count_bracket = 0;
        self.count_brace = 0;

        let mut is_backslash = false;
        let mut in_string = false;

        for ch in content.chars() {
            if is_backslash != false && ch == '"' {
                in_string = !in_string;
            }

            if is_backslash != false && ch == '\\' {
                is_backslash = false;
            } else {
                is_backslash = true;
            }

            if in_string {
                self.temp_json.push(ch);
                continue;
            } else if ch.is_whitespace() {
                continue;
            } else if matches!(ch, '{' | '}' | '[' | ']' | ':' | ',') {
                self.push_and_clear(ch.to_string());
            } else {
                self.temp_json.push(ch);
            }
        }
    }

    fn parser(&mut self) {}

    // pub fn is_valid(&mut self) -> bool {

    //     dbg!(&self.tokens);

    //     if self.count_brace != 0 || self.count_bracket != 0 {
    //         return false;
    //     } else if self.tokens.is_empty() {
    //         return false;
    //     }

    //     let mut is_valid = true;

    //     if self.tokens.front() == Some(&"[".to_string()){
    //         self.json_parser_state = JsonParserState::InitJsonArray;
    //     }

    //     while !self.tokens.is_empty() {

    //         let s = self.tokens.pop_front().unwrap();

    //         match self.json_parser_state {
    //             JsonParserState::InitJsonObject => {
    //                 if s != "{" {
    //                     is_valid = false;
    //                     break;
    //                 }else if self.tokens.front() == Some(&"}".to_string()) {
    //                     self.json_parser_state = JsonParserState::EndObject;
    //                 } else {
    //                     self.json_parser_state = JsonParserState::InKey;
    //                 }
    //             }
    //             JsonParserState::InKey => {
    //                 if !is_str(&s) {
    //                     is_valid = false;
    //                     break;
    //                 }
    //                 self.json_parser_state = JsonParserState::Separator;

    //             }
    //             JsonParserState::Separator => {
    //                 if s != ":" {
    //                     is_valid = false;
    //                     break;
    //                 }else if self.tokens.front() == Some(&"{".to_string()) {
    //                     self.json_parser_state = JsonParserState::InitJsonObject;
    //                 } else if self.tokens.front() == Some(&"[".to_string()) {
    //                     self.json_parser_state = JsonParserState::InitJsonArray;
    //                 } else {
    //                     self.json_parser_state = JsonParserState::InValue;
    //                 }
    //             }
    //             JsonParserState::InValue => {
    //                 if !is_value(&s) {
    //                     is_valid = false;
    //                     break;
    //                 }
    //                 if self.tokens.front() == Some(&",".to_string()) {
    //                     self.tokens.pop_front();
    //                     self.json_parser_state = JsonParserState::InKey;
    //                 }else {
    //                     self.json_parser_state = JsonParserState::EndObject;
    //                 }

    //             }
    //             JsonParserState::InitJsonArray => {
    //                 if s == "]" {
    //                     if self.tokens.front() == Some(&",".to_string()) {
    //                         self.tokens.pop_front();
    //                         self.json_parser_state = JsonParserState::InKey;
    //                     } else {
    //                         self.json_parser_state = JsonParserState::EndObject;
    //                     }
    //                 } else if self.tokens.front() == Some(&"{".to_string())  {
    //                     self.json_parser_state = JsonParserState::InitJsonObject;
    //                 } else if s == "[" {
    //                     continue;
    //                 }else if !is_value(&s) {
    //                     is_valid = false;
    //                     break;

    //                 }else if self.tokens.front() == Some(&",".to_string()) {
    //                     self.tokens.pop_front();
    //                 } else if self.tokens.front() == Some(&"]".to_string()) {
    //                     continue;
    //                 } else {
    //                     is_valid = false;
    //                     break;
    //                 }

    //             }

    //             JsonParserState::EndObject => {
    //                 if !(s == "}" || s == "]") {

    //                     is_valid = false;
    //                     break;
    //                 }
    //                 if self.tokens.front() == Some(&",".to_string()) && s == "}" {
    //                     self.tokens.pop_front();
    //                     self.json_parser_state = JsonParserState::InKey;
    //                 } else if  self.tokens.front() == Some(&",".to_string()) && s == "]" {
    //                     self.tokens.pop_front();
    //                     self.json_parser_state = JsonParserState::InitJsonArray;
    //                 }
    //             }
    //         }
    //     }
    //     is_valid
    // }
}

fn is_str(content: &str) -> bool {
    if !content.starts_with('"') || !content.ends_with('"') {
        return false;
    }
    let mut is_backslash = false;

    let mut count_four_chars = 0;

    for ch in content.chars() {
        if count_four_chars > 0 {
            if !ch.is_digit(16) {
                return false;
            }
            count_four_chars -= 1;
            continue;
        }

        if ch == '\\' {
            is_backslash = true;
            continue;
        }
        if is_backslash {
            is_backslash = false;
            if ch == 'u' {
                count_four_chars = 4;
            } else if !matches!(ch, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't') {
                return false;
            }
        }
    }

    return true;
}

fn is_bool(content: &str) -> bool {
    matches!(content, "true" | "false")
}

fn is_null(content: &str) -> bool {
    matches!(content, "null")
}

fn is_number(content: &str) -> bool {
    match content.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_value(content: &str) -> bool {
    if is_str(content) {
        true
    } else if is_bool(content) {
        true
    } else if is_null(content) {
        true
    } else if is_number(content) {
        true
    } else {
        false
    }
}
