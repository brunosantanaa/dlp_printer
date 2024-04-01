use std::path::{Path, PathBuf};

pub struct FilesEnv {
    pub basedir_path: PathBuf,
    pub workdir_path: PathBuf,
    pub storagedir_path: PathBuf,
    pub gcode_path: PathBuf,
}

impl FilesEnv {
    pub fn new(basedir: &str, workfolder: &str, storagefolder: &str) -> Self{
        let basedir_path = Path::new(basedir).to_path_buf();
        let workdir_path = Path::new(&basedir_path).join(workfolder);
        let storagedir_path = Path::new(&basedir_path).join(storagefolder);
        let gcode_path = Path::new(&basedir_path).join("run.gcode");

        Self {basedir_path, workdir_path, storagedir_path, gcode_path}
    }

    pub fn get_gcode_path(&self) -> PathBuf {
        self.gcode_path.to_path_buf()
    }

    pub fn get_workdir_path(&self) -> PathBuf {
        self.workdir_path.to_path_buf()
    }

    pub fn get_storagedir_path(&self) -> PathBuf {
        self.storagedir_path.to_path_buf()
    }
}