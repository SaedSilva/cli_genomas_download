use reqwest::blocking::Client;
use reqwest::Error;
use crate::xml;

const BASE_URL_EUTILS: &'static str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";

pub fn search_entrez_assembly(genome_id: &str) -> String {
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

pub fn summary_entrez_assembly(genome_id: &str) -> String {
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

pub fn search_in_taxonomy(term: &str) -> Result<String, Error> {
    let url = format!("{}esearch.fcgi?db=taxonomy&term={}", BASE_URL_EUTILS, term);
    let http_client: Client = Client::new();
    let response = http_client.get(url).send()?.text()?;
    Ok(response)
}