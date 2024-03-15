mod cli;

use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process;
use walkdir::WalkDir;

/// Represents the structure of the manifest file, which maps file paths to their SHA256 hashes.
#[derive(Debug, Deserialize, Serialize)]
struct Manifest {
    files: HashMap<String, String>,
}

/// Main entry point of the program.
fn main() {
    match run() {
        Ok(_) => println!("Verification successful."),
        Err(_) => {
            println!("Verification failed.");
            process::exit(1);
        }
    }
}

/// Performs the program's operations based on the user's input.
/// 
/// Returns:
/// - Ok(()) if the operation (verify or generate) completes successfully.
/// - Err(String) with an error message if an error occurs.
fn run() -> Result<(), String> {

    // Parse command-line arguments.
    let matches = cli::parse_arguments();

    // Determine the subcommand and execute the corresponding operation.
    match matches.subcommand() {
        Some(("verify", sub_m)) => {
            // Extract paths from arguments for the verification operation.
            let manifest_path: PathBuf = sub_m.value_of("manifest").unwrap().into();
            let directory_path: PathBuf = sub_m.value_of("directory").unwrap().into();
            // Perform verification and handle potential errors.
            verify_operation(&manifest_path, &directory_path)
                .map_err(|_| "Verification failed due to an unexpected error.".to_string())?;
            println!("Verification successful.");
        }
        Some(("generate", sub_m)) => {
            // Extract paths from arguments for the manifest generation operation.
            let directory_path: PathBuf = sub_m.value_of("directory").unwrap().into();
            let output_path: PathBuf = sub_m.value_of("output").unwrap().into();
            // Perform manifest generation and handle potential errors.
            generate_operation(&directory_path, &output_path)
                .map_err(|e| format!("Manifest generation failed: {}", e.to_string()))?;
            println!("Manifest generated successfully.");
        }
        // Handle case where no valid subcommand is provided.
        _ => return Err("No valid subcommand provided. Use 'verify' or 'generate'.".to_string()),
    }

    Ok(())
}

/// Reads the specified manifest file and parses it into a `Manifest` struct.
///
/// Args:
/// - `manifest_path`: The path to the manifest file.
///
/// Returns:
/// - `Result<Manifest, std::io::Error>`: The parsed manifest or an error if reading or parsing failed.
fn read_manifest(manifest_path: &PathBuf) -> Result<Manifest, std::io::Error>
{
    // Attempt to open and read the manifest file.
    let manifest_file: File = File::open(manifest_path)?;
    let reader: BufReader<File> = BufReader::new(manifest_file);
    // Deserialize the JSON content into a Manifest struct.
    let manifest: Manifest = from_reader(reader)?;
    Ok(manifest)
}

/// Performs the verification operation by checking if files in the directory match the manifest.
/// 
/// Args:
/// - `manifest_path`: Path to the manifest file.
/// - `directory_path`: Path to the directory to be verified.
/// 
/// Returns:
/// - Ok(()) if verification is successful, Err(bool) if not.
fn verify_operation(manifest_path: &PathBuf, directory_path: &PathBuf) -> Result<(), bool> {
    let manifest: Manifest = read_manifest(manifest_path).map_err(|_| true)?;
    verify_directory(directory_path, &manifest)
}

/// Generates a manifest file based on the files found in the specified directory.
/// 
/// Args:
/// - `directory_path`: Path to the directory from which to generate the manifest.
/// - `output_path`: Path where the generated manifest file should be saved.
/// 
/// Returns:
/// - Ok(()) if generation is successful, Err(io::Error) if an error occurs during file operations.
fn generate_operation(directory_path: &PathBuf, output_path: &PathBuf) -> Result<(), std::io::Error> {
    let mut manifest: Manifest = Manifest { files: HashMap::new() };

    for entry in WalkDir::new(directory_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file()) {
            let path = entry.path();
            if let Some(relative_path) = path.strip_prefix(directory_path).ok().and_then(|p| p.to_str()) {
                let hash = hash_file(path)?;
                manifest.files.insert(relative_path.replace("\\", "/"), hash);
            }
    }

    let manifest_file = File::create(output_path)?;
    to_writer_pretty(&manifest_file, &manifest)?;
    Ok(())
}

/// Verifies each file listed in the manifest exists in the directory and matches the recorded hash.
///
/// Args:
/// - `directory_path`: Path to the directory containing the files to verify.
/// - `manifest`: The manifest containing expected file hashes.
///
/// Returns:
/// - `Result<(), bool>`: Ok if all files match, Err otherwise.
fn verify_directory(directory_path: &PathBuf, manifest: &Manifest) -> Result<(), bool> {
    // Flag to track if all files match their manifest entry.
    let mut all_files_match = true;

    // Iterate through each entry in the manifest.
    for (expected_path_str, expected_hash) in manifest.files.iter() {
        let file_path = directory_path.join(expected_path_str);

        // Proceed only if the file exists.
        if file_path.exists() {
            // Compute the hash of the file.
            let hash: String = hash_file(&file_path).map_err(|_| true)?;

            // Check if the computed hash matches the expected hash.
            if &hash == expected_hash {
                continue;
            } else {
                println!("Mismatched hash for file: {}", expected_path_str);
                println!("Expected: {}", expected_hash);
                println!("Found:    {}", hash);
                all_files_match = false;
            }
        } else {
            println!("Missing file in directory: {}", expected_path_str);
            all_files_match = false;
        }
    }

    // Return success only if all files matched.
    if all_files_match {
        Ok(())
    } else {
        Err(true)
    }
}

/// Calculates the SHA256 hash of a file at a given path.
///
/// Args:
/// - `path`: A path reference to the file to hash.
///
/// Returns:
/// - `std::io::Result<String>`: The hexadecimal representation of the file hash, or an error.
fn hash_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    // Read the file content in chunks and update the hash.
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break; // End of file reached.
        }
        hasher.update(&buffer[..count]);
    }

    // Return the final hash in hexadecimal format.
    Ok(format!("{:x}", hasher.finalize()))
}
