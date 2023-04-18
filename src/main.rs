use std::fs;
use std::path::Path;

fn get_folder_size(path: &Path) -> u64 {
    let mut size = 0;
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            size += fs::metadata(&path).unwrap().len();
        } else {
            size += get_folder_size(&path);
        }
    }
    size
}

fn recursive_print(path: &Path, level: usize, threshhold: u64) {
    let mut entries: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
    entries.sort_by(|a, b| {
        let a_dir = a.path().is_dir();
        let b_dir = b.path().is_dir();
        if a_dir && !b_dir {
            std::cmp::Ordering::Less
        } else if !a_dir && b_dir {
            std::cmp::Ordering::Greater
        } else {
            b.metadata()
                .unwrap()
                .len()
                .cmp(&a.metadata().unwrap().len())
        }
    });

    for entry in entries {
        let path = entry.path();
        let path_name = path.file_name().unwrap().to_string_lossy()
            + if path.is_dir() { " (Directory)" } else { "" };
        let size = {
            match path.is_dir() {
                true => get_folder_size(&path),
                false => fs::metadata(&path).unwrap().len(),
            }
        };

        if size < threshhold {
            continue;
        }
        print!("{:\t<1$}", "", level);

        print!("{}", path_name);

        match size {
            1_000_000_000.. => print!(" (Size: {:.2} GB)", size as f64 / 1_000_000_000.0),
            1_000_000.. => print!(" (Size: {:.2} MB)", size as f64 / 1_000_000.0),
            _ => print!(" (Size: {} bytes)", size),
        };

        println!();
        if path.is_dir() {
            recursive_print(&path, level + 1, threshhold);
        }
    }
}

fn main() {
    let maindirpath = "../";
    let maindir = Path::new(maindirpath);
    let threshhold = 1_000;
    if maindir.is_dir() {
        println!("Directory: {}", maindir.display());
        let size = get_folder_size(maindir);
        match size {
            1_000_000_000.. => println!("Size: {:.2} GB", size as f64 / 1_000_000_000.0),
            1_000_000.. => println!("Size: {:.2} MB", size as f64 / 1_000_000.0),
            _ => println!("Size: {} bytes", size),
        };
        println!("Contents (sorted by size):");
        recursive_print(maindir, 0, threshhold);
    }
}
