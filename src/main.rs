mod api;
mod utils;

use crate::api::NBCIRequest;
use std::io;

fn main() {
    loop {
        println!("Digite o nome da especie que deseja pesquisar: ");

        let mut especie = String::new();
        io::stdin()
            .read_line(&mut especie)
            .expect("Falha ao ler linha");

        if especie.trim() == "0" {
            panic!("Finalizando...")
        }

        println!("Pesquisando por: {}...", especie.trim());
        let mut request = NBCIRequest::default();
        match api::search_in_assembly(especie.trim()) {
            Ok(xml) => match utils::get_id_from_xml(xml) {
                Ok(id) => {
                    request.id = id.clone();
                    println!("Id encontrado para {}: {}", especie.trim(), id);

                    atipicos(&mut request);

                    niveis(&mut request);

                    data_inicial(&mut request);

                    let filtro = filtro();

                    let total = total();

                    let list = api::get_list_string_of_name(request, filtro.trim(), total as usize);

                    for genome in list.clone() {
                        println!("Genoma encontrado: {}", genome.trim());
                    }

                    api::download_genomes(list, especie.trim()).expect("FALHA!!!");
                }
                Err(_) => {
                    eprintln!("Falha ao encontrar Id");
                    continue;
                }
            },

            _ => {}
        }
    }
}

fn total() -> u32 {
    let mut total = String::new();
    println!("Escolha a quantidade máxima: ");
    io::stdin()
        .read_line(&mut total)
        .expect("Falha ao ler linha");
    match total.trim().parse() {
        Ok(num) => return num,
        Err(_) => eprintln!("Falha ao converter numero, continuando com 100..."),
    }
    100
}

fn filtro() -> String {
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

fn data_inicial(request: &mut NBCIRequest) {
    let mut data_inicial = String::new();
    println!("Digite a data inicial que deve ser filtrada: ");
    io::stdin()
        .read_line(&mut data_inicial)
        .expect("Falha ao ler linha");
    request.first_release_date = data_inicial.trim().parse().unwrap();
}

fn niveis(request: &mut NBCIRequest) {
    let mut numero: [i32; 2] = [0, 0];
    while !utils::mount_is_valid(numero) {
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

        if utils::mount_is_valid(numero) {
            for num in numero {
                match num {
                    1 => {
                        request.assembly_level.insert("contig".parse().unwrap());
                    }
                    2 => {
                        request.assembly_level.insert("scaffold".parse().unwrap());
                    }
                    3 => {
                        request.assembly_level.insert("chromosome".parse().unwrap());
                    }
                    _ => {
                        request
                            .assembly_level
                            .insert("complete_genome".parse().unwrap());
                    }
                }
            }
            break;
        }
        println!("Opcao invalida");
    }
}

fn atipicos(request: &mut NBCIRequest) {
    let mut atipicos = String::new();
    println!("Deseja excluir os genomas atípicos? s/n");
    io::stdin()
        .read_line(&mut atipicos)
        .expect("Falha ao ler linha");
    match atipicos.trim() {
        "s" => request.exclude_atypical = true,
        "n" => request.exclude_atypical = false,
        _ => {
            eprintln!("entrada inválida, proseguindo como false...")
        }
    }
}
