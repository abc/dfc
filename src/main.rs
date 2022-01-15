use std::{collections::HashMap, env, fs, time::Instant};

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Count the number of occurrances of a filename in a directory structure.");
        eprintln!();
        eprintln!("Syntax: {} <file path> <depth>", args[0]);
        eprintln!();
        eprintln!("file path:\t The path to begin counting in.");
        eprintln!("depth:\t\t The level of recursion to use.");
        eprintln!("\t\t With 0, only the file path itself and no subdirectories will be checked.");
        eprintln!("\t\t With 1, the file path and one level of subfolders are checked, etc.");
        return;
    }
    let file_path = &args[1];
    let depth = args[2].parse();
    let depth = match depth {
        Err(_) => {
            eprintln!("ERR: `depth` parameter must be an integer");
            return;
        }
        Ok(depth) => depth,
    };

    let mut files: HashMap<String, i32> = HashMap::new();
    let result = get_files(&mut files, file_path, 0, depth, &start);
    match result {
        Err(_) => {
            eprintln!("ERR: An error occurred while getting files.");
            return;
        }
        Ok(_) => {}
    }

    let unique_count = files.iter().len() as i32;
    let total_count: i32 = files.iter().map(|f| f.1).sum();
    let duplicate_count = total_count - unique_count;

    eprintln!("[{:06}s] Processing complete", start.elapsed().as_secs());

    for file in files {
        println!("{}: {}", file.0, file.1);
    }

    // Write summary to STDOUT.
    println!();
    println!("SUMMARY:");
    println!();
    println!("TOTAL: {: >15}", total_count);
    println!("UNIQUE: {: >14}", unique_count);
    println!("DUPLICATES: {: >10}", duplicate_count);
}

fn get_files(
    files: &mut HashMap<String, i32>,
    path: &String,
    level: i32,
    max_depth: i32,
    start: &Instant,
) -> Result<i32, i32> {
    let paths = fs::read_dir(path);
    let paths = match paths {
        Ok(paths) => paths,
        Err(error) => {
            eprintln!("ERR: Unable to open directory {} - {}", path, error);
            return Err(1);
        }
    };
    let mut count = 0;
    eprintln!(
        "[{:06}s] Started processing for {}",
        start.elapsed().as_secs(),
        path
    );
    for path in paths {
        count += 1;
        let path = path;
        let path = match path {
            Ok(path) => path,
            Err(error) => {
                eprintln!("ERR: Unable to read path - {}", error);
                return Err(1);
            }
        };
        let path_str = format!("{}", path.path().display());
        let file_type = path.file_type();
        let file_type = match file_type {
            Ok(file_type) => file_type,
            Err(error) => {
                eprintln!("ERR: Unable to read file type - {}", error);
                return Err(1);
            }
        };

        if file_type.is_dir() && level < max_depth {
            let result = get_files(files, &path_str, level + 1, max_depth, start);
            match result {
                Ok(_) => {}
                Err(_) => eprintln!("ERR: An error occurred while getting file counts."),
            };
        } else if file_type.is_file() {
            let filepath = path.file_name().into_string();
            let filepath = match filepath {
                Ok(filepath) => filepath,
                Err(error) => {
                    eprintln!(
                        "ERR: Filename ({:?}) could not be converted into a UTF-8 string.",
                        error
                    );
                    return Err(1);
                }
            };
            if files.contains_key(&filepath) {
                *files.get_mut(&filepath).unwrap() += 1;
            } else {
                files.insert(filepath, 1);
            }
        }
    }
    eprintln!(
        "[{:06}s] Finished handling {} - {} files.",
        start.elapsed().as_secs(),
        path,
        count
    );
    Ok(0)
}
