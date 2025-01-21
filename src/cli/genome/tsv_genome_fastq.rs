use std::thread::sleep;
use std::time::Duration;
use crate::cli::genome::search_genome::input_filter_bank;
use crate::cli::genome::tsv_genome::{input_folder_name, input_tsv_path};
use crate::requests::entrez::{
    search_entrez_assembly, search_sra, summary_entrez_assembly, summary_rsa,
};
use crate::xml::{
    get_bioproject_accn_from_xml, get_count_from_xml, get_id_list_from_xml, get_run_from_xml,
};
use crate::{download, tsv};

pub fn tsv_genome_fastq() -> Result<(), download::Error> {
    let tsv_path = input_tsv_path();
    let folder_name = input_folder_name();
    let filter_bank = input_filter_bank();

    let accensions: Vec<String> = tsv::get_assembly_accension(&tsv_path)
        .into_iter()
        .filter(|accension| accension.starts_with(&filter_bank))
        .collect();
    println!("Accensions: {:?}", accensions);

    let mut ids = vec![];
    for accension in accensions {
        let xml = search_entrez_assembly(&accension);
        let id = get_id_list_from_xml(xml).unwrap();
        ids.push(id);
        sleep(Duration::from_millis(250));
    }
    println!("Ids: {:?}", ids);

    let mut bioproject_accn = vec![];
    for id in ids {
        let xml = summary_entrez_assembly(&id);
        let bioproject = get_bioproject_accn_from_xml(xml);
        match bioproject {
            Ok(bioproject) => bioproject_accn.push(bioproject),
            Err(_) => (),
        }
        sleep(Duration::from_millis(500));
    }
    println!("Bioprojects: {:?}", bioproject_accn);

    let mut ids = vec![];
    for bioproject in bioproject_accn {
        let xml = search_sra(&bioproject)?;
        match get_count_from_xml(xml.clone()) {
            Ok(count) => {
                if count == 0 {
                    continue;
                }
                let id = get_id_list_from_xml(xml).unwrap();
                ids.push(id);
            }
            Err(_) => ()
        }
        sleep(Duration::from_millis(250));
    }
    println!("Ids: {:?}", ids);

    let mut runs = vec![];
    for id in ids {
        let xml = summary_rsa(&id)?;
        let run = get_run_from_xml(xml).unwrap();
        runs.push(run);
        sleep(Duration::from_millis(250));
    }
    println!("Runs: {:?}", runs);

    Ok(())
}
