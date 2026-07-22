use anyhow::{anyhow, Result};
use binrw::{binrw, BinRead};
use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Bytes;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

/// UPK file header
#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct UPKHeader {
    pub signature: u32, // Should be 0x9E2A83C1
    pub version: u16,
    pub license_mode: u16,
    pub package_flags: u32,
    pub name_count: u32,
    pub name_offset: u32,
    pub export_count: u32,
    pub export_offset: u32,
    pub import_count: u32,
    pub import_offset: u32,
}

impl UPKHeader {
    pub const SIGNATURE: u32 = 0x9E2A83C1;
}

/// Export table entry
#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct ExportEntry {
    pub class_index: i32,
    pub super_index: i32,
    pub package_index: i32,
    pub object_name_index: u32,
    pub object_flags: u32,
    pub serial_size: u32,
    pub serial_offset: u32,
}

/// Import table entry
#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct ImportEntry {
    pub package_index: i32,
    pub class_name_index: u32,
    pub package_index_2: i32,
    pub object_name_index: u32,
}

/// Represents a loaded UPK file
pub struct UPKFile {
    pub path: std::path::PathBuf,
    pub header: UPKHeader,
    pub names: Vec<String>,
    pub exports: Vec<ExportEntry>,
    pub imports: Vec<ImportEntry>,
    file_data: Bytes,
}

impl UPKFile {
    /// Load a UPK file from disk
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(&path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        let bytes = Bytes::from(data);

        Self::from_bytes(bytes, path.as_ref().to_path_buf())
    }

    /// Parse UPK from bytes
    pub fn from_bytes(bytes: Bytes, path: std::path::PathBuf) -> Result<Self> {
        use std::io::Cursor;

        let mut cursor = Cursor::new(&bytes);

        // Read header
        let header = UPKHeader::read(&mut cursor)?;

        if header.signature != UPKHeader::SIGNATURE {
            return Err(anyhow!(
                "Invalid UPK signature: {:#x}",
                header.signature
            ));
        }

        // Read name table
        cursor.seek(std::io::SeekFrom::Start(header.name_offset as u64))?;
        let mut names = Vec::with_capacity(header.name_count as usize);
        for i in 0..header.name_count {
            match cursor.read_i32::<LittleEndian>() {
                Ok(name_length) => {
                    if name_length > 0 && name_length < 1024 {
                        let mut name_bytes = vec![0u8; name_length as usize];
                        if cursor.read_exact(&mut name_bytes).is_ok() {
                            // Remove null terminator if present
                            if name_bytes.last() == Some(&0) {
                                name_bytes.pop();
                            }
                            let name = String::from_utf8_lossy(&name_bytes).to_string();
                            names.push(name);
                        } else {
                            log::warn!("Failed to read name {} data", i);
                            names.push(format!("Unknown_{}", i));
                        }
                    } else {
                        names.push(format!("Invalid_{}", i));
                    }
                }
                Err(e) => {
                    log::warn!("Failed to read name {} length: {}", i, e);
                    names.push(format!("Error_{}", i));
                }
            }
        }

        // Read import table
        cursor.seek(std::io::SeekFrom::Start(header.import_offset as u64))?;
        let mut imports = Vec::with_capacity(header.import_count as usize);
        for _ in 0..header.import_count {
            match ImportEntry::read(&mut cursor) {
                Ok(entry) => imports.push(entry),
                Err(e) => {
                    log::warn!("Failed to read import entry: {}", e);
                    break; // Stop reading imports on error
                }
            }
        }

        // Read export table
        cursor.seek(std::io::SeekFrom::Start(header.export_offset as u64))?;
        let mut exports = Vec::with_capacity(header.export_count as usize);
        for _ in 0..header.export_count {
            match ExportEntry::read(&mut cursor) {
                Ok(entry) => exports.push(entry),
                Err(e) => {
                    log::warn!("Failed to read export entry: {}", e);
                    break; // Stop reading exports on error
                }
            }
        }

        Ok(UPKFile {
            path,
            header,
            names,
            exports,
            imports,
            file_data: bytes,
        })
    }

    /// Get the name of an object by index
    pub fn get_name(&self, index: u32) -> Option<&str> {
        self.names.get(index as usize).map(|s| s.as_str())
    }

    /// Get the class name of an export
    pub fn get_export_class(&self, export: &ExportEntry) -> Option<&str> {
        let class_index = export.class_index;
        if class_index > 0 {
            let export_idx = (class_index - 1) as usize;
            self.exports.get(export_idx)
                .and_then(|e| self.get_name(e.object_name_index))
        } else if class_index < 0 {
            let import_idx = (-class_index - 1) as usize;
            self.imports.get(import_idx)
                .and_then(|i| self.get_name(i.class_name_index))
        } else {
            None
        }
    }

    /// Extract raw data for an export
    pub fn extract_export_data(&self, export: &ExportEntry) -> Result<Bytes> {
        let start = export.serial_offset as usize;
        let end = start + export.serial_size as usize;

        if end > self.file_data.len() {
            return Err(anyhow!(
                "Export data out of bounds: offset={}, size={}",
                export.serial_offset,
                export.serial_size
            ));
        }

        Ok(self.file_data.slice(start..end))
    }
}
