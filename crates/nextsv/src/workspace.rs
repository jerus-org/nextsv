use std::path::Path;

use crate::Error;
use cargo_toml::Manifest;

/// Represents the Cargo.toml of the workspace
///
/// See <https://doc.rust-lang.org/cargo/reference/manifest.html#the-workspace>
///  
#[derive(Debug, Clone)]
pub struct Workspace {
    /// The Cargo.toml of the workspace
    pub manifest: Manifest,
}

impl Workspace {
    /// Creates a new Workspace
    ///
    /// # Example
    ///     
    /// ```no_run
    /// # use std::path::Path;
    /// # fn main() -> Result<(),nextsv::Error> {
    /// # use nextsv::Workspace;
    ///     let path = Path::new("./Cargo.toml");
    ///     let workspace = Workspace::new(path)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(ws_cargo_toml: &Path) -> Result<Self, Error> {
        let manifest = Manifest::from_path(ws_cargo_toml)?;

        Ok(Self { manifest })
    }

    /// Returns the list of packages in the workspace
    ///     
    /// # Example
    ///     
    /// ```no_run
    /// # use std::path::Path;
    /// # fn main() -> Result<(),nextsv::Error> {
    /// # use nextsv::Workspace;
    ///     let path = Path::new("./Cargo.toml");
    ///     let workspace = Workspace::new(path)?;
    ///     let packages = workspace.packages();
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn packages(&self) -> Option<Vec<Package>> {
        if let Some(workspace) = &self.manifest.workspace {
            let members = &workspace.members;

            let mut packages = Vec::new();

            for member in members {
                let member_file = format!("./{member}/Cargo.toml");
                let path = Path::new(&member_file);
                let manifest = Manifest::from_path(path).unwrap();
                if let Some(package) = manifest.package {
                    let name = package.name;
                    let version = package.version.get().unwrap().to_string();

                    let package = Package {
                        name,
                        version,
                        member: member.to_string(),
                    };

                    packages.push(package);
                }
            }

            return Some(packages);
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub member: String,
}
