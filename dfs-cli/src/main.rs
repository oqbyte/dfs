use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};

use anyhow::{Ok, anyhow};
use clap::Parser;
use dfs_core::{chunk_file, purge, reassmble};

#[derive(Parser)]
struct Args {
    #[arg(short, long, num_args = 1)]
    chunk_size: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or_default();
        match command.trim() {
            "ls" => list_files()?,
            "add" => {
                let from = parts.next().unwrap_or_default();
                match add_file(from, args.chunk_size) {
                    Result::Ok(_) => println!("Done!"),
                    Err(e) => println!("{e}"),
                };
            }
            "purge" => {
                let file_name = parts.next().unwrap_or_default();
                purge(file_name)?;
            },
            "read" => {
                let file_name = parts.next().unwrap_or_default();
                reassmble(file_name)?;
                let mut file = File::open(format!("./restored/{}",file_name ))?;
                let mut vec_buf: Vec<u8> = Vec::new();
                file.read_to_end(&mut vec_buf)?;
                let content = String::from_utf8_lossy(vec_buf.as_slice());
                println!("{content}");
            }
            "exit" | "quit" => return Ok(()),
            x => println!("unknown command: {x}"),
        }
    }
}

fn list_files() -> anyhow::Result<()> {
    for entry in fs::read_dir("./maps")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if let Some(s) = file_name.strip_suffix(".map.json") {
            println!("{s}")
        }
    }

    Ok(())
}

fn add_file(from: &str, chunk_size: usize) -> anyhow::Result<()> {
    let path = PathBuf::from(from);
    if !path.exists() {
        return Err(anyhow!("file dont exists"));
    }
    if path.is_dir() {
        return Err(anyhow!("cannot add a dirictory"));
    }

    chunk_file(from, chunk_size)?;
    Ok(())
}

