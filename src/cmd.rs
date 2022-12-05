use std::env;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

pub fn run() -> Result<(), Error> {
    println!("current dir {:?}", env::current_dir());

    let stdout = Command::new("./client_windows_amd64.exe")
        .args(&["-c", "./config.json"])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn capture_stdout() {
        run();
    }
}
