extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs;
use std::io;
use std::collections::HashMap;

use super::arguments;



pub fn get_test_cases(args: &arguments::EvaluatorArgs) -> Result<HashMap<String, String>, io::Error> {
    lazy_static! {
        static ref FILE_REGEXES: Vec<(Regex, Regex)> = vec![
            (Regex::new(r"(?P<i>([a-z]|[A-Z]|\d)+).in$").unwrap(), Regex::new(r"(?P<i>([a-z]|[A-Z]|\d)+).out$").unwrap()),
            (Regex::new(r"in.(?P<i>([a-z]|[A-Z]|\d)+)$").unwrap(), Regex::new(r"out.(?P<i>([a-z]|[A-Z]|\d)+)$").unwrap())
        ];
    }

    let mut input_output_map: HashMap<String, String> = HashMap::new();
    if args.io_directory.len() > 0 {
        let entries = fs::read_dir(args.io_directory)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        
        let mut entries: Vec<String> = entries.iter()
            .filter(|p| p.is_file())
            .map(|p| p.to_str().unwrap().to_string())
            .collect();
        
        entries.sort();
        for (in_re, out_re) in FILE_REGEXES.iter() {
            let mut identifier_map: HashMap<String, String> = HashMap::new();
            let valid_entries: Vec<String> = entries
                .clone()
                .into_iter()
                .filter(|e| in_re.is_match(e)).collect();
            
            for e in valid_entries.into_iter() {
                match in_re.captures(&e) {
                    Some(c) => {
                        identifier_map.insert(c.name("i").unwrap().as_str().to_string(), e);
                    },
                    None => {}
                }
            }

            let output_entries: Vec<String> = entries
                .clone()
                .into_iter()
                .filter(|e| out_re.is_match(e)).collect();
            
            for e in output_entries.into_iter() {
                match out_re.captures(&e) {
                    Some(c) => {
                        let id = c.name("i").unwrap().as_str().to_string();
                        match identifier_map.get(&id) {
                            Some(input) => {input_output_map.insert(input.to_string(), e);},
                            None => {}
                        }
                    },
                    None => {}
                }
            }
        }
        return Ok(input_output_map);
    } else {
        input_output_map.insert(args.input_file_path.to_string(), args.correct_output_file_path.to_string());
        return Ok(input_output_map);
    }
}