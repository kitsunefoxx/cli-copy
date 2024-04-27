use clap::{Command, Arg};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::path::{PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("cli-copy")
        .version("1.0")
        .about("Handles various clipboard copying tasks")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Copies the full path of the current directory or a specified file/folder"),
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Copies just the name of the specified file or directory"),
        )
        .get_matches();

    let mut ctx: ClipboardContext = ClipboardProvider::new()?;

    if let Some(p) = matches.get_one::<String>("path") {
        let path = if p == "." {
            std::env::current_dir()?
        } else {
            let mut path = std::env::current_dir()?;
            path.push(p);
            path
        };
        ctx.set_contents(path.to_string_lossy().into_owned())?;
    }

    if let Some(n) = matches.get_one::<String>("name") {
        let path = std::env::current_dir()?;
        let mut name_path = PathBuf::from(&path);
        name_path.push(n);
        // Get only the file or directory name, not the full path
        if let Some(file_name) = name_path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                ctx.set_contents(name_str.to_string())?;
            }
        }
    }

    Ok(())
}
