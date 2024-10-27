use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

macro_rules! die {
    ($($arg:tt)*) => {{
        eprintln!("\x1b[31mm: {}\x1b[0m", format!($($arg)*));
        std::process::exit(1);
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        die!("usage: m <src> <src2> ... <dst>\n\nWARNING\nm is NOT safe\nuse it with great care\nif you fuck up backups are made in /tmp/m");
    }

    let d = Path::new(&args[args.len() - 1]);
    let ss = &args[1..args.len() - 1];

    for s_ in ss {
        let s = Path::new(s_);
        if let Err(e) = mv(s, d) {
            die!("failed to move {:?} to {:?}: {}", s, d, e);
        }
    }
}

fn backup(d: &Path) -> io::Result<()> {
    match d.to_str() {
        Some(".") | Some("..") => return Ok(()),
        _ => {}
    };

    let bak = Path::new("/tmp/m");
    if !bak.exists() {
        fs::create_dir_all(bak)?;
    }

    let b = bak.join(d.file_name().unwrap_or_else(|| OsStr::new("backup")));

    let metadata = fs::metadata(d)?;
    if !metadata.is_file() && !metadata.is_dir() {
        println!("not backing up special file: {:?}", d);
        return Ok(());
    }

    if b.exists() {
        if b.is_dir() {
            fs::remove_dir_all(&b)?;
        } else {
            fs::remove_file(&b)?;
        }
    }

    fs::rename(d, b)?;
    Ok(())
}

fn mv(s: &Path, d: &Path) -> io::Result<()> {
    if !s.exists() {
        die!("source {:?} does not exist", s);
    }

    if d.exists() {
        if let Err(e) = backup(d) {
            die!("failed to backup {:?}: {}", d, e);
        }
        println!("{:?} -x", d);
    }

    fs::rename(s, d)?;
    println!("{:?} -> {:?}", s, d);
    Ok(())
}
