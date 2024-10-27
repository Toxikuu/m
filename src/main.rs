use std::env;
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
    if args.len() != 3 {
        die!("usage: m <source> <destination>\n\nWARNING\nm is NOT safe\nuse it with great care\nif you fuck up backups are made in /tmp/m");
    }

    let src = &args[1];
    let dst = &args[2];

    let s = Path::new(src);
    let d = Path::new(dst);

    if let Err(e) = mv(s, d) {
        die!("failed to move {:?} to {:?}: {}", s, d, e);
    }
}

fn backup(d: &Path) -> io::Result<()> {
    let bak = Path::new("/tmp/m");
    if !bak.exists() {
        fs::create_dir_all(bak)?;
    }

    let b = bak.join(d.file_name().unwrap());

    if b.exists() {
        if d.is_dir() {
            fs::remove_dir_all(d)?;
        } else {
            fs::remove_file(d)?;
        }
    }

    fs::rename(d, b)?;
    Ok(())
}

fn mv(s: &Path, d: &Path) -> io::Result<()> {
    if !s.exists() {
        die!("source {:?} does not exist.", s);
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
