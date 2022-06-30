//////////////////////////////////////////////////
// Using

use crate::game::resource::*;

//////////////////////////////////////////////////
// Definition

pub struct ResourceContext {
    package: Option<PackageInfo>,
    level: Option<usize>,
}

//////////////////////////////////////////////////
// Implementation

impl ResourceContext {
    pub fn new() -> ResourceContext {
        ResourceContext {
            package: None,
            level: None,
        }
    }

    pub fn load_package(&mut self, package: &str) {
        self.package = Some(PackageInfo::from(package));
    }

    pub fn unload_package(&mut self) {
        self.package = None;
    }

    pub fn load_level(&mut self, level: usize) {
        self.level = self
            .package
            .as_ref()
            .filter(|package| level < package.levels.len())
            .map(|_| level);
    }

    pub fn load_level_next(&mut self) {
        self.level = if let Some(package) = self.package.as_ref() {
            self.level
                .map(|level| level + 1)
                .filter(|level| *level < package.levels.len())
        } else {
            None
        };
    }

    pub fn unload_level(&mut self) {
        self.level = None;
    }

    pub fn package_info(&self) -> Option<&PackageInfo> {
        self.package.as_ref()
    }

    pub fn level_info(&self) -> Option<&LevelInfo> {
        self.package
            .as_ref()
            .map(|package| self.level.map(|level| &package.levels[level]))
            .flatten()
    }

    pub fn level(&self) -> Option<usize> {
        self.level
    }
}
