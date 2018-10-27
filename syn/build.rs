extern crate version_check;

fn query_print_cfg(version: &str, flag: &str) {
    match version_check::is_min_version(version) {
        Some((true, _version)) => {
            println!("cargo:rustc-cfg={}", flag);
        },
        Some(_) => (),
        None => {
            eprintln!("couldn't query version info from rustc");
        },
    }
}

fn main() {
    query_print_cfg("1.28.0", "stable_nonzero_types");
}
