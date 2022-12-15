#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon_with_id("resources/icon.ico", "main-icon");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
