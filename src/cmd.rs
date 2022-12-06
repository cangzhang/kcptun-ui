use std::env;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

pub fn run(tx: Sender<String>) -> Result<(), Error> {
    println!("current dir {:?}", env::current_dir());

    let output = Command::new("./client_windows_amd64.exe")
        .args(&["-c", "config.json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .stderr
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(output);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            let _r = tx.send(line);
        });

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn capture_stdout() {
        let (tx, _rx) = mpsc::channel();
        let r = crate::cmd::run(tx);
        println!("{:?}", r);
        loop {}
    }
}
