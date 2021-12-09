//! Set your wallpaper on Wayland or X11.

use anyhow::anyhow;
use gumdrop::{Options, ParsingStyle};
use rand::seq::IteratorRandom;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::str::FromStr;

/// Set your wallpaper on Wayland or X11.
#[derive(Options)]
struct Args {
    /// Print this help message.
    help: bool,

    #[options(command)]
    command: Option<Cmd>,
}

#[derive(Options)]
enum Cmd {
    /// Set a specific image file as the background.
    Set(Set),
    /// Choose a random image file from a given directory.
    Random(Random),
}

#[derive(Options)]
struct Set {
    /// Print this help message.
    help: bool,

    /// Sway output to set background to or X screen number if using X11
    #[options(help = "sway output name or X screen number")]
    output: Vec<String>,

    /// The path to the image file.
    #[options(free)]
    image: PathBuf,

    /// The target output compositor. (values: sway, x11 [default])
    #[options(meta = "C")]
    comp: Compositor,

    /// Should the program kill existing swaybg processes
    #[options(help = "kill existing swaybg processes")]
    kill_swaybg: bool,
}

#[derive(Options)]
struct Random {
    /// Print this help message.
    help: bool,

    /// Sway output to set background to or X screen number if using X11
    #[options(help = "sway output name or X screen number")]
    output: Vec<String>,

    /// The path to choose an image file from.
    #[options(free)]
    dir: PathBuf,

    /// The target output compositor. (values: sway, x11 [default])
    comp: Compositor,

    /// Should the program kill existing swaybg processes
    #[options(help = "kill existing swaybg processes")]
    kill_swaybg: bool,
}

enum Compositor {
    Sway,
    X11,
}

impl Default for Compositor {
    fn default() -> Self {
        Compositor::X11
    }
}

impl FromStr for Compositor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x11" => Ok(Compositor::X11),
            "sway" => Ok(Compositor::Sway),
            _ => Err("Unsupported compositor type."),
        }
    }
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
        Some(Cmd::Set(Set { mut output, image, comp, kill_swaybg, .. })) if image.is_file() => match comp {
                Compositor::Sway => {
                    if kill_swaybg {
                        Command::new("pkill").arg("swaybg").output()?;
                    }
                    if output.is_empty() {
                        output = Vec::new();
                        output.push(String::from("*"));
                    }
                    let mut res: anyhow::Result<()> = Ok(());
                    for o in output {
                        res = swaybg(o.as_str(), &image)
                    }
                    res
                },
                Compositor::X11 => {
                    if output.is_empty() {
                        output = Vec::new();
                        output.push(std::option_env!("DISPLAY").unwrap_or(":0").to_string());
                    }
                    let mut res: anyhow::Result<()> = Ok(());
                    for o in output {
                        res = hsetroot(o.as_str(), &image)
                    }
                    res
                },
        },
        Some(Cmd::Random(Random { mut output, dir, comp, kill_swaybg, .. })) if dir.is_dir() => {
            match comp {
                Compositor::Sway => {
                    if kill_swaybg {
                        Command::new("pkill").arg("swaybg").output()?;
                    }
                    if output.is_empty() {
                        output = Vec::new();
                        output.push(String::from("*"));
                    }
                    let mut res: anyhow::Result<()> = Ok(());
                    for o in output {
                        let p = rand_img(&dir)?;
                        res = swaybg(o.as_str(), &p)
                    }
                    res
                },
                Compositor::X11 => {
                    if output.is_empty() {
                        output = Vec::new();
                        output.push(std::option_env!("DISPLAY").unwrap_or(":0").to_string());
                    }
                    let mut res: anyhow::Result<()> = Ok(());
                    for o in output {
                        let p = rand_img(&dir)?;
                        res = hsetroot(o.as_str(), &p)
                    }
                    res
                },
            }
        }
        Some(_) => Err(anyhow!("File doesn't exist!")),
        None => {
            if let Some(cl) = Args::command_list() {
                Err(anyhow!(format!(
                    "{}\n\nAvailable commands:\n{}",
                    Args::usage(),
                    cl,
                )))
            } else {
                Err(anyhow!("Something is wrong with the commandline flags."))
            }
        }
    }
}

fn swaybg(output: &str, path: &Path) -> anyhow::Result<()> {
    Command::new("swaybg")
        .arg("-o")
        .arg(output)
        .arg("-i")
        .arg(path)
        .arg("-m")
        .arg("fill")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

fn hsetroot(output: &str, path: &Path) -> anyhow::Result<()> {
    Command::new("hsetroot").arg("-screens").arg(output).arg("-fill").arg(path).output()?;
    Ok(())
}

/// Fetches a random image file from some directory.
fn rand_img(dir: &Path) -> anyhow::Result<PathBuf> {
    let files = std::fs::read_dir(dir)?;
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
