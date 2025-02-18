use std::env;
use std::fs;
use std::path::Path;
use dotenv::dotenv;

// Function to get excluded folders from environment variables or args
fn get_excluded_folders() -> Vec<String> {
    dotenv().ok();  // Load .env file if available

    // Check if "EXCLUDED_FOLDERS" exists in .env or environment variables
    let env_excluded = env::var("EXCLUDED_FOLDERS")
        .unwrap_or_else(|_| String::from("node_modules,build,bin,user-management-service,franchise-service,gradle,logs,customer-service,node_modules"));

    // Convert the comma-separated list into a vector
    env_excluded.split(',').map(|s| s.trim().to_string()).collect()
}

// Function to print directory tree recursively
fn print_tree(path: &Path, prefix: String, is_last: bool, excluded_folders: &[String]) {
    if let Some(name) = path.file_name() {
        let name = name.to_string_lossy();

        // Skip excluded folders
        if path.is_dir() && (excluded_folders.contains(&name.to_string()) || name.starts_with(".")) {
            return;
        }

        // Print the current entry
        let connector = if is_last { "└── " } else { "├── " };
        println!("{}{}{}", prefix, connector, name);

        if path.is_dir() {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            if let Ok(entries) = fs::read_dir(path) {
                let entries: Vec<_> = entries.flatten().collect();
                let count = entries.len();
                for (i, entry) in entries.into_iter().enumerate() {
                    print_tree(&entry.path(), new_prefix.clone(), i == count - 1, excluded_folders);
                }
            }
        }
    }
}

fn main() {
    // Get directory path from command-line args, default to current dir
    let args: Vec<String> = env::args().collect();
    let dir_path = args.get(1).map_or(".", |s| s.as_str());

    // Get excluded folders from environment or args
    let excluded_folders = get_excluded_folders();

    let path = Path::new(dir_path);
    if path.exists() {
        println!("Tree structure of: {}\n", path.display());
        print_tree(path, "".to_string(), true, &excluded_folders);
    } else {
        eprintln!("Error: Path '{}' does not exist.", dir_path);
    }
}
