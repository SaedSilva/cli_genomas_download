use crate::download::download_and_save_fasta_gz;
use crate::entities::NCBIDatasetGenomeRequest;
use crate::file::unzip;
use crate::genome::remove_plasmidial;
use crate::requests::entrez::{
    search_entrez_assembly, search_in_taxonomy, summary_entrez_assembly,
};
use crate::requests::ncbidataset::get_ncbi_dataset;
use crate::{download, xml};
use std::collections::HashSet;
use std::fs::remove_file;
use std::io;
use std::thread::sleep;
use std::time::Duration;

pub fn init() {
    search_genome().unwrap();
}

fn search_genome() -> Result<String, download::Error> {
    loop {
        let mut species = input_species();

        if species == "0" {
            return Ok(String::from("Saindo..."));
        }

        println!("Pesquisando por: {}...", species);
        let xml = search_in_taxonomy(species.as_str())?;
        match xml::get_id_from_xml(xml) {
            Ok(id) => {
                println!("Id encontrado para {}: {}", species, id);

                let exclude_atypical = input_exclude_atypical();
                let assembly_level = input_assembly_level();
                let first_release_date = input_first_release_date();

                let mut request = NCBIDatasetGenomeRequest {
                    id,
                    exclude_atypical,
                    assembly_level,
                    first_release_date,
                    ..NCBIDatasetGenomeRequest::default()
                };

                let filter_bank = input_filter_bank();
                let quantity_of_genomes = input_total_genomes();

                let list = get_list_string_genomes_bank(
                    &mut request,
                    filter_bank.as_str(),
                    quantity_of_genomes as usize,
                );

                for genome in list.clone() {
                    println!("Genoma encontrado: {}", genome);
                }

                download_genomes_save_unzip_delete_remove_plasmidial(list, species.as_str())?;
            }
            Err(_) => {
                println!("Falha ao encontrar Id");
                continue;
            }
        }
    }
}

fn input_species() -> String {
    println!("Digite o nome da especie que deseja pesquisar (0 Para sair):");
    let mut especie = String::new();
    io::stdin().read_line(&mut especie).unwrap();
    String::from(especie.trim())
}

fn input_total_genomes() -> u32 {
    let mut total = String::new();
    println!("Escolha a quantidade máxima: ");
    io::stdin()
        .read_line(&mut total)
        .expect("Falha ao ler linha");
    match total.trim().parse() {
        Ok(num) => return num,
        Err(_) => println!("Falha ao converter numero, continuando com 100..."),
    }
    100
}

fn input_filter_bank() -> String {
    let mut filtro = String::new();
    println!("Deseja filtrar por banco?");
    println!("1- GCF");
    println!("2- GCA");
    println!("3- Sem filtro!!!");
    println!("Outro, Digite: ");

    io::stdin()
        .read_line(&mut filtro)
        .expect("Falha ao ler linha");

    match filtro.trim() {
        "1" => filtro = "GCF".to_string(),
        "2" => filtro = "GCA".to_string(),
        "3" => filtro = "".to_string(),
        _ => return "".to_string(),
    }
    filtro
}

fn input_first_release_date() -> String {
    let mut data_inicial = String::new();
    println!("Digite a data inicial que deve ser filtrada: ");
    io::stdin()
        .read_line(&mut data_inicial)
        .expect("Falha ao ler linha");
    String::from(data_inicial.trim())
}

fn input_assembly_level() -> HashSet<String> {
    let mut numero: [i32; 2] = [0, 0];
    while !mount_is_valid(numero) {
        println!("Escolha o nivel minimo de montagem: ");
        println!("1- contig");
        println!("2- andaime");
        println!("3- cromossomo");
        println!("4- completo");
        let mut nivel_minimo = String::new();
        io::stdin()
            .read_line(&mut nivel_minimo)
            .expect("Falha ao ler linha");
        match nivel_minimo.trim().parse() {
            Ok(num) => numero[0] = num,
            Err(_) => eprintln!("Falha ao converter numero"),
        }

        let mut nivel_maximo = String::new();
        println!("Escolha o nivel maximo de montagem: ");
        io::stdin()
            .read_line(&mut nivel_maximo)
            .expect("Falha ao ler linha");
        match nivel_maximo.trim().parse() {
            Ok(num) => numero[1] = num,
            Err(_) => eprintln!("Falha ao converter numero"),
        }

        if mount_is_valid(numero) {
            let mut result = HashSet::new();
            for num in numero {
                match num {
                    1 => {
                        result.insert("contig".parse().unwrap());
                    }
                    2 => {
                        result.insert("scaffold".parse().unwrap());
                    }
                    3 => {
                        result.insert("chromosome".parse().unwrap());
                    }
                    _ => {
                        result.insert("complete_genome".parse().unwrap());
                    }
                }
            }
            return result;
        }
    }
    HashSet::new()
}

fn input_exclude_atypical() -> bool {
    let mut atipicos = String::new();
    println!("Deseja excluir os genomas atípicos? s/n");
    io::stdin()
        .read_line(&mut atipicos)
        .expect("Falha ao ler linha");
    match atipicos.trim() {
        "s" => true,
        "n" => false,
        _ => {
            eprintln!("entrada inválida, proseguindo como false...");
            false
        }
    }
}

fn mount_is_valid(array: [i32; 2]) -> bool {
    let first = array[0];
    let second = array[1];

    if first < 1 || first > 4 {
        return false;
    }
    if second < 1 || second > 4 {
        return false;
    }
    if second < first {
        return false;
    }
    true
}

fn process_genome_file(assembly: &str, folder_name: &str) -> Result<String, download::Error> {
    let id = search_entrez_assembly(assembly);
    let url = summary_entrez_assembly(&id);
    let file_path = download_and_save_fasta_gz(url.as_str(), folder_name, assembly)?;

    match unzip_and_process_file(file_path) {
        Ok(file_out_path) => {
            remove_plasmidial(file_out_path.as_str()).unwrap();
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
    let mut count = 1;

    for genome in genomes {
        println!("{}/{} - Baixando genoma: {}", count, total, genome);

        match process_genome_file(&genome, folder_name) {
            Ok(_) => count += 1,
            Err(_) => eprintln!("Falha ao processar o genoma: {}", genome),
        }

        println!();
        sleep(Duration::from_secs(1)); // Consider replacing with async for better performance
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
