use glob::glob;
use regex::Regex;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

// Function to update file content based on the import path replacements
fn update_file_content(src_file: &Path, replacements: &[(String, String)]) -> io::Result<String> {
    let content = fs::read_to_string(src_file)?;
    let mut updated_content = content.clone();

    for (old_alias, new_alias) in replacements {
        // Enhanced regex pattern to handle quotes, optional whitespace, and varying formats
        let re = Regex::new(&format!(
            r#"['"]\s*{}\s*([^'"]*)['"]"#,
            regex::escape(old_alias)
        ))
        .unwrap();

        // Replace matching import paths
        updated_content = re
            .replace_all(&updated_content, |caps: &regex::Captures| {
                format!("'{}{}'", new_alias, &caps[1])
            })
            .to_string();
    }

    Ok(updated_content)
}

// Function to copy the updated content to the target directory
fn copy_file(
    updated_content: &str,
    src_file: &Path,
    target_dir: &Path,
    src_dir: &str,
) -> io::Result<()> {
    // Determine the new file path in the target directory
    let relative_path = src_file.strip_prefix(src_dir).unwrap();
    let new_path = target_dir.join(relative_path);

    // Create the target directory if it doesn't exist
    if let Some(parent) = new_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the updated content to the new file path
    let mut file = fs::File::create(&new_path)?;
    file.write_all(updated_content.as_bytes())?;
    println!("Updated and copied: {:?}", new_path);

    Ok(())
}

// Function to process all files in the source directory
fn process_files(
    src_dir: &str,
    target_dir: &str,
    replacements: &[(String, String)],
) -> io::Result<()> {
    let target_dir_path = Path::new(target_dir);

    // Find all .tsx files in the source directory
    for entry in glob(&format!("{}/**/*.tsx", src_dir)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // Update the file content
                let updated_content = update_file_content(&path, replacements)?;

                // Copy the updated content to the target directory
                copy_file(&updated_content, &path, target_dir_path, src_dir)?;
            }
            Err(e) => println!("Error reading file: {:?}", e),
        }
    }
    Ok(())
}

fn main() {
    // Set the source and target directories without trailing slashes
    let src_dir = "/home/weiying-chen/node/comps/src/components"; // Source directory
    let target_dir = "/home/weiying-chen/node/aeonverse/packages/ui/src/custom"; // Target directory

    // Define the import path replacements
    let replacements = vec![
        ("@/components".to_string(), "@repo/ui/custom".to_string()),
        ("@/utils".to_string(), "@repo/ui/lib/utils".to_string()),
    ];

    // Run the file processing function
    match process_files(src_dir, target_dir, &replacements) {
        Ok(_) => println!("All files processed successfully."),
        Err(e) => eprintln!("Error processing files: {:?}", e),
    }
}
