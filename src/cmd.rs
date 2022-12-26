use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;
use std::{env, thread};

pub fn run(conf_path: &String, tx: Sender<(String, u32)>) -> Result<Child, Error> {
    #[cfg(target_os = "windows")]
    use std::os::windows::process::CommandExt;

    let bin_path = match env::consts::OS {
        "windows" => "./client_windows_amd64.exe",
        "macos" => "./client_darwin_amd64",
        _ => "./client_linux_amd64",
    };

    #[cfg(not(target_os = "windows"))]
    let mut handler = Command::new(bin_path)
        .args(["-c", conf_path])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    #[cfg(target_os = "windows")]
    let mut handler = Command::new(bin_path)
        .args(["-c", conf_path])
        .creation_flags(0x08000000)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let pid = handler.id();
    let output = handler
        .stderr
        .take()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(output);
    thread::spawn(move || {
        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| {
                let _r = tx.send((line, pid));
            });
    });

    Ok(handler)
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn capture_stdout() {
        let (tx, rx) = mpsc::channel();
        let conf = String::from("./config.json");
        let r = crate::cmd::run(&conf, tx);
        println!("[run result] {:?}", r);
        loop {
            let r = rx.recv();
            println!("[received log] {:?}", r);
        }
    }
}
