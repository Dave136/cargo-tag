use std::env::current_dir;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use semver::Version as SemVer;
use serde::Deserialize;

const CARGO_TOML: &str = "Cargo.toml";

#[derive(Debug, Default)]
pub struct Metadata {
    path: PathBuf,
}

/// A `Cargo.toml` file's representation as a struct
#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub(crate) package: Package,
    #[serde(skip_deserializing)]
    pub(crate) meta: Metadata,
}

/// Representation of the `Cargo.toml` `package` section
#[derive(Debug, Deserialize)]
pub struct Package {
    /// Crate version
    pub(crate) version: SemVer,
}

impl CargoToml {
    /// Attempts to read a `Cargo.toml` in the current directory and retrieves
    /// an instance of `CartToml` from it.
    pub fn open() -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = current_dir()?;
        path.push(CARGO_TOML);

        let file = read_to_string(&path)?;
        let mut package: CargoToml = toml::from_str(&file)?;

        package.meta.path = path;

        Ok(package)
    }

    /// Update's current `Cargo.toml` version
    pub fn write_version(&self, version: SemVer) -> Result<(), Box<dyn std::error::Error>> {
        let file_str = read_to_string(&self.meta.path)?;
        let mut document = file_str.parse::<toml_edit::Document>()?;

        document["package"]["version"] = toml_edit::value(version.to_string());

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.meta.path)?;

        file.write_all(document.to_string().as_bytes())?;
        Ok(())
    }
}
