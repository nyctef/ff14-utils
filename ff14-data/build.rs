use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let csv_dir = Path::new(&out_dir);
    
    // Look for ffxiv-datamining repository 
    // First check if Nix provided it via environment variable
    let datamining_csv = if let Ok(nix_path) = env::var("FFXIV_DATAMINING_PATH") {
        Path::new(&nix_path).join("csv")
    } else {
        // Fall back to sibling directory for local development
        Path::new(&manifest_dir)
            .parent()
            .unwrap()
            .parent()
            .expect("Failed to get parent directory")
            .join("ffxiv-datamining")
            .join("csv")
    };
    
    if !datamining_csv.exists() {
        panic!(
            "ffxiv-datamining repository not found. Expected at: {:?}\n\
            For local development, clone the repository:\n\
            cd {}\n\
            git clone https://github.com/xivapi/ffxiv-datamining.git",
            datamining_csv,
            Path::new(&manifest_dir).parent().unwrap().display()
        );
    }
    
    // Copy required CSV files
    let files_to_copy = ["Recipe.csv", "RecipeLevelTable.csv", "Item.csv", "Materia.csv"];
    
    for file in &files_to_copy {
        let src = datamining_csv.join(file);
        let dst = csv_dir.join(file);
        
        if !src.exists() {
            panic!("Required CSV file {} not found at {:?}", file, src);
        }
        
        // Check if destination already exists and has same content
        if dst.exists() {
            let src_content = fs::read_to_string(&src)
                .unwrap_or_else(|e| panic!("Failed to read source {}: {}", file, e));
            let dst_content = fs::read_to_string(&dst)
                .unwrap_or_else(|e| panic!("Failed to read existing {}: {}", file, e));
            
            if src_content == dst_content {
                println!("{} already exists with correct content, skipping", file);
                continue;
            } else {
                panic!("Existing {} has different content than source - this shouldn't happen in a clean build", file);
            }
        }
        
        fs::copy(&src, &dst)
            .unwrap_or_else(|e| panic!("Failed to copy {}: {}", file, e));
        println!("Copied {}", file);
    }
}

