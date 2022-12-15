use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;
use std::{env, thread};

pub fn run(conf_path: &String, tx: Option<Sender<(String, u32)>>) -> Result<Child, Error> {
    println!("[cmd::run] current dir {:?}", env::current_dir().unwrap());

    let bin_path = match env::consts::OS {
        "windows" => "./client_windows_amd64.exe",
        "macos" => "./client_darwin_amd64",
        _ => "./client_linux_amd64",
    };

    let mut cmd = Command::new(bin_path)
        .args(["-c", conf_path])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let pid = cmd.id();
    let output = cmd
        .stderr
        .take()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(output);
    thread::spawn(move || {
        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| {
                if let Some(tx) = &tx {
                    let _r = tx.send((line, pid));
                }
            });
    });

    Ok(cmd)
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
        let r = crate::cmd::run(&conf, Some(tx));
        println!("[run result] {:?}", r);
        loop {
            let r = rx.recv();
            println!("[received log] {:?}", r);
        }
    }
}
