use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::Parser;

fn publicise(mut r: impl io::Read) -> anyhow::Result<String> {
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    let f = syn::parse_file(&s)?;
    Ok(prettyplease::unparse(&f))
}

struct MakePub;

impl syn::visit_mut::VisitMut for MakePub {
    fn visit_visibility_mut(&mut self, i: &mut syn::Visibility) {
        *i = syn::Visibility::Public(syn::token::Pub::default())
    }
}

#[derive(Parser)]
struct Args {
    #[arg(name("FILE"))]
    files: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let Args { files } = Args::parse();
    if files.is_empty() || (files.len() == 1 && files[0] == Path::new("-")) {
        eprintln!("publicise: <stdin>");
        println!("{}", publicise(io::stdin())?);
    } else {
        for file in files {
            println!("publicise: {}", file.display());
            let s = publicise(fs::File::open(&file)?)?; // mind your drop guards!
            fs::write(file, s)?;
        }
    }
    Ok(())
}
