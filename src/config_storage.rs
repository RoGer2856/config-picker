use std::{
    fs::{copy, remove_dir_all, rename},
    path::PathBuf,
    rc::Rc,
};

use crate::{
    config_type_descriptor::ConfigTypeDescriptor,
    directories::Directories,
    error::{
        ConfigStorageConstructionError, ConfigTypeDirValidationError, CreateConfigTypeError,
        GetConfigTypeError, IterConfigTypeLabelsError, IterConfigTypesError,
        LoadLabeledConfigTypeError, StoreLabeledConfigTypeError,
    },
    utils::{create_new_directory, ensure_directory, SubDirectoryIterator},
    variable_resolver::VariableResolver,
};

pub struct ListTypesIterator {
    sub_directory_iterator: SubDirectoryIterator,
}

impl Iterator for ListTypesIterator {
    type Item = Result<String, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sub_directory_iterator.next().map(|dir_entry| {
            dir_entry.map(|dir_entry| dir_entry.file_name().to_string_lossy().into())
        })
    }
}

pub struct LabelIterator {
    sub_directory_iterator: SubDirectoryIterator,
}

impl Iterator for LabelIterator {
    type Item = Result<String, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sub_directory_iterator.next().map(|dir_entry| {
            dir_entry.map(|dir_entry| dir_entry.file_name().to_string_lossy().into())
        })
    }
}

pub struct ConfigStorage {
    variable_resolver: Rc<VariableResolver>,
    directories: Rc<Directories>,
}

impl ConfigStorage {
    pub fn new(
        variable_resolver: VariableResolver,
        root_dir: impl Into<PathBuf>,
    ) -> Result<Self, ConfigStorageConstructionError> {
        let directories = Rc::new(Directories::new(root_dir));

        ensure_directory(directories.root_dir_path(), true).map_err(|_| {
            ConfigStorageConstructionError::InvalidStoragePath(
                directories.root_dir_path().to_path_buf(),
            )
        })?;
        ensure_directory(directories.db_dir_path(), true).map_err(|_| {
            ConfigStorageConstructionError::InvalidStoragePath(
                directories.root_dir_path().to_path_buf(),
            )
        })?;
        ensure_directory(directories.temp_dir_path(), true).map_err(|_| {
            ConfigStorageConstructionError::InvalidStoragePath(
                directories.temp_dir_path().to_path_buf(),
            )
        })?;

        Ok(Self {
            variable_resolver: Rc::new(variable_resolver),
            directories,
        })
    }

    pub fn iter_config_types(&self) -> Result<ListTypesIterator, IterConfigTypesError> {
        Ok(ListTypesIterator {
            sub_directory_iterator: SubDirectoryIterator::new(self.directories.db_dir_path())?,
        })
    }

    pub fn get_config_type_storage(
        &self,
        config_type: impl AsRef<str>,
    ) -> Result<ConfigTypeStorage, GetConfigTypeError> {
        Ok(ConfigTypeStorage::new(
            self.variable_resolver.clone(),
            config_type.as_ref(),
            self.directories.clone(),
        )
        .map_err(|e| (config_type.as_ref(), e))?)
    }

    pub fn create_config_type(
        &self,
        config_type: impl AsRef<str>,
    ) -> Result<ConfigTypeStorage, CreateConfigTypeError> {
        match self.get_config_type_storage(config_type.as_ref()) {
            Ok(_) => Err(CreateConfigTypeError::ConfigTypeAlreadyExists {
                config_type: config_type.as_ref().into(),
            }),
            Err(GetConfigTypeError::ConfigTypeNotFound { config_type, .. }) => {
                create_config_type_dir(
                    self.variable_resolver.clone(),
                    self.directories.clone(),
                    config_type,
                )
            }
            Err(GetConfigTypeError::IncorrectConfigTypeDir {
                config_type,
                validation_error,
            }) => Err(CreateConfigTypeError::IncorrectConfigTypeFound {
                config_type: config_type.clone(),
                validation_error,
            }),
        }
    }
}

fn create_config_type_dir(
    variable_resolver: Rc<VariableResolver>,
    directories: Rc<Directories>,
    config_type: impl AsRef<str>,
) -> Result<ConfigTypeStorage, CreateConfigTypeError> {
    let config_type_dir_path = directories.config_type_dir_path(config_type.as_ref());
    create_new_directory(&config_type_dir_path).map_err(|e| {
        CreateConfigTypeError::CouldNotCreateDirectory {
            path: config_type_dir_path.clone(),
            new_directory_error: e,
        }
    })?;

    let empty_config_type_descriptor = ConfigTypeDescriptor::new();

    empty_config_type_descriptor
        .write_to_file(directories.config_type_descriptor_path(config_type.as_ref()))
        .map_err(CreateConfigTypeError::CouldNotWriteDescriptorToFile)?;

    ConfigTypeStorage::new(variable_resolver, config_type.as_ref(), directories).map_err(|e| {
        CreateConfigTypeError::IncorrectConfigTypeFound {
            config_type: config_type.as_ref().to_string(),
            validation_error: e,
        }
    })
}

pub struct ConfigTypeStorage {
    variable_resolver: Rc<VariableResolver>,
    config_type: String,
    directories: Rc<Directories>,
    descriptor: Rc<ConfigTypeDescriptor>,
}

impl ConfigTypeStorage {
    fn new(
        variable_resolver: Rc<VariableResolver>,
        config_type: impl AsRef<str>,
        directories: Rc<Directories>,
    ) -> Result<Self, ConfigTypeDirValidationError> {
        ensure_directory(
            directories.config_type_dir_path(config_type.as_ref()),
            false,
        )?;

        let descriptor = ConfigTypeDescriptor::from_file(
            directories.config_type_descriptor_path(config_type.as_ref()),
        )
        .map_err(ConfigTypeDirValidationError::CouldNotReadDescriptor)?;

        Ok(Self {
            variable_resolver,
            config_type: config_type.as_ref().into(),
            directories,
            descriptor: Rc::new(descriptor),
        })
    }

    pub fn descriptor_path(&self) -> PathBuf {
        self.directories.config_type_dir_path(&self.config_type)
    }

    pub fn iter_labels(&self) -> Result<LabelIterator, IterConfigTypeLabelsError> {
        Ok(LabelIterator {
            sub_directory_iterator: SubDirectoryIterator::new(
                self.directories.config_type_dir_path(&self.config_type),
            )?,
        })
    }

    pub fn store(&self, label: impl AsRef<str>) -> Result<(), StoreLabeledConfigTypeError> {
        let temp_dir = self.directories.create_temp_dir_path();

        create_new_directory(&temp_dir).map_err(|e| {
            StoreLabeledConfigTypeError::CouldNotCreateTempDirectory {
                io_error: e,
                path: temp_dir.clone(),
            }
        })?;

        let labeled_config_type_storage = LabeledConfigTypeStorage::new(
            self.variable_resolver.clone(),
            self.descriptor.clone(),
            &temp_dir,
        );

        labeled_config_type_storage.store()?;

        drop(labeled_config_type_storage);

        let labeled_config_type_dir_path = self
            .directories
            .labeled_config_type_dir_path(&self.config_type, label);
        if labeled_config_type_dir_path.exists() {
            remove_dir_all(&labeled_config_type_dir_path).map_err(|e| {
                StoreLabeledConfigTypeError::CouldNotRemoveOldDirectory {
                    io_error: e,
                    path: labeled_config_type_dir_path.clone(),
                }
            })?;
        }
        rename(&temp_dir, &labeled_config_type_dir_path).map_err(|e| {
            StoreLabeledConfigTypeError::CouldNotRenameTempDirectory {
                io_error: e,
                source_path: temp_dir,
                dest_path: labeled_config_type_dir_path,
            }
        })?;

        Ok(())
    }

    pub fn load(&self, label: impl AsRef<str>) -> Result<(), LoadLabeledConfigTypeError> {
        let labeled_config_type_dir_path = self
            .directories
            .labeled_config_type_dir_path(&self.config_type, label);

        let labeled_config_type_storage = LabeledConfigTypeStorage::new(
            self.variable_resolver.clone(),
            self.descriptor.clone(),
            &labeled_config_type_dir_path,
        );

        labeled_config_type_storage.load()?;

        Ok(())
    }
}

pub struct LabeledConfigTypeStorage {
    variable_resolver: Rc<VariableResolver>,
    descriptor: Rc<ConfigTypeDescriptor>,
    directory_path: PathBuf,
}

impl LabeledConfigTypeStorage {
    pub fn new(
        variable_resolver: Rc<VariableResolver>,
        descriptor: Rc<ConfigTypeDescriptor>,
        directory_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            variable_resolver,
            descriptor,
            directory_path: directory_path.into(),
        }
    }

    pub fn store(&self) -> Result<(), StoreLabeledConfigTypeError> {
        for path in self.descriptor.paths() {
            let decoded_path = PathBuf::from(
                self.variable_resolver
                    .decode_string(path.to_string_lossy())?,
            );

            let src_file_path = decoded_path;

            let mut dest_file_path = self.directory_path.clone();
            dest_file_path.push(path);

            println!("dest = {:?}, source = {:?}", dest_file_path, src_file_path);

            let parent_path = dest_file_path.parent().ok_or_else(|| {
                StoreLabeledConfigTypeError::InvalidParentOfFileLocationInConfigTypeDescriptor(
                    dest_file_path.clone(),
                )
            })?;

            ensure_directory(parent_path, true)?;

            copy(&src_file_path, &dest_file_path).map_err(|e| {
                StoreLabeledConfigTypeError::CouldNotCopyFile {
                    io_error: e,
                    source_path: src_file_path,
                    dest_path: dest_file_path,
                }
            })?;
        }

        Ok(())
    }

    pub fn load(&self) -> Result<(), LoadLabeledConfigTypeError> {
        for path in self.descriptor.paths() {
            let decoded_path = PathBuf::from(
                self.variable_resolver
                    .decode_string(path.to_string_lossy())?,
            );

            let dest_file_path = decoded_path;

            let mut src_file_path = self.directory_path.clone();
            src_file_path.push(path);

            println!("dest = {:?}, source = {:?}", dest_file_path, src_file_path);

            let parent_path = dest_file_path.parent().ok_or_else(|| {
                LoadLabeledConfigTypeError::InvalidParentOfFileLocationInConfigTypeDescriptor(
                    dest_file_path.clone(),
                )
            })?;

            ensure_directory(parent_path, true)?;

            copy(&src_file_path, &dest_file_path).map_err(|e| {
                LoadLabeledConfigTypeError::CouldNotCopyFile {
                    io_error: e,
                    source_path: src_file_path,
                    dest_path: dest_file_path,
                }
            })?;
        }

        Ok(())
    }
}
