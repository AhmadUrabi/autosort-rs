use std::fs;
use std::io;
use std::io::BufRead;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::collections::HashMap;

use clap::Parser;

#[cfg(target_os = "windows")]
fn sort_files(files: Vec<PathBuf>, map: HashMap<String, String>, base: &str) {
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
fn sort_files(files: Vec<PathBuf>, map: HashMap<String, String>, base: &str) {
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
    ignore_symlink: Option<bool>,
    #[arg(short, long, value_name = "Configuration File")]
    config_file: Option<PathBuf>
}



fn read_config(config: PathBuf) -> Result<HashMap<String, String>, std::io::Error> {
    if !config.is_file() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Config file is not valid"));
    }

    // Parsing Configuration
    // File Format is as follows:
    // extenstion:category
    
    let file = fs::File::open(config)?;
    let reader = std::io::BufReader::new(file);
    let mut res: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let split: Vec<&str> = line.split(':').collect();
        if split.len() == 2 {
            res.insert(split[0].to_string(), split[1].to_string());
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid line format in config file"));
        }
    }

    Ok(res)
}


fn read_default_config() -> HashMap<String, String> {
    // Return default hardcoded Configuration
    HashMap::from([
        ("pdf".to_string(), "Documents".to_string()),
        ("txt".to_string(), "Documents".to_string()),
        ("docx".to_string(), "Documents".to_string()),
        ("doc".to_string(), "Documents".to_string()),
        ("ppt".to_string(), "Slides".to_string()),
        ("pptx".to_string(), "Slides".to_string()),
        ("png".to_string(), "Pictures".to_string()),
        ("jpg".to_string(), "Pictures".to_string()),
        ("jpeg".to_string(), "Pictures".to_string()),
        ("csv".to_string(), "Spreadsheets".to_string()),
        ("xlsx".to_string(), "Spreadsheets".to_string()),
        ("xls".to_string(), "Spreadsheets".to_string()),
        ("zip".to_string(), "Compressed".to_string()),
        ("rar".to_string(), "Compressed".to_string()),
        ("tar".to_string(), "Compressed".to_string()),
        ("gz".to_string(), "Compressed".to_string()),
        ("7z".to_string(), "Compressed".to_string()),
        ("dmg".to_string(), "Applications".to_string()),
        ("exe".to_string(), "Applications".to_string()),
        ("app".to_string(), "Applications".to_string()),
    ])
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

    let config: HashMap<String, String> = match args.config_file {
        Some(file) => {
            match read_config(file) {
                Ok(conf) => conf,
                Err(_e) => read_default_config()
            }
        },
        None => read_default_config()
    };

    // TODO: Extract to config file
    /*let map: HashMap<&str, &str> = HashMap::from([
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
    ]);*/
    let files = read_folder(&mut base, ignore_hidden, ignore_symlink);
    if files.is_err() {
        return;
    }
    let files = files.unwrap();
    sort_files(files, config, base.to_str().unwrap());

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
