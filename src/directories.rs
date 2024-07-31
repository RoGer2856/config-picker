use std::path::{Path, PathBuf};

const DESCRIPTOR_FILENAME: &str = "descriptor.json";

pub struct Directories {
    root_dir: PathBuf,
    temp_dir: PathBuf,
    db_dir: PathBuf,
}

impl Directories {
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        let root_dir = root_dir.into();

        let mut root_temp_dir = root_dir.clone();
        root_temp_dir.push("temp");

        let mut root_db_dir = root_dir.clone();
        root_db_dir.push("db");

        Self {
            root_dir,
            db_dir: root_db_dir,
            temp_dir: root_temp_dir,
        }
    }

    pub fn root_dir_path(&self) -> &Path {
        &self.root_dir
    }

    pub fn db_dir_path(&self) -> &Path {
        &self.db_dir
    }

    pub fn temp_dir_path(&self) -> &Path {
        &self.temp_dir
    }

    pub fn config_type_dir_path(&self, config_type: impl AsRef<str>) -> PathBuf {
        let mut ret = self.db_dir.clone();
        ret.push(config_type.as_ref());
        ret
    }

    pub fn labeled_config_type_dir_path(
        &self,
        config_type: impl AsRef<str>,
        label: impl AsRef<str>,
    ) -> PathBuf {
        let mut ret = self.db_dir.clone();
        ret.push(config_type.as_ref());
        ret.push(label.as_ref());
        ret
    }

    pub fn config_type_descriptor_path(&self, config_type: impl AsRef<str>) -> PathBuf {
        let mut ret = self.db_dir.clone();
        ret.push(config_type.as_ref());
        ret.push(DESCRIPTOR_FILENAME);
        ret
    }

    pub fn create_temp_dir_path(&self) -> PathBuf {
        let mut tmp_dir = self.temp_dir.clone();
        tmp_dir.push(uuid::Uuid::new_v4().as_hyphenated().to_string());
        tmp_dir
    }
}
