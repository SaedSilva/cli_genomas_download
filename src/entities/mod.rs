use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NCBIDatasetGenomeRequest {
    pub(crate) id: String,
    pub(crate) exclude_atypical: bool,
    pub(crate) assembly_level: HashSet<String>,
    pub(crate) first_release_date: String,
    pub(crate) last_release_date: String,
    pub(crate) next_page_token: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NBCIDatasetGenomeResponse {
    pub(crate) reports: Vec<Report>,
    pub(crate) next_page_token: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Report {
    pub(crate) accession: String,
    pub assembly_info: AssemblyInfo,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AssemblyInfo {
    biosample: Option<BioSample>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BioSample {
    sample_ids: Option<Vec<SampleId>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SampleId {
    db: Option<String>,
    value: Option<String>,
}
