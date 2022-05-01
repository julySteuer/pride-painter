use std::collections::HashMap;

use regex::{Regex, Captures};

pub fn parse_text(text: String){
    let vars = Regex::new(r"\w+=\{\d+,\d+,\d+\}").unwrap();
    let content = Regex::new(r"\[( ?\w+,?){1,}\]").unwrap(); // experimental regex \[(\[( ?\w+,?){1,}\],?\n?){1,}\]
    parse_variable(vars.captures_iter(text.as_str()).collect());
    parse_flag_content(content.captures_iter(text.as_str()).collect())
}

fn parse_variable(vars: Vec<Captures>) -> HashMap<String, (i32,i32,i32)>{
    let stack: HashMap<String,(i32,i32,i32)> = HashMap::new();
    let split_val = Regex::new(r"\d+").unwrap();
    for var in vars.into_iter() {
        let name_value:Vec<&str> = var[0].split("=").collect();
        let (name, value) = {
            (name_value[0], name_value[1])
        };
        let parsed_value = {
            let cap_i =  
            for cap in split_val.captures_iter(value) {
                
            }
        }; 
        // Make string to index and give it color value
        stack.entry(String::from(name)).or_insert(parsed_value);
    }
    stack
}

fn parse_flag_content(content: Vec<Captures>){
    
}
