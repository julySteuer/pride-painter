use std::collections::HashMap;

use regex::{Regex, Captures};
use crate::color_pallet::make_color_from_pallet; 

pub fn parse_text(text: String) -> Vec<Vec<i32>>{
    let vars = Regex::new(r"\w+=\{\d+,\d+,\d+\}").unwrap();
    let content = Regex::new(r"\[( ?\w+,?){1,}\]").unwrap(); // experimental regex \[(\[( ?\w+,?){1,}\],?\n?){1,}\]
    let stack = parse_variable(vars.captures_iter(text.as_str()).collect());
    let out = parse_flag_content(content.captures_iter(text.as_str()).collect(), stack);
    out
}

fn parse_variable(vars: Vec<Captures>) -> HashMap<String, Vec<i32>>{
    let mut stack: HashMap<String,Vec<i32>> = HashMap::new();
    let split_val = Regex::new(r"\d+").unwrap();
    for var in vars.into_iter() {
        let name_value:Vec<&str> = var[0].split("=").collect();
        let (name, value) = {
            (name_value[0], name_value[1])
        };
        let parsed_value = {
                let mut cap_data: Vec<i32> = Vec::new();
                let splitted = split_val.captures_iter(value);
                for split in splitted {
                    cap_data.push(split.get(0).unwrap().as_str().parse().unwrap());
                }
                cap_data
        }; 
        // Make string to index and give it color value
        stack.entry(String::from(name)).or_insert(parsed_value);
    }
    stack
}

fn parse_flag_content(content: Vec<Captures>, stack: HashMap<String, Vec<i32>>) -> Vec<Vec<i32>>{
    let mut col_vec: Vec<Vec<i32>> = Vec::new();
    let key_num = {
        let mut key_num: HashMap<String, i32> = HashMap::new();
        for (i, (k, v)) in stack.iter().enumerate() {
            make_color_from_pallet((i * 3) as i32, (v[0], v[1], v[2]));
            key_num.entry(k.clone()).or_insert((i * 3 + 3) as i32);
        }
        key_num
    };
    for i in 0..content.len() {
        let mut colors: Vec<i32> = Vec::new();
        let word_re = Regex::new(r"\w+").unwrap();
        let t = content[i].get(0).unwrap().as_str();
        let words = word_re.captures_iter(t); 
        for word in words {
            colors.push(key_num.get(word.get(0).unwrap().as_str()).unwrap().clone());
        }
        col_vec.push(colors);
    }
    col_vec
}
