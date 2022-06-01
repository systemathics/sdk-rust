use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn is_proto(entry: Option<DirEntry>) -> Option<DirEntry> {
    match entry {
        Some(e) => match e.path().to_str() {
            Some(name) => match name.ends_with(".proto") {
                true => Some(e),
                false => None,
            },
            None => None,
        },
        None => None,
    }
}

// fn display(vec: &Vec<&Path>) -> () {
//     for element in vec {
//         println!("{}", element.display());
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vec: Vec<DirEntry> = Vec::new();
    for entry in WalkDir::new("proto")
        .into_iter()
        .filter_map(|e| (is_proto(e.ok())))
    {
        //println!("{}", entry.path().display());
        vec.push(entry);
    }
    //println!("{}", vec.len());
    let protos: Vec<&Path> = vec.iter().map(|d| d.path()).collect();
    //display(&protos);
    tonic_build::configure()
        .build_server(false)
        .out_dir("src")
        .compile(&protos[..], &["proto"])?;
    return Ok(());
}
