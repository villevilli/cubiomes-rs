# Changelog

Major changes are documented here

The project follows semver

## [Unreleased]

## [0.3.3]

### Changed
- Heighmap generation should takke a reference, not owning the noise

## [0.3.2] [YANKED]

### Added
- Surfacenoise

### Changed
- Heightmap generation now requires explicit noise

## [0.3.1] - 2025-03-05


## [0.3.0] - 2025-03-02

### Added
- Crate feature image for image generation using the image crate
- Benchmarking using criterion
- Colormap for mapping biomes to colors
- added Cache::new(..) in favor of Generator::new_cache(..)

### Changed
- Marked the generator Send And Sync
- Derives Ord and PartialOrd where it makes sense
- Moved structures::Strongholds to structures::strongholds::StrongholdIter
- Moved generator::GeneratorError to its own module error generator::error::GeneratorError
- Cache now fills itself eagerly to allow for more ergonomic use

### Removed 
- Generator::new_cache(..)


## [0.2.1] - 2025-02-23

### Removed
- Unnessecary re-export

## [0.2.0] - 2025-02-22

### Added

- Changelog.md for tracking changes
- MinecraftCoordinates to represent x and z position inside a minecraft world
- Adds functions to access the underlying pointer to the generator
- Structure generation to generator
- StructureRegion for help with structure generation
- Adds stronghold generation to generator

### Changed

- renamed Generator.buffer() -> Generator.as_vec()

## [0.1.3] - 2025-02-17

### Fixed

- Fixes a broken link in the docs

## [0.1.2] - 2025-02-17

Initial release on crates.io