# Cubiomes-rs
![docs.rs](https://img.shields.io/docsrs/cubiomes)

A (hopefully) safe rust wrapper to generate information about minecraft seeds
using the cubiomes library.

The original cubiomes library is available here: https://github.com/cubitect/cubiomes/

## Installation
Use cargo to install cubiomes ``cargo add cubiomes``. Note that, to build 
cubiomes you will also need a c compiler. For further instructions if building
cubiomes-sys doesn't work out of the box consult the `cc` crates documentation.
or file an issue

## Usage
For documentation and usage see [docs.rs](https://docs.rs/cubiomes/latest/cubiomes/). 
Please also see the crate on [crates.io](https://crates.io/crates/cubiomes).

## Contributing
The currently the library support biome and structure generation. 
Feel free to submit issues or pull requests for the project. For bigger changes
please open an issue to discuss them first.

## Cargo features
All features are enabled by default.
- ``cc_build``, use the cc create for building cubiomes. If not enabled
cubiomes is built with make instead.
- ``image`` Use the image crate to generate images of areas.

## License
cubiomes-rs is licensed under the [MIT](license.md) license, following cubiomes.