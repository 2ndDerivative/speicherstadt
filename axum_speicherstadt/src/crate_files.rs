use std::{
    fs::{create_dir_all, remove_file, OpenOptions},
    io::{ErrorKind, Read, Write},
    os::windows::fs::OpenOptionsExt,
    path::PathBuf,
};

pub trait CrateFileStorage: Clone + Send + Sync + 'static {
    fn store_file(&self, crate_name: &str, version: &str, data: &[u8]) -> std::io::Result<()>;
    fn delete_file(&self, crate_name: &str, version: &str) -> std::io::Result<()>;
    fn get_file(&self, crate_name: &str, version: &str) -> std::io::Result<Option<Vec<u8>>>;
}
#[derive(Clone)]
pub struct Filesystem {
    base_path: PathBuf,
}
impl Filesystem {
    pub fn new(base_path: PathBuf) -> Self {
        Filesystem { base_path }
    }
}
impl Filesystem {
    fn file_path(&self, crate_name: &str, version: &str) -> PathBuf {
        self.base_path.join(crate_name).join(version)
    }
}
impl CrateFileStorage for Filesystem {
    fn store_file(&self, crate_name: &str, version: &str, data: &[u8]) -> std::io::Result<()> {
        let path = self.file_path(crate_name, version);
        create_dir_all(path.parent().expect("file_path guarantees at least one parent"))?;
        let mut open_options = OpenOptions::new();
        open_options.write(true).create_new(true);
        #[cfg(windows)]
        open_options.share_mode(0);
        open_options.open(self.file_path(crate_name, version))?.write_all(data)
    }
    fn delete_file(&self, crate_name: &str, version: &str) -> std::io::Result<()> {
        remove_file(self.file_path(crate_name, version))
    }
    fn get_file(&self, crate_name: &str, version: &str) -> std::io::Result<Option<Vec<u8>>> {
        let mut data = Vec::new();
        match OpenOptions::new()
            .read(true)
            .open(self.file_path(crate_name, version))?
            .read_to_end(&mut data)
        {
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
            Ok(_) => Ok(Some(data)),
        }
    }
}
