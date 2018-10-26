use std::path::Path;
use std::io::{stdin, stdout, Write};

pub fn keyfile_exists(path: &String) -> bool {
    Path::new(path).exists()
}

pub fn get_affirmation(warn_msg: String) -> bool {
    let mut s = String::new();
    print!("[!] WARNING! {} Proceed? y/n\n", warn_msg);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("[-] You did not enter a correct string");
    if s.trim() == "y" || s.trim() == "yes" || s.trim() == "Y" || s.trim() == "YES" || s.trim() == "Yes" { true } else { false }
}
