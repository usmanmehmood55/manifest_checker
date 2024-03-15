use clap::{App, Arg};
use serde::Deserialize;
use serde_json::from_reader;
use sha2::{Digest, Sha256};
use std::process;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

/// Represents the expected structure of the manifest file.
#[derive(Debug, Deserialize)]
struct Manifest {
    files: HashMap<String, String>,
}

/// The entry point for the verification program.
fn main() {
    match run() {
        Ok(_) => println!("Verification successful."),
        Err(_) => {
            println!("Verification failed.");
            process::exit(1);
        }
    }
}

/// Performs the main steps of the program: parsing arguments, reading the manifest, and verifying the directory.
fn run() -> Result<(), bool> {
    // Parse command line arguments to get paths for the manifest and the directory.
    let (manifest_path, directory_path) = parse_arguments();

    // Read and parse the manifest file.
    let manifest = read_manifest(&manifest_path).map_err(|_| true)?;

    // Verify the directory against the manifest.
    verify_directory(&directory_path, &manifest).map_err(|_| true)?;

    // If verification is successful, print a success message.
    println!("Success: All files in the directory match the manifest entries!");

    Ok(())
}

/// Parses command-line arguments, extracting paths for the manifest and the target directory.
///
/// Returns:
/// - `(PathBuf, PathBuf)`: Tuple containing the manifest path and the directory path.
fn parse_arguments() -> (PathBuf, PathBuf)
{
    let matches = App::new("Manifest Checker")
        .version("0.1.0")
        .author("Usman Mehmood (usmanmehmood55@gmail.com)")
        .about("Verifies files in a directory against a checksum manifest")
        .arg(Arg::with_name("manifest")
            .short('m')
            .long("manifest")
            .value_name("FILE")
            .help("Sets the path to the manifest file")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("directory")
            .short('d')
            .long("directory")
            .value_name("DIR")
            .help("Sets the input directory path")
            .takes_value(true)
            .required(true))
        .get_matches();

    // Extract and return the manifest and directory paths from the arguments.
    let manifest_path: PathBuf  = matches.value_of("manifest").unwrap().into();
    let directory_path: PathBuf = matches.value_of("directory").unwrap().into();

    (manifest_path, directory_path)
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
    let manifest_file: File     = File::open(manifest_path)?;
    let reader: BufReader<File> = BufReader::new(manifest_file);
    let manifest: Manifest      = from_reader(reader).expect("Error parsing JSON");

    Ok(manifest)
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
