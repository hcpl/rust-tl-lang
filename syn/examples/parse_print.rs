extern crate nom;
extern crate tl_lang_syn;


use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;


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
    let source = {
        let mut input_file = File::open(input_path)?;
        let mut string = String::new();

        input_file.read_to_string(&mut string)?;

        string
    };

    let scheme = tl_lang_syn::parse_file(&source)
        .map_err(|error| io::Error::new(io::ErrorKind::Other, nom_err_to_owned(error)))?;

    {
        let mut output_file = File::create(output_path)?;

        match print_mode {
            PrintMode::SyntaxTree => write!(output_file, "{:#?}", scheme)?,
            PrintMode::Schema => write!(output_file, "{}", scheme)?,
        }
    }

    Ok(())
}

fn nom_err_to_owned<I, E>(error: nom::Err<&I, E>) -> nom::Err<I::Owned, E>
where
    I: ToOwned + ?Sized,
{
    match error {
        nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
        nom::Err::Error(context)     => nom::Err::Error(nom_context_to_owned(context)),
        nom::Err::Failure(context)   => nom::Err::Failure(nom_context_to_owned(context)),
    }
}

fn nom_context_to_owned<I, E>(context: nom::Context<&I, E>) -> nom::Context<I::Owned, E>
where
    I: ToOwned + ?Sized,
{
    match context {
        nom::Context::Code(input, kind) => nom::Context::Code(input.to_owned(), kind),
    }
}
