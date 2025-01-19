use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;

pub fn remove_plasmidial(file_path: &str) -> Result<(), String> {
    if !file_path.ends_with(".fasta") {
        return Err(String::from("Arquivo não é .fasta"));
    }
    println!("Removendo plasmidial: {}", file_path);
    let file = File::open(file_path);
    match file {
        Ok(file) => {
            let mut count = 0;
            let reader = io::BufReader::new(file);
            let mut lines: Vec<String> = Vec::new();

            for line in reader.lines() {
                let line = line.expect("Falha ao ler linha");
                if line.starts_with(">") {
                    count += 1;
                }
                if count > 1 {
                    break;
                }
                lines.push(line);
            }
            let mut file = File::create(file_path).expect("Falha ao criar novo arquivo");
            for line in lines {
                writeln!(file, "{}", line).expect("Falha ao escrever linha");
            }

            Ok(())
        }
        Err(_) => Err(String::from("Falha")),
    }
}