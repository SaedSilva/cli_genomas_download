use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_assembly_accension(file_path: &str) -> Vec<String> {
    let mut accensions: Vec<String> = Vec::new();
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let accension = line.split("\t").collect::<Vec<&str>>()[0].to_string();
        if accension != "Assembly Accession" {
            accensions.push(String::from(accension.trim()));
        }
    }
    accensions
}