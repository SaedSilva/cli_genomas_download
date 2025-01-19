use crate::entities::{NBCIDatasetGenomeResponse, NCBIDatasetGenomeRequest};
use reqwest::blocking::Client;
use reqwest::Error;

const BASE_URL_NCBI: &'static str = "https://api.ncbi.nlm.nih.gov/datasets/v2/";

pub fn get_ncbi_dataset(
    request: &NCBIDatasetGenomeRequest,
) -> Result<NBCIDatasetGenomeResponse, Error> {
    let vc = request
        .assembly_level
        .clone()
        .into_iter()
        .collect::<Vec<String>>();
    let assembly_level = vc.join(",");
    let url = format!("{}genome/taxon/{}/dataset_report?filters.exclude_atypical={}&filters.assembly_level={}&filters.first_release_date={}&page_token={}&page_size=100", BASE_URL_NCBI, request.id, request.exclude_atypical, assembly_level, request.first_release_date, request.next_page_token);
    let http_client: Client = Client::new();
    let response: NBCIDatasetGenomeResponse = http_client.get(url).send()?.json()?;
    Ok(response)
}
