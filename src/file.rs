use flate2::read::GzDecoder;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::Path;
use zip::ZipArchive;

pub struct Unzipper {
    output_file: Option<&'static File>,
}

impl Unzipper {
    pub fn unzip(&mut self, file_path: &str) {
        if file_path.ends(".gz") {
            self.unzip_gz(file_path);
        } else {
            self.unzip_zip(file_path)
        }
    }

    fn unzip_zip(&mut self, file_path: &str) {
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
                self.output_file = Some(&outfile);
                copy(&mut file, &mut outfile).unwrap();
            }
        }
    }

    fn unzip_gz(&mut self, file_path: &str) {
        let file = File::open(file_path).expect("Falha ao abrir arquivo .gz");
        let mut decoder = GzDecoder::new(file);
        let output_file_path = file_path.strip_suffix(".gz").unwrap();
        let mut output_file =
            File::create(output_file_path).expect("Falha ao criar arquivo extraído");
        self.output_file = Some(&output_file);
        copy(&mut decoder, &mut output_file).expect("Falha ao descompactar arquivo .gz");
    }
}
