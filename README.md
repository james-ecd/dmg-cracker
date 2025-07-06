# DMG Cracker

[![CI](https://github.com/james-ecd/dmg-cracker/actions/workflows/testing-and-linting.yml/badge.svg)](https://github.com/james-ecd/dmg-cracker/actions/workflows/testing-and-linting.yml)
[![Crates.io](https://img.shields.io/crates/v/dmg-cracker)](https://crates.io/crates/dmg-cracker)
[![Downloads](https://img.shields.io/crates/d/dmg-cracker)](https://crates.io/crates/dmg-cracker)
[![codecov](https://codecov.io/gh/james-ecd/dmg-cracker/branch/main/graph/badge.svg?token=C96XYRSKI4)](https://codecov.io/gh/james-ecd/dmg-cracker)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.72.1%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)](https://www.apple.com/macos)

CLI for performing dictionary attacks on encrypted Apple Disk Image files (.dmg)

While this tool offers a solution for accessing encrypted disk images,
it is important to note that it is intended for personal use only and 
should not be used for illegal purposes. The author assumes no 
responsibility for any misuse of the tool and it is the responsibility 
of the user to comply with all applicable laws and regulations.

<!--suppress HtmlDeprecatedAttribute -->
<p align="center">
  <img src="./resources/demo.gif" alt="Demo GIF">
</p>

## Installation
`cargo install dmg-cracker`

## Usage

### Basic Usage
```bash
dmg-cracker -p <password-list.txt> -d <encrypted-file.dmg> -t <thread-count>
```

**Required Arguments:**
- `-p, --password-list-path`: Path to a password list file (.txt or .csv format)
- `-d, --dmg-path`: Path to the encrypted DMG file

**Optional Arguments:**
- `-t, --thread-count`: Number of threads to use (defaults to number of logical CPU cores)

### Creating a Password List

**Text File Format (.txt):**
Create a text file with one password per line:

```bash
# Example: passwords.txt
password
123456
qwerty
letmein
admin
password123
welcome
```

**CSV Format (.csv):**
Create a CSV file with passwords in the first column:

```bash
# Example: passwords.csv
password1,common
123456,weak
qwerty,keyboard
letmein,simple
admin,default
password123,variation
welcome,greeting
```

The tool automatically detects file format based on extension and uses only the first column for CSV files.

You can use existing wordlists like:
- [SecLists](https://github.com/danielmiessler/SecLists) - Comprehensive password lists
- [rockyou.txt](https://github.com/brannondorsey/naive-hashcat/releases/download/data/rockyou.txt) - Popular password dictionary

### Examples

**Using text file:**
```bash
# Create a simple password list
echo -e "password\n123456\nletmein\npassword123" > passwords.txt

# Run the cracker
dmg-cracker -p passwords.txt -d encrypted.dmg -t 4
```

**Using CSV file:**
```bash
# Create a CSV password list
echo -e "password,type\n123456,weak\nletmein,simple\npassword123,variation" > passwords.csv

# Run the cracker
dmg-cracker -p passwords.csv -d encrypted.dmg -t 4
```

The tool will display progress bars for each thread and stop when the correct password is found.

## Upcoming features
- investigate implementing AES decrpytion manually. Current road block being a method of extracting the key generation salt from the dmg file headers
- support for testing different permutations of a given password list
- increase test coverage

## Testing / Contributing
Any contributions or issue raising is welcomed. If you wish to contribute then:
1. fork/clone this repo
2. make changes on a branch taken from main
3. submit a pull request against main

Pull requests will be blocked from merging automatically if:
- there are failing tests
- linting rules have been violated.

## Updating package
- bump version number in `cargo.toml` and `cargo.lock`
- commit version bump
- create new release on github
- `cargo publish`
