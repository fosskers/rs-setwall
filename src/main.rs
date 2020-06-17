//! A glorified wrapper around `hsetroot`.

use gumdrop::{Options, ParsingStyle};
use rand::seq::IteratorRandom;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

#[derive(Options)]
struct Args {
    help: bool,

    #[options(command)]
    command: Option<Commands>,
}

#[derive(Options)]
enum Commands {
    /// Set a specific image file as the background.
    Set(Set),
    /// Choose a random image file from a given directory.
    Random(Random),
}

#[derive(Options)]
struct Set {
    #[options(free)]
    image: PathBuf,
}

#[derive(Options)]
struct Random {
    #[options(free)]
    dir: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse_args_or_exit(ParsingStyle::AllOptions);

    match args.command {
        Some(Commands::Set(Set { image })) if image.is_file() => set_wall(&image),
        Some(Commands::Random(Random { dir })) if dir.is_dir() => {
            let p = rand_img(&dir)?;
            set_wall(&p)
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, "File does not exist!")),
    }
}

fn set_wall(path: &Path) -> io::Result<()> {
    Command::new("hsetroot")
        .arg("-fill")
        .arg(path)
        .output()
        .map(|_| ()) // TODO Any better way to do this?
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
