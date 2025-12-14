# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2025-12-14

### Added

- `get_id_meaning()` function to extract meaning from Thai National ID
  - Person type (1-8) with Thai and English descriptions
  - Province code and name (77 provinces)
  - Amphoe/district code and name (918 districts)
  - Checksum validity
- Python type stubs (`konjingjing.pyi`) for IDE support
- WASM package (`konjingjing-wasm`) for browser usage
- Documentation site with MkDocs

### Changed

- Bumped all packages to v1.0.0

## [0.2.0] - 2024-12-13

### Added

- Initial release
- `verify_id()` function to validate Thai National ID checksum
- Support for Rust, Python, and Node.js
