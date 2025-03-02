# Cubiomes-rs
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/villevilli/cubiomes-rs/rust.yml)
![Crates.io Version](https://img.shields.io/crates/v/cubiomes?style=flat&label=crates.io%20cubiomes)
![Crates.io Version](https://img.shields.io/crates/v/cubiomes-sys?style=flat&label=crates.io%20cubiomes-sys)



A (hopefully) safe rust wrapper for the cubiomes library which mimics
minecraft biome generation for fast seed finding and previews of
minecraft worlds.

The cubiomes library is developed by Cubitect, and is available here: 
https://github.com/Cubitect/cubiomes

This repo contains:
- cubiomes-sys, bindgen generated bindings for the original cubiomes library
- cubiomes, a safe rust wrapper for cubiomes-sys

## Usage
See each crates own readme and docs.rs for usage
- [cubiomes](cubiomes/README.md)
- [cubiomes-sys](cubiomes-sys/README.md)

## Contribution
Feel free to open an issue or make a pr.

When cloning, remeber to initialize submodules with ``git submodule init && git submodule update``
Othervise cubiomes-sys will fail to build.

## License
The project is licensed under the [MIT](license.md) license, following cubiomes.