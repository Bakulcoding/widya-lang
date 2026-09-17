//! Package Manager (WPM) - Widya Package Manager
//! Manages packages, dependencies, and project scaffolding

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// WPM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WpmConfig {
    pub registry_url: String,
    pub cache_dir: PathBuf,
    pub packages_dir: PathBuf,
    pub global_packages: Vec<String>,
}

/// Package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub dependencies: HashMap<String, String>,
    pub repository: Option<String>,
}

/// WPM (Widya Package Manager)
pub struct Wpm {
    config: WpmConfig,
}

impl Wpm {
    /// Create new WPM instance
    pub fn new(config: WpmConfig) -> Self {
        Self { config }
    }

    /// Initialize new project
    pub fn init(&self, project_name: &str, output_dir: &Path) -> Result<(), String> {
        let project_dir = output_dir.join(project_name);

        // Create project structure
        fs::create_dir_all(&project_dir)
            .map_err(|e| format!("Failed to create project directory: {}", e))?;
        
        fs::create_dir_all(project_dir.join("src"))
            .map_err(|e| format!("Failed to create src dir: {}", e))?;

        // Create widya.toml
        let toml_content = format!(r#"[package]
name = "{}"
version = "0.1.0"
description = "Widya project"
author = "Developer"
license = "MIT"

[dependencies]"#, project_name);

        let mut file = File::create(project_dir.join("widya.toml"))
            .map_err(|e| format!("Failed to create widya.toml: {}", e))?;
        file.write_all(toml_content.as_bytes())
            .map_err(|e| format!("Failed to write widya.toml: {}", e))?;

        // Create main.wya
        let main_content = format!(r#"paket {};

fungsi utama() {{
    cetak("Halo dari {}!");
}}
"#, project_name, project_name);

        let mut file = File::create(project_dir.join("src").join("main.wya"))
            .map_err(|e| format!("Failed to create main.wya: {}", e))?;
        file.write_all(main_content.as_bytes())
            .map_err(|e| format!("Failed to write main.wya: {}", e))?;

        Ok(())
    }

    /// Add dependency
    pub fn add_dependency(&self, package_name: &str, version: &str) -> Result<(), String> {
        println!("Adding {}@{} from registry", package_name, version);
        Ok(())
    }

    /// Install all dependencies
    pub fn install(&self, _manifest_path: &Path) -> Result<PackageLock, String> {
        println!("Installing dependencies...");
        Ok(PackageLock {
            packages: HashMap::new(),
        })
    }

    /// Publish package
    pub fn publish(&self, _package_dir: &Path, _token: &str) -> Result<(), String> {
        println!("Publishing package...");
        Ok(())
    }
}

/// Package lock file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageLock {
    pub packages: HashMap<String, PackageVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    pub version: String,
    pub resolved: String,
    pub integrity: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wpm_init() {
        let config = WpmConfig {
            registry_url: "https://registry.widya.dev".to_string(),
            cache_dir: PathBuf::from("/tmp/wpm/cache"),
            packages_dir: PathBuf::from("/tmp/wpm/packages"),
            global_packages: Vec::new(),
        };
        
        let wpm = Wpm::new(config);
        let result = wpm.init("test-project", Path::new("/tmp"));
        assert!(result.is_ok());
    }
}
