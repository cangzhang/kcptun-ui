use std::env;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;

pub fn run(tx: Option<Sender<(String, u32, u8)>>, idx: u8) -> Result<Child, Error> {
    println!("[cmd::run] current dir {:?}", env::current_dir().unwrap());

    let mut cmd = Command::new("./client_windows_amd64.exe")
        .args(["-c", "config.json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let pid = cmd.id();
    let output = cmd
        .stderr
        .take()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(output);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            if let Some(tx) = tx.clone() {
                let _r = tx.send((line, pid, idx));
            }
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
        let (tx, _rx) = mpsc::channel();
        let r = crate::cmd::run(Some(tx), 0);
        println!("{:?}", r);
        loop {}
    }
}
