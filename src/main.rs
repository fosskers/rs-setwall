//! A glorified wrapper around `hsetroot`.

use anyhow::anyhow;
use gumdrop::{Options, ParsingStyle};
use rand::seq::IteratorRandom;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;

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

fn main() {
    let args = Args::parse_args_or_exit(ParsingStyle::AllOptions);

    match work(args) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}

fn work(args: Args) -> anyhow::Result<()> {
    match args.command {
        Some(Commands::Set(Set { image })) if image.is_file() => set_wall(&image),
        Some(Commands::Random(Random { dir })) if dir.is_dir() => {
            let p = rand_img(&dir)?;
            set_wall(&p)
        }
        _ => Err(anyhow!("File does not exist!")),
    }
}

fn set_wall(path: &Path) -> anyhow::Result<()> {
    Command::new("hsetroot").arg("-fill").arg(path).output()?;
    Ok(())
}

/// Fetches a random image file from some directory.
fn rand_img(dir: &Path) -> anyhow::Result<PathBuf> {
    let files = fs::read_dir(dir)?;
    let mut rng = rand::thread_rng();
    let image = files
        .filter_map(|p| p.ok())
        .map(|p| p.path())
        .filter(|p| is_image(p))
        .choose(&mut rng)
        .ok_or(anyhow!("No files!"))?;

    Ok(image)
}

/// Extension must be lower case.
fn is_image(path: &Path) -> bool {
    match path
        .extension()
        .and_then(|p| p.to_str())
        .map(|p| p.to_lowercase())
    {
        Some(p) if p == "jpg" || p == "png" || p == "jpeg" => true,
        _ => false,
    }
}

#[test]
fn they_are_images() {
    assert!(is_image(Path::new("foo.jpg")));
    assert!(is_image(Path::new("foo.png")));
    assert!(is_image(Path::new("foo.jpeg")));
    assert!(is_image(Path::new("foo.JPEG")));
    assert!(!is_image(Path::new("foo.txt")));
}
