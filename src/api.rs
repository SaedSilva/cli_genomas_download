use reqwest::blocking::Client;
use reqwest::Error;
use std::collections::HashSet;
use std::fs::{remove_file, File};
use std::io;
use std::io::{BufRead, Write};
use std::thread::sleep;
use std::time::Duration;

const BASE_URL_EUTILS: &'static str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";
const BASE_URL_NCBI: &'static str = "https://api.ncbi.nlm.nih.gov/datasets/v2/";

use crate::download::download_and_save_file;
use crate::file::unzip;
use crate::{download, xml};
use serde::{Deserialize, Serialize};

pub fn search_in_taxonomy(term: &str) -> Result<String, Error> {
    let url = format!("{}esearch.fcgi?db=taxonomy&term={}", BASE_URL_EUTILS, term);
    let http_client: Client = Client::new();
    let response = http_client.get(url).send()?.text()?;
    Ok(response)
}

pub fn get_list_of_responses(request: &NBCIRequest) -> Result<NBCIResponse, Error> {
    let vc = request
        .assembly_level
        .clone()
        .into_iter()
        .collect::<Vec<String>>();
    let assembly_level = vc.join(",");
    let url = format!("{}genome/taxon/{}/dataset_report?filters.exclude_atypical={}&filters.assembly_level={}&filters.first_release_date={}&page_token={}&page_size=100", BASE_URL_NCBI, request.id, request.exclude_atypical, assembly_level, request.first_release_date, request.next_page_token);
    let http_client: Client = Client::new();
    let response: NBCIResponse = http_client.get(url).send()?.json()?;
    Ok(response)
}

fn download_genome_save_unzip(
    assembly: &str,
    folder_name: &str,
) -> Result<String, download::Error> {
    let id = search_entrez_assembly(assembly);
    let url = summary_entrez_assembly(&id);

    let file_path = download_and_save_file(url.as_str(), folder_name, assembly)?;

    println!("Deszipando: {}", file_path);
    let file_out_path = unzip(file_path.as_str());

    println!("deletando zip: {}", file_path);
    remove_file(file_path).expect("Falha ao excluir zip");

    remove_plasmidial(file_out_path.as_str()).expect("Falha ao reescrever arquivo");

    Ok("Salvo com sucesso!".to_string())
}

fn remove_plasmidial(file_path: &str) -> Result<(), String> {
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

pub fn download_genomes(
    genomes: Vec<String>,
    folder_name: &str,
) -> Result<String, download::Error> {
    let total = genomes.len();
    let mut count = 1;
    for genome in genomes {
        println!("{}/{} - Baixando genoma: {}", count, total, genome);
        download_genome_save_unzip(&genome, folder_name)?;
        println!();
        sleep(Duration::from_secs(1));
        count += 1;
    }
    println!("Todos os genomas foram baixados, extraidos e salvos com sucesso!");
    Ok("Salvos com sucesso".to_string())
}

pub fn get_list_string_of_name(request: &mut NBCIRequest, value: &str, total: usize) -> Vec<String> {
    let mut result: HashSet<String> = HashSet::new();
    loop {
        match get_list_of_responses(&request) {
            Ok(response) => {
                for item in response.reports {
                    if item.accession.starts_with(value) {
                        result.insert(item.accession);
                    }
                    if result.len() >= total {
                        break;
                    }
                }
                match response.next_page_token {
                    None => {
                        break;
                    }
                    Some(token) => request.next_page_token = token,
                }
            }
            Err(error) => {
                println!("{}", error);
                break;
            }
        }
        if result.len() >= total {
            break;
        }
        sleep(Duration::from_millis(250))
    }
    result.into_iter().collect::<Vec<_>>()
}

fn search_entrez_assembly(genome_id: &str) -> String {
    let url = format!(
        "{}esearch.fcgi?db=assembly&term={}",
        BASE_URL_EUTILS, genome_id
    );
    let http_client: Client = Client::new();
    let response = http_client
        .get(url)
        .send()
        .expect("Falha ao fazer requisição")
        .text()
        .unwrap();
    xml::get_id_list_from_xml(response).expect("Falha ao obter id")
}

fn summary_entrez_assembly(genome_id: &str) -> String {
    let url = format!(
        "{}esummary.fcgi?db=assembly&id={}",
        BASE_URL_EUTILS, genome_id
    );
    let http_client: Client = Client::new();
    let response = http_client
        .get(url)
        .send()
        .expect("Falha ao fazer requisição")
        .text()
        .unwrap();
    xml::get_download_link_from_xml(response).unwrap_or_else(|error| {
        println!("{}", error);
        String::new()
    })
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NBCIRequest {
    pub(crate) id: String,
    pub(crate) exclude_atypical: bool,
    pub(crate) assembly_level: HashSet<String>,
    pub(crate) first_release_date: String,
    pub(crate) last_release_date: String,
    next_page_token: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NBCIResponse {
    reports: Vec<Accession>,
    next_page_token: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Accession {
    accession: String,
}

#[derive(Default, Serialize, Deserialize)]
struct Accessions {
    pub accessions: Vec<String>,
}
