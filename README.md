# RWX Toolchain (Rust Workspace)

This project is an independent Rust-based toolchain for working with .rwx geometry files.  
It is not affiliated with Delta Worlds, ActiveWorlds, or any similar platform.  
The goal is to provide a modern, open, and reliable pipeline for parsing RWX files and converting them into formats suitable for modern engines.

## Workspace Structure

```
dw/
 ├── rwx_parser   - RWX file parser
 ├── rwx_scene    - Engine-agnostic scene model
 ├── rwx_export   - JSON and glTF exporters
 ├── rwx_tool     - Command-line conversion tool
 └── Cargo.toml   - Workspace configuration
```

## Crate Overview

### rwx_parser
Reads RWX text files into a structured Rust AST.  
Currently supports basic geometry.  
Full RWX grammar support is planned.

### rwx_scene
Defines a unified scene representation used by exporters.  
Includes basic object and mesh structures.

### rwx_export
Exports scenes to JSON and glTF.  
Additional export features will be added as scene support grows.

### rwx_tool
Command-line interface.  
Example usage:

```
rwx_tool to-json input.rwx output.json
rwx_tool to-gltf input.rwx output.gltf
```

## Build Instructions

```
git clone https://github.com/BronBron-Commits/dw
cd dw
cargo build
```

## Project Goals

- Provide a modern Rust implementation for working with RWX files  
- Bridge RWX data into formats compatible with current engines  
- Maintain modular crates for clean development and expansion  
- Allow external tools (like Unity importers) to consume exported JSON or glTF without depending on RWX directly

## License

MIT License.  
RWX file format is used for compatibility and research purposes only.