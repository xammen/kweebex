//! Feature flags for Kweebex - enables gradual rollout of content types.
//! All features default to OFF (Hytale-minimal mode). Set env vars to "true" to enable.
//!
//! # Example .env:
//! ```env
//! FEATURE_SHADERS=false
//! FEATURE_DATAPACKS=false
//! FEATURE_RESOURCEPACKS=false
//! FEATURE_PLUGINS=false
//! FEATURE_LOADER_FABRIC=false
//! FEATURE_LOADER_HYTALE_NATIVE=true
//! ```

use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct Features {
    pub shaders: bool,
    pub datapacks: bool,
    pub resourcepacks: bool,
    pub plugins: bool,
    pub mods: bool,
    pub modpacks: bool,

    pub loader_fabric: bool,
    pub loader_forge: bool,
    pub loader_legacy_forge: bool,
    pub loader_quilt: bool,
    pub loader_neoforge: bool,
    pub loader_liteloader: bool,
    pub loader_rift: bool,

    pub loader_hytale_native: bool,
}

impl Features {
    pub fn from_env() -> Self {
        Self {
            shaders: parse_bool_env("FEATURE_SHADERS", false),
            datapacks: parse_bool_env("FEATURE_DATAPACKS", false),
            resourcepacks: parse_bool_env("FEATURE_RESOURCEPACKS", false),
            plugins: parse_bool_env("FEATURE_PLUGINS", false),
            mods: parse_bool_env("FEATURE_MODS", true),
            modpacks: parse_bool_env("FEATURE_MODPACKS", true),

            loader_fabric: parse_bool_env("FEATURE_LOADER_FABRIC", false),
            loader_forge: parse_bool_env("FEATURE_LOADER_FORGE", false),
            loader_legacy_forge: parse_bool_env("FEATURE_LOADER_LEGACY_FORGE", false),
            loader_quilt: parse_bool_env("FEATURE_LOADER_QUILT", false),
            loader_neoforge: parse_bool_env("FEATURE_LOADER_NEOFORGE", false),
            loader_liteloader: parse_bool_env("FEATURE_LOADER_LITELOADER", false),
            loader_rift: parse_bool_env("FEATURE_LOADER_RIFT", false),

            loader_hytale_native: parse_bool_env("FEATURE_LOADER_HYTALE_NATIVE", true),
        }
    }

    pub fn shaders_enabled(&self) -> bool {
        self.shaders
    }

    pub fn plugins_enabled(&self) -> bool {
        self.plugins
    }

    pub fn any_minecraft_loader_enabled(&self) -> bool {
        self.loader_fabric
            || self.loader_forge
            || self.loader_legacy_forge
            || self.loader_quilt
            || self.loader_neoforge
            || self.loader_liteloader
            || self.loader_rift
    }

    pub fn enabled_project_types(&self) -> Vec<&'static str> {
        let mut types = Vec::new();
        
        if self.mods { types.push("mod"); }
        if self.modpacks { types.push("modpack"); }
        if self.shaders { types.push("shader"); }
        if self.datapacks { types.push("datapack"); }
        if self.resourcepacks { types.push("resourcepack"); }
        if self.plugins { types.push("plugin"); }

        types
    }

    pub fn enabled_loaders(&self) -> Vec<&'static str> {
        let mut loaders = Vec::new();

        if self.loader_hytale_native { loaders.push("hytale"); }
        if self.loader_fabric { loaders.push("fabric"); }
        if self.loader_forge { loaders.push("forge"); }
        if self.loader_legacy_forge { loaders.push("legacyforge"); }
        if self.loader_quilt { loaders.push("quilt"); }
        if self.loader_neoforge { loaders.push("neoforge"); }
        if self.loader_liteloader { loaders.push("liteloader"); }
        if self.loader_rift { loaders.push("rift"); }

        loaders
    }
}

fn parse_bool_env(key: &str, default: bool) -> bool {
    std::env::var(key)
        .map(|v| v.to_lowercase() == "true" || v == "1")
        .unwrap_or(default)
}

pub static FEATURES: LazyLock<Features> = LazyLock::new(Features::from_env);

pub fn is_project_type_enabled(project_type: &str) -> bool {
    match project_type.to_lowercase().as_str() {
        "mod" => FEATURES.mods,
        "modpack" => FEATURES.modpacks,
        "shader" => FEATURES.shaders,
        "datapack" => FEATURES.datapacks,
        "resourcepack" => FEATURES.resourcepacks,
        "plugin" => FEATURES.plugins,
        _ => false,
    }
}

pub fn is_loader_enabled(loader: &str) -> bool {
    match loader.to_lowercase().as_str() {
        "hytale" => FEATURES.loader_hytale_native,
        "fabric" => FEATURES.loader_fabric,
        "forge" => FEATURES.loader_forge,
        "legacyforge" => FEATURES.loader_legacy_forge,
        "quilt" => FEATURES.loader_quilt,
        "neoforge" => FEATURES.loader_neoforge,
        "liteloader" => FEATURES.loader_liteloader,
        "rift" => FEATURES.loader_rift,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_features() {
        std::env::remove_var("FEATURE_SHADERS");
        std::env::remove_var("FEATURE_MODS");

        let features = Features::from_env();
        
        assert!(features.mods);
        assert!(features.modpacks);
        assert!(!features.shaders);
        assert!(!features.plugins);
        assert!(features.loader_hytale_native);
        assert!(!features.loader_fabric);
    }

    #[test]
    fn test_enabled_project_types() {
        let features = Features {
            mods: true,
            modpacks: true,
            shaders: false,
            datapacks: false,
            resourcepacks: false,
            plugins: false,
            loader_fabric: false,
            loader_forge: false,
            loader_legacy_forge: false,
            loader_quilt: false,
            loader_neoforge: false,
            loader_liteloader: false,
            loader_rift: false,
            loader_hytale_native: true,
        };

        let types = features.enabled_project_types();
        assert_eq!(types, vec!["mod", "modpack"]);
    }
}
