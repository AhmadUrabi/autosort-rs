use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::collections::HashMap;

use clap::Parser;

#[cfg(target_os = "windows")]
fn sort_files(files: Vec<PathBuf>, map: HashMap<&str, &str>, base: &str) {
    for i in files {
        let ext = i.extension();
        if ext.is_none() {
            println!("Cannot Find {:?}",i.to_str());
            continue;
        }
        let ext = ext.unwrap();
        if let Some(dir) = map.get(ext.to_str().unwrap()) {
            if std::fs::read_dir(format!("{}\\{}",base,dir)).is_err() {
                std::fs::create_dir(format!("{}\\{}",base,dir)).unwrap();
            }
            let filename = i.to_str().unwrap().split('\\').last().unwrap();
            std::fs::rename(format!("{}\\{}",base,filename), format!("{}\\{}\\{}",base,dir,filename)).unwrap();
        } else {
            if std::fs::read_dir(format!("{}\\{}",base,"Others")).is_err() {
                std::fs::create_dir(format!("{}\\{}",base,"Others")).unwrap();
            }
            let filename = i.to_str().unwrap().split('\\').last().unwrap();
            std::fs::rename(format!("{}\\{}",base,filename), format!("{}\\{}\\{}",base,"Others",filename)).unwrap();
        }
    }
}

// TODO: Recursive search, directories with a single file type inside parent dir

#[cfg(not(target_os = "windows"))]
fn sort_files(files: Vec<PathBuf>, map: HashMap<&str, &str>, base: &str) {
    for i in files {
        let ext = i.extension();
        if ext.is_none() {
            println!("Cannot Find {:?}",i.to_str());
            continue;
        }
        let ext = ext.unwrap();
        if let Some(dir) = map.get(ext.to_str().unwrap()) {
            if std::fs::read_dir(format!("{}/{}",base,dir)).is_err() {
                std::fs::create_dir(format!("{}/{}",base,dir)).unwrap();
                println!("Created Dir");
            }
            let filename = i.to_str().unwrap().split('/').last().unwrap();
            std::fs::rename(format!("{}/{}",base,filename), format!("{}/{}/{}",base,dir,filename)).unwrap();
        } else {
            if std::fs::read_dir(format!("{}/{}",base,"Others")).is_err() {
                std::fs::create_dir(format!("{}/{}",base,"Others")).unwrap();
            }
            let filename = i.to_str().unwrap().split('/').last().unwrap();
            std::fs::rename(format!("{}/{}",base,filename), format!("{}/{}/{}",base,"Others",filename)).unwrap();
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // TODO: Fix reading relative paths
    #[arg(short, long, value_name = "DIR")]
    path: PathBuf,
    #[arg(short, long, value_name = "Ignore Hidden Files")]
    ignore_hidden: Option<bool>,
    #[arg(short, long, value_name = "Ignore SymLinks Files")]
    ignore_symlink: Option<bool>
}

fn main() {
    println!("Autosort Started");
    let args = Args::parse();
    let mut base = args.path;

    let ignore_hidden = match args.ignore_hidden {
        Some(a) => a,
        None => true
    };

    let ignore_symlink = match args.ignore_hidden {
        Some(a) => a,
        None => true
    };

    let now = std::time::Instant::now();

    // TODO: Extract to config file
    let map: HashMap<&str, &str> = HashMap::from([
        ("pdf","Documents"),
        ("txt","Documents"),
        ("docx","Documents"),
        ("doc","Documents"),
        ("ppt","Slides"),
        ("pptx","Slides"),
        ("png","Pictures"),
        ("jpg","Pictures"),
        ("jpeg","Pictures"),
        ("csv","Spreadsheets"),
        ("xlsx","Spreadsheets"),
        ("xls","Spreadsheets"),
        ("zip","Compressed"),
        ("rar","Compressed"),
        ("tar", "Compressed"),
        ("gz", "Compressed"),
        ("7z","Compressed"),
        ("dmg", "Applications"),
        ("exe", "Applications"),
        ("app", "Applications")
    ]);
    let files = read_folder(&mut base, ignore_hidden, ignore_symlink);
    if files.is_err() {
        return;
    }
    let files = files.unwrap();
    sort_files(files, map, base.to_str().unwrap());

    println!("Finished in {}ms", now.elapsed().as_millis());
}

fn read_folder(dir: &mut PathBuf, ignore_hidden: bool, ignore_symlink: bool) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut res: Vec<PathBuf> = Vec::new();


    match std::fs::read_dir(dir) {
        Ok(dir) => {
            dir.for_each(|entry| {
                let path = entry.unwrap().path();
                if !path.is_dir() {
                    let filename = path.file_name().unwrap();
                    if ignore_hidden && filename.as_bytes()[0] == b'.' {
                        return;
                    };

                    if ignore_symlink && path.is_symlink() {
                        return;
                    };
                    
                    println!("Found File: {:?}", path);
                    res.push(path);
                    
                }
            });
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(e)
        }
    }
    Ok(res)
}
