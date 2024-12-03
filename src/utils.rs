use std::process::Command;

pub fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .spawn()
            .expect("Falha ao limpar o console");
    } else {
        Command::new("clear")
            .spawn()
            .expect("Falha ao limpar o console");
    }
}

pub fn mount_is_valid(array: [i32; 2]) -> bool {
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

pub fn get_id_from_xml(xml: String) -> Result<String, String> {
    let splited = xml.split("\n").collect::<Vec<_>>();
    for i in 0..splited.len() {
        if splited[i].starts_with("<Id>") {
            if splited[i + 1].starts_with("<Id>") {
                return Ok(splited[i + 1].replace("<Id>", "").replace("</Id>", ""));
            }
            return Ok(splited[i].replace("<Id>", "").replace("</Id>", ""));
        }
    }
    Err(String::from("Falha ao parsear arquivo"))
}

pub fn get_id_list_from_xml(xml: String) -> Result<String, String> {
    let splited = xml.split("\n");
    for line in splited {
        if line.starts_with("<Id>") {
            return Ok(line.replace("<Id>", "").replace("</Id>", ""));
        }
    }
    Err(String::from("Falha ao parsear arquivo"))
}

pub fn get_download_link_from_xml(xml: String) -> Result<String, String> {
    let splited = xml.split("\n");
    let mut numb = 1;
    for line in splited {
        numb += 1;
        if line.trim().starts_with("<FtpPath_GenBank>") {
            let ftp_link = line
                .replace("<FtpPath_GenBank>", "")
                .replace("</FtpPath_GenBank>", "")
                .replace("ftp://", "");
            let last = ftp_link.split("/").last();
            return match last {
                None => Err("Nenhum link encontrado".parse().unwrap()),
                Some(value) => {
                    let ftp_link_download = "https://".to_owned()
                        + ftp_link.clone().trim()
                        + "/"
                        + value
                        + "_genomic.fna.gz";
                    Ok(ftp_link_download)
                }
            };
        }
    }
    Err(String::from("Falha ao parsear arquivo"))
}
