use anyhow::Result;
use log::{debug, info, warn};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::asset_types::{AssetType, ue3_classes};
use crate::upk::UPKFile;

pub struct AssetExtractor {
    output_dir: PathBuf,
}

impl AssetExtractor {
    pub fn new(output_dir: PathBuf) -> Self {
        AssetExtractor { output_dir }
    }

    /// Recursively process all UPK files in a directory
    pub fn process_directory(&self, input_dir: &Path) -> Result<()> {
        info!("Scanning for UPK files in: {}", input_dir.display());

        let mut upk_count = 0;
        let mut total_assets = 0;

        for entry in WalkDir::new(input_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("upk") {
                upk_count += 1;
                match self.process_upk_file(path) {
                    Ok(asset_count) => {
                        info!("Processed {}: extracted {} assets", path.display(), asset_count);
                        total_assets += asset_count;
                    }
                    Err(e) => {
                        warn!("Failed to process {}: {}", path.display(), e);
                    }
                }
            }
        }

        info!("Processing complete: {} UPK files, {} total assets extracted", upk_count, total_assets);
        Ok(())
    }

    /// Process a single UPK file
    fn process_upk_file(&self, upk_path: &Path) -> Result<u32> {
        debug!("Loading UPK file: {}", upk_path.display());

        let upk = UPKFile::load(upk_path)?;
        let mut asset_count = 0;

        // Process each export
        for (idx, export) in upk.exports.iter().enumerate() {
            let object_name = upk.get_name(export.object_name_index)
                .unwrap_or("Unknown")
                .to_string();
            let class_name = upk.get_export_class(export)
                .unwrap_or("Unknown")
                .to_string();

            debug!("Export {}: {} (class: {})", idx, object_name, class_name);

            // Determine asset type and extract
            match class_name.as_str() {
                ue3_classes::SKELETAL_MESH | ue3_classes::STATIC_MESH => {
                    if let Ok(data) = upk.extract_export_data(export) {
                        if let Err(e) = self.save_model_asset(&object_name, &data) {
                            warn!("Failed to save model asset {}: {}", object_name, e);
                        } else {
                            asset_count += 1;
                        }
                    }
                }
                ue3_classes::ANIM_SET | ue3_classes::ANIM_SEQUENCE => {
                    if let Ok(data) = upk.extract_export_data(export) {
                        if let Err(e) = self.save_animation_asset(&object_name, &data) {
                            warn!("Failed to save animation asset {}: {}", object_name, e);
                        } else {
                            asset_count += 1;
                        }
                    }
                }
                ue3_classes::TEXTURE_2D => {
                    if let Ok(data) = upk.extract_export_data(export) {
                        if let Err(e) = self.save_texture_asset(&object_name, &data) {
                            warn!("Failed to save texture asset {}: {}", object_name, e);
                        } else {
                            asset_count += 1;
                        }
                    }
                }
                _ => {
                    debug!("Skipping unknown class type: {}", class_name);
                }
            }
        }

        Ok(asset_count)
    }

    /// Save extracted model data
    fn save_model_asset(&self, name: &str, _data: &[u8]) -> Result<()> {
        let asset_dir = self.output_dir.join(AssetType::Model.folder_name());
        std::fs::create_dir_all(&asset_dir)?;

        // TODO: Parse UE3 skeletal/static mesh format and convert to glTF
        let output_path = asset_dir.join(format!("{}.gltf", sanitize_name(name)));
        
        debug!("Placeholder model asset: {}", output_path.display());
        std::fs::write(&output_path, b"# Model asset placeholder\n")?;

        Ok(())
    }

    /// Save extracted animation data
    fn save_animation_asset(&self, name: &str, _data: &[u8]) -> Result<()> {
        let asset_dir = self.output_dir.join(AssetType::Animation.folder_name());
        std::fs::create_dir_all(&asset_dir)?;

        // TODO: Parse UE3 animation format and convert to glTF
        let output_path = asset_dir.join(format!("{}.gltf", sanitize_name(name)));
        
        debug!("Placeholder animation asset: {}", output_path.display());
        std::fs::write(&output_path, b"# Animation asset placeholder\n")?;

        Ok(())
    }

    /// Save extracted texture data
    fn save_texture_asset(&self, name: &str, data: &[u8]) -> Result<()> {
        let asset_dir = self.output_dir.join(AssetType::Texture.folder_name());
        std::fs::create_dir_all(&asset_dir)?;

        // TODO: Parse DDS/texture format and convert to PNG
        let output_path = asset_dir.join(format!("{}.png", sanitize_name(name)));
        
        debug!("Placeholder texture asset: {}", output_path.display());
        std::fs::write(&output_path, data)?;

        Ok(())
    }
}

/// Sanitize filename to remove invalid characters
fn sanitize_name(name: &str) -> String {
    name
        .replace('/', "_")
        .replace('\\', "_")
        .replace(':', "_")
        .replace('*', "_")
        .replace('?', "_")
        .replace('"', "_")
        .replace('<', "_")
        .replace('>', "_")
        .replace('|', "_")
}
