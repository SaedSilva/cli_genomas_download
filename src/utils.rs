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




