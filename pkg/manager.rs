use crate::pkg::package::Package;

pub trait PackageManager {
    fn name(&self) -> &str;

    fn search(&self, query: &str) -> Result<Vec<Package>, String>;

    fn install(&self, package: &str) -> Result<(), String>;

    fn remove(&self, package: &str) -> Result<(), String>;

    fn update(&self, package: Option<&str>) -> Result<(), String>;

    fn info(&self, package: &str) -> Result<Package, String>;
}
