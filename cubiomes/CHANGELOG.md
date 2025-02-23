# Changelog

Major changes are documented here

The project follows semver

## [Unreleased]

## [0.2.1] - 2024-02-23

### Removed
- Unnessecary re-export

## [0.2.0] - 2024-02-22

### Added

- Changelog.md for tracking changes
- MinecraftCoordinates to represent x and z position inside a minecraft world
- Adds functions to access the underlying pointer to the generator
- Structure generation to generator
- StructureRegion for help with structure generation
- Adds stronghold generation to generator

### Changed

- renamed Generator.buffer() -> Generator.as_vec()

## [0.1.3] - 2024-02-17

### Fixed

- Fixes a broken link in the docs

## [0.1.2] - 2024-02-17

Initial release on crates.io