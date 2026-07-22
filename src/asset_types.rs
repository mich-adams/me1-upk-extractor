/// Represents different asset types that can be extracted from UPK files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Model,
    Animation,
    Texture,
}

impl AssetType {
    pub fn extension(&self) -> &'static str {
        match self {
            AssetType::Model => "gltf",
            AssetType::Animation => "gltf",
            AssetType::Texture => "png", // or dds
        }
    }

    pub fn folder_name(&self) -> &'static str {
        match self {
            AssetType::Model => "models",
            AssetType::Animation => "animations",
            AssetType::Texture => "textures",
        }
    }
}

/// UE3 asset class names commonly found in UPK files
pub mod ue3_classes {
    pub const SKELETAL_MESH: &str = "SkeletalMesh";
    pub const STATIC_MESH: &str = "StaticMesh";
    pub const ANIM_SET: &str = "AnimSet";
    pub const ANIM_SEQUENCE: &str = "AnimSequence";
    pub const TEXTURE_2D: &str = "Texture2D";
    pub const MATERIAL: &str = "Material";
}
