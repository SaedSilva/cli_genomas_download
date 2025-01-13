use flate2::read::GzDecoder;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::Path;
use zip::ZipArchive;

pub struct Unzipper {
    file_path: String,
    pub output_file: Option<String>,
}

impl Unzipper {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: String::from(file_path),
            output_file: None,
        }
    }
    pub fn unzip(&mut self) {
        if self.file_path.ends_with(".gz") {
            self.unzip_gz();
        } else {
            self.unzip_zip();
        }
    }

    fn unzip_zip(&mut self) {
        let path_str = self.file_path.as_str();
        let file = File::open(path_str).expect("Falha ao abrir arquivo .zip");
        let mut zip = ZipArchive::new(file).expect("Falha ao criar zip");
        let zip_dir = Path::new(path_str).parent().ok_or("Falha").expect("Falha");
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
                self.output_file = Some(self.file_path.replace(".zip", ".fasta"));
                copy(&mut file, &mut outfile).unwrap();
            }
        }
    }

    fn unzip_gz(&mut self) {
        let file = File::open(self.file_path.as_str()).expect("Falha ao abrir arquivo .gz");
        let mut decoder = GzDecoder::new(file);
        let output_file_path = self.file_path.strip_suffix(".gz").unwrap();
        let mut output_file =
            File::create(output_file_path).expect("Falha ao criar arquivo extraído");
        self.output_file = Some(String::from(output_file_path));
        copy(&mut decoder, &mut output_file).expect("Falha ao descompactar arquivo .gz");
    }
}
