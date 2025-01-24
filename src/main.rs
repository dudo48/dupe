mod size;
use size::Size;

use std::hash::Hash;
use std::{
    collections::{BTreeMap, HashMap},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

use clap::{Parser, ValueEnum};
use walkdir::{DirEntry, WalkDir};

#[derive(PartialEq, Eq, ValueEnum, Clone)]
enum Algorithm {
    NAME,
    SIZE,
    BOTH,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    root: PathBuf,

    #[arg(short, long, default_value_t = 1.0)]
    min_size: f32,

    #[arg(value_enum, long, default_value_t = Algorithm::NAME)]
    algorithm: Algorithm,
}

// Groups a vector of DirEntry according to the result of the closure f
fn find_duplicates_by<F, K>(files: Vec<DirEntry>, f: F) -> impl Iterator<Item = Vec<DirEntry>>
where
    F: Fn(&DirEntry) -> Option<K>,
    K: Eq + Hash,
{
    return files
        .into_iter()
        .fold(HashMap::new(), |mut map, entry| {
            f(&entry).map(|key| map.entry(key).or_insert(Vec::new()).push(entry));
            return map;
        })
        .into_values()
        .filter(|g| g.len() > 1);
}

fn find_duplicate_files(
    files: Vec<DirEntry>,
    algorithm: Algorithm,
) -> impl Iterator<Item = Vec<DirEntry>> {
    let mut dupes = vec![files].into_iter();

    if [Algorithm::SIZE, Algorithm::BOTH].contains(&algorithm) {
        dupes = dupes
            .map(|g| find_duplicates_by(g, |e| e.metadata().map(|m| m.size()).ok()))
            .flatten()
            .collect::<Vec<_>>()
            .into_iter();
        println!("Found {} duplicate groups by size", dupes.len());
    }
    if [Algorithm::NAME, Algorithm::BOTH].contains(&algorithm) {
        dupes = dupes
            .map(|g| find_duplicates_by(g, |e| Some(e.file_name().to_owned())))
            .flatten()
            .collect::<Vec<_>>()
            .into_iter();
        println!("Found {} duplicate groups by name", dupes.len());
    }

    println!();
    return dupes;
}

fn main() {
    let args = Args::parse();
    let root = args.root;
    let min_size = (args.min_size * 1_000_000.0) as u64;
    let algorithm = args.algorithm;

    // Find all files
    let files: Vec<_> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| {
            e.ok().filter(|e| {
                e.file_type().is_file() && e.metadata().is_ok_and(|m| m.size() > min_size)
            })
        })
        .collect();

    // Find duplicate files and sort them according to size
    let dupe_map =
        find_duplicate_files(files, algorithm).fold(BTreeMap::new(), |mut map, group| {
            let size = group
                .iter()
                .filter_map(|e| e.metadata().map(|m| m.size()).ok())
                .sum();

            map.entry(size).or_insert(group);
            return map;
        });

    let mut total_size = 0;
    let total_count = dupe_map.len();
    for (size, group) in dupe_map.into_iter().rev() {
        total_size += size;
        println!("{}", Size::new(size));
        for file in group {
            println!("{}", file.into_path().display());
        }
        println!();
    }

    println!(
        "Found a total of {} duplicate groups occupying a space of {}",
        total_count,
        Size::new(total_size)
    );
}
