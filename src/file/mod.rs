use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::Path;

pub fn unzip(file_path: &str) -> String {
    if file_path.ends_with(".gz") {
        unzip_gz(file_path)
    } else {
        unzip_zip(file_path)
    }
}

fn unzip_zip(file_path: &str) -> String {
    let file = File::open(file_path).expect("Falha ao abrir arquivo .zip");
    let mut zip = ZipArchive::new(file).expect("Falha ao criar zip");
    let zip_dir = Path::new(file_path).parent().ok_or("Falha").expect("Falha");
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).expect("Falha ao converter arquivo");
        let out_path = zip_dir.join(file.name());

        if file.is_dir() {
            create_dir_all(&out_path).expect("Falha ao criar diretório");
        } else {
            if let Some(parent) = out_path.parent() {
                create_dir_all(parent).unwrap();
            }
            let mut outfile = File::create(&out_path).unwrap();
            copy(&mut file, &mut outfile).unwrap();
            return String::from(file_path.replace(".zip", ".fasta"))
        }
    }
    String::from("")
}

fn unzip_gz(file_path: &str) -> String {
    let file = File::open(file_path).expect("Falha ao abrir arquivo .gz");
    let mut decoder = GzDecoder::new(file);
    let output_file_path = file_path.strip_suffix(".gz").unwrap();
    let mut output_file = File::create(output_file_path).expect("Falha ao criar arquivo extraído");
    copy(&mut decoder, &mut output_file).expect("Falha ao descompactar arquivo .gz");
    String::from(output_file_path)
}