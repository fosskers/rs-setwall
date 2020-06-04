//! A glorified wrapper around `hsetroot`.

use rand::seq::IteratorRandom;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Args {
    /// Set a specific image file as the background.
    Set {
        #[structopt(parse(from_os_str))]
        image: PathBuf,
    },
    /// Choose a random image file from a given directory.
    Random {
        #[structopt(parse(from_os_str))]
        dir: PathBuf,
    },
}

fn main() -> io::Result<()> {
    match Args::from_args() {
        Args::Set { image } if image.is_file() => set_wall(&image),
        Args::Random { dir } if dir.is_dir() => rand_img(&dir).and_then(|p| set_wall(&p)),
        _ => Err(io::Error::new(io::ErrorKind::Other, "File does not exist!")),
    }
}

fn set_wall(path: &Path) -> io::Result<()> {
    Command::new("hsetroot")
        .arg("-fill")
        .arg(path)
        .output()
        .map(|_| ())
}

/// Fetches a random image file from some directory.
fn rand_img(dir: &Path) -> io::Result<PathBuf> {
    let files = fs::read_dir(dir)?;
    let mut rng = rand::thread_rng();
    let image = files
        .filter_map(|p| p.ok())
        .map(|p| p.path())
        .filter(|p| is_image(p))
        .choose(&mut rng)
        .ok_or(io::Error::new(io::ErrorKind::Other, "No files!"))?;

    Ok(image)
}

/// Extension must be lower case.
fn is_image(path: &Path) -> bool {
    match path.extension() {
        Some(p) if p == "jpg" || p == "png" || p == "jpeg" => true,
        _ => false,
    }
}

#[test]
fn they_are_images() {
    assert!(is_image(Path::new("foo.jpg")));
    assert!(is_image(Path::new("foo.png")));
    assert!(is_image(Path::new("foo.jpeg")));
    assert!(!is_image(Path::new("foo.txt")));
}
