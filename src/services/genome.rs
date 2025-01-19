use std::collections::HashSet;
use std::fs::{remove_file, File};
use std::io;
use std::io::Write;
use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;
use crate::{download, xml};
use crate::download::download_and_save_fasta_gz;
use crate::entities::NCBIDatasetGenomeRequest;
use crate::file::unzip;
use crate::requests::entrez::{search_entrez_assembly, summary_entrez_assembly};
use crate::requests::ncbidataset::get_ncbi_dataset;

fn process_genome_file(assembly: &str, folder_name: &str) -> Result<String, download::Error> {
    let xml = search_entrez_assembly(assembly);
    let id = xml::get_id_list_from_xml(xml).unwrap();
    let xml = summary_entrez_assembly(&id);
    let url = xml::get_download_link_from_xml(xml).unwrap();
    let file_path = download_and_save_fasta_gz(url.as_str(), folder_name, assembly)?;

    match unzip_and_process_file(file_path) {
        Ok(file_out_path) => {
            remove_plasmidial_from_file(file_out_path.as_str()).unwrap();
            Ok("Salvo com sucesso!".to_string())
        }
        Err(e) => Err(e),
    }
}

fn unzip_and_process_file(file_path: String) -> Result<String, download::Error> {
    println!("Deszipando: {}", file_path);
    let file_out_path = unzip(file_path.as_str());

    println!("Deletando zip: {}", file_path);
    remove_file(&file_path).expect("Falha ao excluir zip");

    Ok(file_out_path)
}

pub fn download_genomes_save_unzip_delete_remove_plasmidial(
    genomes: Vec<String>,
    folder_name: &str,
) -> Result<String, download::Error> {
    let total = genomes.len();
    let mut count = 0;

    for genome in genomes {
        println!("{}/{} - Baixando genoma: {}", count + 1, total, genome);

        match process_genome_file(&genome, folder_name) {
            Ok(_) => count += 1,
            Err(_) => println!("Falha ao processar o genoma: {}", genome),
        }

        println!();
        sleep(Duration::from_secs(1));
    }

    if count == total {
        println!("Todos os genomas foram baixados, extraídos e salvos com sucesso!");
    } else {
        println!("Alguns genomas falharam ao serem baixados, extraídos e salvos!");
        println!("Total de genomas baixados: {}", count);
    }

    Ok("Salvos com sucesso".to_string())
}

pub fn get_list_string_genomes_bank(
    request: &mut NCBIDatasetGenomeRequest,
    filter: &str,
    total: usize,
) -> Vec<String> {
    let mut result: HashSet<String> = HashSet::new();

    loop {
        match get_ncbi_dataset(request) {
            Ok(response) => {
                for item in response.reports {
                    if item.accession.starts_with(filter) {
                        result.insert(item.accession);
                    }
                    if result.len() >= total {
                        break;
                    }
                }

                if result.len() >= total || response.next_page_token.is_none() {
                    break;
                }

                request.next_page_token = response.next_page_token.unwrap();
            }
            Err(error) => {
                eprintln!("Erro ao consultar o NCBI Dataset: {}", error);
                break;
            }
        }
        if result.len() >= total {
            break;
        }
        sleep(Duration::from_millis(250));
    }

    result.into_iter().collect::<Vec<_>>()
}

pub fn remove_plasmidial_from_file(file_path: &str) -> Result<(), String> {
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