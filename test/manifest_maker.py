import hashlib
import json
import os

def calculate_sha256_checksum(file_path):
    """Calculate SHA256 checksum for a file."""
    sha256_hash = hashlib.sha256()
    with open(file_path, "rb") as f:
        # Read and update hash in chunks of 4K
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    return sha256_hash.hexdigest()

def generate_manifest(folder_path):
    """Generate a manifest.json file with paths and SHA256 checksums of all files."""
    manifest = {}
    for root, dirs, files in os.walk(folder_path):
        for filename in files:
            file_path = os.path.join(root, filename)
            checksum = calculate_sha256_checksum(file_path)
            # Use relative path for file in manifest to make it more universally applicable
            relative_path = os.path.relpath(file_path, folder_path)
            manifest[relative_path] = checksum
    
    with open("manifest.json", "w") as json_file:
        json.dump(manifest, json_file, indent=4)

# Example usage
folder_path = "SomeProject"
generate_manifest(folder_path)
