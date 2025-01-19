use reqwest::blocking::Client;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

pub fn download_and_save_fasta_gz(
    url: &str,
    folder_name: &str,
    file_name: &str,
) -> Result<String, Error> {
    let file_path = get_download_path(folder_name).join(format!("{}.fasta.gz", file_name));
    let mut file_result = File::create(&file_path)?;

    println!("Downloading from: {}", url);
    let http_client: Client = Client::new();
    let response = http_client.get(url).send()?;
    let bytes = response.bytes()?;

    file_result.write_all(&bytes)?;
    Ok(String::from(file_path.to_str().unwrap()))
}

fn get_download_path(folder_name: &str) -> PathBuf {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let download_dir = exe_dir.join(format!("downloads/{}", folder_name));
    create_dir_all(&download_dir).unwrap();
    download_dir
}

#[derive(Debug)]
pub enum Error {
    IO(()),
    REQWEST(()),
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::IO(())
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Error::REQWEST(())
    }
}
