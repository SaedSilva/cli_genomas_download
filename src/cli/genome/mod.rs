mod search_genome;
mod tsv_genome;
mod tsv_genome_fastq;

use crate::cli::genome::search_genome::search_genome;
use std::io;

pub fn init() {
    loop {
        let mode = input_mode();
        if mode == "0" {
            break;
        }
        if mode == "1" {
            tsv_genome::tsv_genome().unwrap();
        }
        if mode == "2" {
            search_genome().unwrap();
        }
        if mode == "3" {
            tsv_genome_fastq::tsv_genome_fastq().unwrap();
        }
    }
}

fn input_mode() -> String {
    let mut input = String::new();
    println!("O que voce deseja fazer?");
    println!("0 - Sair");
    println!("1 - Baixar genomas de um arquivo TSV");
    println!("2 - Pesquisar e baixar por um genoma");
    println!("3 - Baixar genomas e fastq de um arquivo TSV");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
