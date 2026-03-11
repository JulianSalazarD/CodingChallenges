use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum JsonParserState {
    InitJsonObject,
    InitJsonArray,
    InKey,
    InValue,
    Separator,
    EndObject,
}

#[derive(Debug)]
pub struct JsonObject {
    object: VecDeque<String>,
    temp_json: Vec<char>,
    count_bracket: usize,
    count_brace: usize,
    json_parser_state: JsonParserState,
}

impl JsonObject {
    pub fn new() -> Self {
        Self {
            object: VecDeque::new(),
            temp_json: Vec::new(),
            count_bracket: 0,
            count_brace: 0,
            json_parser_state: JsonParserState::InitJsonObject,
        }
    }

    fn push(&mut self, s: String) {
        if s.is_empty() {
            return;
        }else if s == "{" {
            self.count_brace += 1;
        } else if s == "}" {
            self.count_brace -= 1;
        } else if s == "[" {
            self.count_bracket += 1;
        } else if s == "]" {
            self.count_bracket -= 1;
        }
        self.object.push_back(s);
    }

    fn push_and_clear(&mut self, s: String) {
        if !self.temp_json.is_empty() {
            self.push(self.temp_json.iter().collect::<String>());
            self.temp_json.clear();
        }
        self.push(s);
    }

    pub fn build_json_object(&mut self, content: String) {
        self.object.clear();
        self.temp_json.clear();
        self.count_bracket = 0;
        self.count_brace = 0;

        for ch in content.chars() {
            if ch.is_whitespace(){
                continue;
            } else if ch == '{' || ch == '[' {
                self.push_and_clear(ch.to_string());
            } else if ch == '}' || ch == ']' {
                self.push_and_clear(ch.to_string());
            } else if ch == ':' {
                self.push_and_clear(ch.to_string());
            } else if ch == ',' {
                self.push_and_clear(ch.to_string());
            } 
            else {
                self.temp_json.push(ch);
            }
        }
    }

    pub fn is_valid(&mut self) -> bool {
        if self.count_brace != 0 || self.count_bracket != 0 {
            return false;
        } else if self.object.is_empty() {
            return false;
        }

        let mut is_valid = true;


        while !self.object.is_empty() {

            let s = self.object.pop_front().unwrap(); 

            match self.json_parser_state {
                JsonParserState::InitJsonObject => {
                    if s != "{" {
                        is_valid = false;
                        break;
                    }else if self.object.front() == Some(&"}".to_string()) {
                        self.json_parser_state = JsonParserState::EndObject;
                    } else {
                        self.json_parser_state = JsonParserState::InKey;
                    }
                }
                JsonParserState::InKey => {
                    if !is_str(&s) {
                        is_valid = false;
                        break;
                    }
                    self.json_parser_state = JsonParserState::Separator;
                    
                }
                JsonParserState::Separator => {
                    if s != ":" {
                        is_valid = false;
                        break;
                    }else if self.object.front() == Some(&"{".to_string()) {
                        self.json_parser_state = JsonParserState::InitJsonObject;
                    } else if self.object.front() == Some(&"[".to_string()) {
                        self.json_parser_state = JsonParserState::InitJsonArray;
                    } else {
                        self.json_parser_state = JsonParserState::InValue;
                    }
                }
                JsonParserState::InValue => {
                    if !is_value(&s) {
                        is_valid = false;
                        break;
                    }
                    if self.object.front() == Some(&",".to_string()) {
                        self.object.pop_front();
                        self.json_parser_state = JsonParserState::InKey;
                    }else {
                        self.json_parser_state = JsonParserState::EndObject;
                    }
                    
                }
                JsonParserState::InitJsonArray => {
                    if s ==  "[" {
                        continue;
                    }
                    if s == "]" {
                        if self.object.front() == Some(&",".to_string()) {
                            self.object.pop_front();
                            self.json_parser_state = JsonParserState::InKey;
                        } else {
                            self.json_parser_state = JsonParserState::EndObject;
                        }
                    } else if s == "{" {
                        self.json_parser_state = JsonParserState::InitJsonObject;
                    } else if s == "[" {
                        self.json_parser_state = JsonParserState::InitJsonArray;
                    } else if !is_value(&s) {
                        is_valid = false;
                        break;

                    }else if self.object.front() == Some(&",".to_string()) {
                        self.object.pop_front();
                    } else if self.object.front() == Some(&"]".to_string()) {
                        continue;
                    } else {
                        is_valid = false;
                        break;
                    }
                    
                }

                JsonParserState::EndObject => {
                    if s != "}" {
                        is_valid = false;
                        break;
                    }
                    if self.object.front() == Some(&",".to_string()) {
                        self.object.pop_front();
                        self.json_parser_state = JsonParserState::InKey;
                    } 
                }
            }
        }        

        is_valid
    }

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






