use std::env;

fn check_bin() {
    // let path = Path::new("./")
    let os = match env::consts::OS {
        "windows" => "windows",
        "linux" => "linux",
        "macos" => "darwin",
        _ => ""
    };
    let arch = match env::consts::ARCH {
        "x86" => "386",
        "x86_64" => "amd64",
        _ => "",
    };
    let tag = format!("{}-{}", os, arch);
    println!("tag is: {}", tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_bin() {
        check_bin()
    }
}