# ME1 UPK Asset Extractor

A Rust CLI tool for extracting models, animations, and textures from Mass Effect 1 UPK files.

## Overview

This tool recursively scans directories for UPK (Unreal Package) files from Mass Effect 1 (which uses Unreal Engine 3) and extracts:

- **Models**: Skeletal and static meshes (→ glTF format)
- **Animations**: Animation sequences and sets (→ glTF format)
- **Textures**: Texture2D assets (→ PNG/DDS format)

## Building

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Compile

```bash
cargo build --release
```

The binary will be in `target/release/me1-upk-extractor`

## Usage

```bash
me1-upk-extractor --input <INPUT_DIR> --output <OUTPUT_DIR> [OPTIONS]
```

### Arguments

- `--input, -i`: Path to directory containing UPK files (required)
- `--output, -o`: Path to output directory for extracted assets (required)
- `--verbose, -v`: Enable verbose logging (optional)
- `--asset-types, -a`: Extract only specific types: `models`, `animations`, `textures` (optional, comma-separated)

### Examples

```bash
# Extract all assets from ME1 game files
me1-upk-extractor --input "C:\\Mass Effect\\CookedPC" --output "./extracted_assets"

# Verbose output
me1-upk-extractor -i ./game_files -o ./output -v

# Extract only textures
me1-upk-extractor -i ./game_files -o ./output -a textures
```

## Output Structure

Assets are organized into subdirectories:

```
output/
├── models/
│   ├── Mesh_Name_1.gltf
│   ├── Mesh_Name_2.gltf
│   └── ...
├── animations/
│   ├── Animation_Name_1.gltf
│   ├── Animation_Name_2.gltf
│   └── ...
└── textures/
    ├── Texture_Name_1.png
    ├── Texture_Name_2.png
    └── ...
```

## Features

✅ Recursive directory scanning for UPK files
✅ UPK file parsing (header, name tables, export/import tables)
✅ Asset type detection and classification
✅ Error recovery (continues on individual file failures)
✅ Verbose logging support
✅ Organized output directory structure

## TODO / In Progress

- [ ] UE3 Skeletal Mesh format parsing → glTF conversion
- [ ] UE3 Static Mesh format parsing → glTF conversion
- [ ] UE3 Animation format parsing → glTF conversion
- [ ] Texture2D DDS/compressed format decoding → PNG conversion
- [ ] Material extraction and reference preservation
- [ ] Skeleton/bone hierarchy extraction
- [ ] Test suite with sample ME1 assets
- [ ] Performance optimization for large UPK files

## Architecture

### Modules

- **main.rs**: CLI entry point and argument parsing
- **upk.rs**: UPK file format parsing (header, tables, exports)
- **extractor.rs**: Asset extraction logic and filesystem operations
- **asset_types.rs**: Asset type definitions and UE3 class names

### Data Flow

```
Input Directory
    ↓
Walk file tree for .upk files
    ↓
For each UPK file:
  - Load and parse header
  - Parse name table, export table, import table
  - Iterate exports and identify asset types
  - Extract raw asset data
  - Convert to output format (glTF, PNG, etc.)
    ↓
Output Directory
```

## References

- [UPK File Format Documentation](https://github.com/gildor2/UE3)
- [glTF Specification](https://www.khronos.org/gltf/)
- [Unreal Engine 3 Asset Format](https://www.unrealengine.com/en-US/)

## License

MIT

## Disclaimer

This tool is for educational and preservation purposes. Mass Effect 1 and its assets are the intellectual property of BioWare and Electronic Arts. Ensure you have the right to extract and use game assets before proceeding.
