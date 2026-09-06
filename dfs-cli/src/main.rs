use dfs_core::{chunk_file, reassmble};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let x = chunk_file("hello.txt")?;
    reassmble("hello.txt")?;
    // println!("{:?}", x);
    Ok(())
}
