use reqwest::blocking::Client;
use reqwest::Error;
use std::collections::HashSet;
use std::fs::{create_dir_all, remove_file, File};
use std::io::{BufRead, Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{env, io};

const BASE_URL_EUTILS: &'static str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";
const BASE_URL_NCBI: &'static str = "https://api.ncbi.nlm.nih.gov/datasets/v2/";

use crate::file::Unzipper;
use crate::utils;
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

fn download_genome_save_unzip(assembly: &str, folder_name: &str) -> Result<String, Error> {
    let id = e_search(assembly);
    let url = e_summary(&*id);
    println!("Downloading from: {}", url);
    let http_client: Client = Client::new();

    let response = http_client.get(url).send()?;
    let bytes = response.bytes()?;

    let exe_path = env::current_exe().expect("Não foi possível obter o diretório do executável");
    let exe_dir = exe_path
        .parent()
        .expect("Não foi possível obter o diretório do executável");

    let download_dir = exe_dir.join(format!("downloads/{}", folder_name));
    let file_path = download_dir.join(format!("{}.fasta.gz", assembly));

    create_dir_all(&download_dir).expect("Falha ao criar diretório para downloads");

    let mut file_result = File::create(&file_path).expect("Falha ao salvar arquivo");

    file_result
        .write_all(&bytes)
        .expect("Falha ao salvar arquivo");

    unzip_and_remove_plasmidial(file_path.to_str().expect("Falha ao parsear"));

    Ok("Salvo com sucesso!".to_string())
}

fn unzip_and_remove_plasmidial(file_path: &str) {
    println!("Deszipando: {}", file_path);

    let mut unzipper = Unzipper::new(file_path);
    unzipper.unzip();
    let file_out_path = &unzipper.output_file.unwrap();

    remove_plasmidial(file_out_path).expect("Falha ao reescrever arquivo");

    println!("deletando zip: {}", file_path);
    remove_file(file_path).expect("Falha ao excluir zip");
}

fn remove_plasmidial(file_path: &str) -> Result<&str, &str> {
    if !file_path.ends_with(".fasta") {
        return Err("Arquivo não é .fasta");
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

            Ok("Reescrito com sucesso")
        }
        Err(_) => Err("Falha"),
    }
}

pub fn download_genomes(genomes: Vec<String>, folder_name: &str) -> Result<String, Error> {
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

pub fn get_list_string_of_name(mut request: NBCIRequest, value: &str, total: usize) -> Vec<String> {
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
                eprintln!("{}", error);
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

fn e_search(genome_id: &str) -> String {
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
        .expect("Falha ao sei la");
    utils::get_id_list_from_xml(response).expect("Falha ao obter id")
}

fn e_summary(genome_id: &str) -> String {
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
        .expect("Falha ao sei la");
    utils::get_download_link_from_xml(response).unwrap_or_else(|error| {
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
