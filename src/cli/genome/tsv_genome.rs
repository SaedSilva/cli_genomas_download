use crate::cli::genome::search_genome::input_filter_bank;
use crate::services::genome::download_genomes_save_unzip_delete_remove_plasmidial;
use crate::{download, tsv};
use std::io;

pub fn tsv_genome() -> Result<(), download::Error> {
    let tsv_path = input_tsv_path();
    let folder_name = input_folder_name();
    let filter_bank = input_filter_bank();
    let accensions = tsv::get_assembly_accension(&tsv_path)
        .into_iter()
        .filter(|accension| accension.starts_with(&filter_bank))
        .collect();
    download_genomes_save_unzip_delete_remove_plasmidial(accensions, &folder_name)?;
    Ok(())
}

pub fn input_tsv_path() -> String {
    let mut input = String::new();
    println!("Digite o caminho do arquivo TSV:");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn input_folder_name() -> String {
    let mut input = String::new();
    println!("Digite o nome da pasta para salvar os genomas baixados:");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
