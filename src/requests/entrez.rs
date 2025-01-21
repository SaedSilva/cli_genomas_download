use reqwest::blocking::Client;
use reqwest::Error;

const BASE_URL_EUTILS: &'static str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";

pub fn search_entrez_assembly(term: &str) -> String {
    search_entrez("assembly", term)
}

fn search_entrez(db: &str, term: &str) -> String {
    let url = format!("{}esearch.fcgi?db={}&term={}", BASE_URL_EUTILS, db, term);
    let http_client: Client = Client::new();
    let xml = http_client
        .get(url)
        .send()
        .expect("Falha ao fazer requisição")
        .text()
        .unwrap();
    xml
}

pub fn summary_entrez_assembly(genome_id: &str) -> String {
    summary_entrez("assembly", genome_id)
}

fn summary_entrez(db: &str, id: &str) -> String {
    let url = format!("{}esummary.fcgi?db={}&id={}", BASE_URL_EUTILS, db, id);
    let http_client: Client = Client::new();
    let xml = http_client
        .get(url)
        .send()
        .expect("Falha ao fazer requisição")
        .text()
        .unwrap();
    xml
}

pub fn search_in_taxonomy(term: &str) -> Result<String, Error> {
    let url = format!("{}esearch.fcgi?db=taxonomy&term={}", BASE_URL_EUTILS, term);
    let http_client: Client = Client::new();
    let response = http_client.get(url).send()?.text()?;
    Ok(response)
}

pub fn search_sra(term: &str) -> Result<String, Error> {
    let url = format!("{}esearch.fcgi?db=sra&term={}", BASE_URL_EUTILS, term);
    let http_client: Client = Client::new();
    let response = http_client.get(url).send()?.text()?;
    Ok(response)
}

pub fn summary_rsa(id: &str) -> Result<String, Error> {
    let url = format!("{}esummary.fcgi?db=sra&id={}", BASE_URL_EUTILS, id);
    let http_client: Client = Client::new();
    let response = http_client.get(url).send()?.text()?;
    Ok(response)
}
