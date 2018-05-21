extern crate version_check;

fn main() {
    match version_check::is_min_version("1.24.0") {
        Some((true, _version)) => {
            println!("cargo:rustc-cfg=char_stable_inherent_ascii_methods");
        },
        Some(_) => (),
        None => {
            eprintln!("couldn't query version info from rustc");
        },
    }
}
