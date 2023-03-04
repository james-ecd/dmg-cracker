# DMG Cracker
[![Linting and tests](https://github.com/james-ecd/dmg-cracker/actions/workflows/testing-and-linting.yml/badge.svg?branch=main)](https://github.com/james-ecd/dmg-cracker/actions/workflows/testing-and-linting.yml)
<img src="https://img.shields.io/crates/v/dmg-cracker.svg"/>
[![codecov](https://codecov.io/gh/james-ecd/dmg-cracker/branch/main/graph/badge.svg?token=C96XYRSKI4)](https://codecov.io/gh/james-ecd/dmg-cracker)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

CLI for performing dictionary attacks on encrypted Apple Disk Image files (.dmg)

While this tool offers a solution for accessing encrypted disk images,
it is important to note that it is intended for personal use only and 
should not be used for illegal purposes. The author assumes no 
responsibility for any misuse of the tool and it is the responsibility 
of the user to comply with all applicable laws and regulations.

<p align="center">
  <img src="./resources/demo.gif" alt="Demo GIF">
</p>

## Installation
`cargo install dmg-cracker`

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
