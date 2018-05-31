extern crate nom;
extern crate tl_lang_syn;


use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process;

use tl_lang_syn::print::Print;


#[derive(Clone, Copy, Debug)]
enum PrintMode {
    SyntaxTree,
    Schema,
}


fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 4 {
        eprintln!("Usage: {} <DUMP_MODE> <INPUT_FILE> <OUTPUT_FILE>", args[0]);
        process::exit(1);
    }

    let print_mode = match args[1].as_str() {
        "syntax-tree" | "syntax_tree" => PrintMode::SyntaxTree,
        "schema" => PrintMode::Schema,
        other => {
            eprintln!("Invalid print mode: {}", other);
            process::exit(1);
        },
    };

    parse_print(print_mode, &args[2], &args[3]).unwrap();
}

fn parse_print<P1, P2>(print_mode: PrintMode, input_path: P1, output_path: P2) -> io::Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let scheme = tl_lang_syn::parse_file(input_path)?;

    {
        let mut output_file = File::create(output_path)?;

        match print_mode {
            PrintMode::SyntaxTree => write!(output_file, "{:#?}", scheme)?,
            PrintMode::Schema => write!(output_file, "{}", scheme.display_wrapper())?,
        }
    }

    Ok(())
}
