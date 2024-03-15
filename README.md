# Manifest Checker

Manifest Checker is a Rust-based command-line tool designed to ensure the integrity
of files within a specified directory by comparing them against a manifest file.
This manifest file contains the expected SHA256 hashes of the files. Manifest Checker
is can be useful in environments where ensuring the authenticity and integrity of
firmware or software is critical.

## Features

- Manifest Verification: Compares files in a directory against expected hashes
  listed in a manifest file.
- SHA256 Hashing: Utilizes SHA256 hashing to ensure the integrity and authenticity
  of each file.
- Customizable Paths: Allows for specification of both the manifest file and the
  directory to verify via command-line arguments.

## Installation

To install and run Manifest Checker, ensure you have Rust and Cargo installed on
your system. Follow these steps:

### Clone the repository

```bash
git clone https://github.com/usmanmehmood55/manifest_checker
cd manifest_checker
```

### Build the project

It can be built wherever, but it is recommended to open this in a container,
the files are provided in [`.devcontainer`](.devcontainer) folder.

```bash
cargo build --release
```

The executable will be available in `./target/release/`

### Cross Compilation

To run this app on a RaspberryPi4. cross-compile it using this command.

```bash
cargo build --target=armv7-unknown-linux-gnueabihf --release
```

The executable will be available in `./target/release/armv7-unknown-linux-gnueabihf/release`

## Usage

To use Manifest Checker, you need a JSON manifest file containing the expected
SHA256 hashes for the files in your directory. The manifest file should follow
this format:

```json
{
  "files": {
    "relative/path/to/file1": "sha256hash1",
    "relative/path/to/file2": "sha256hash2"
  }
}
```

Run the verification with the following command:

```bash
./target/release/manifest_checker -m path/to/manifest.json -d path/to/directory
```

- `-m` or `--manifest`: Specifies the path to the manifest file.
- `-d` or `--directory`: Specifies the path to the directory containing the files
  to verify.

### Exit Codes

It returns:

- `0` if all checksums match.
- `1` if there is any mismatch or a file in the manifest is not found in the directory.

### Example

```bash
./target/release/manifest_checker -m manifest.json -d firmware/
```
