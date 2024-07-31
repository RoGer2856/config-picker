#![allow(clippy::enum_variant_names)]

use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnsureDirectoryError {
    #[error("path exists and not directory")]
    PathIsNotADirectory(PathBuf),

    #[error("could not create directory, path = \"{path}\"")]
    CouldNotCreateDirectory {
        path: PathBuf,

        #[source]
        error: std::io::Error,
    },

    #[error("path does not exist")]
    PathDoesNotExist(PathBuf),
}

#[derive(Debug, Error)]
pub enum CreateNewDirectoryError {
    #[error("could not create directory, path = \"{path}\"")]
    CouldNotCreateDirectory {
        path: PathBuf,

        #[source]
        error: std::io::Error,
    },

    #[error("path already exists, path = \"{0}\"")]
    PathAlreadyExists(PathBuf),
}

#[derive(Debug, Error)]
pub enum ConfigStorageConstructionError {
    #[error("invalid storage path = \"{0}\"")]
    InvalidStoragePath(PathBuf),
}

#[derive(Debug, Error)]
pub enum IterConfigTypesError {
    #[error("io error = {0}")]
    IoError(
        #[source]
        #[from]
        std::io::Error,
    ),
}

#[derive(Debug, Error)]
pub enum IterConfigTypeLabelsError {
    #[error("io error = {0}")]
    IoError(
        #[source]
        #[from]
        std::io::Error,
    ),
}

#[derive(Debug, Error)]
pub enum StoreLabeledConfigTypeError {
    #[error("could not create temp directory, path = {path}, error = {io_error}")]
    CouldNotCreateTempDirectory {
        #[source]
        io_error: CreateNewDirectoryError,
        path: PathBuf,
    },

    #[error("could not decode file location in config type descriptor, error = {0}")]
    CouldNotDecodeFileLocationInConfigTypeDescriptor(#[source] DecodeStringError),

    #[error("file location in config file descriptor does not contain a valid parent directory, path = {0}")]
    InvalidParentOfFileLocationInConfigTypeDescriptor(PathBuf),

    #[error("could not remove old directory, path = {path}, error = {io_error}")]
    CouldNotRemoveOldDirectory {
        #[source]
        io_error: std::io::Error,
        path: PathBuf,
    },

    #[error("could not rename temp directory, source_path = {source_path}, dest_path = {dest_path}, error = {io_error}")]
    CouldNotRenameTempDirectory {
        #[source]
        io_error: std::io::Error,
        source_path: PathBuf,
        dest_path: PathBuf,
    },

    #[error("could not copy file, source_path = {source_path}, dest_path = {dest_path}, error = {io_error}")]
    CouldNotCopyFile {
        #[source]
        io_error: std::io::Error,
        source_path: PathBuf,
        dest_path: PathBuf,
    },

    #[error("could not create directory, error = {0}")]
    CouldNotCreateDirectory(#[source] EnsureDirectoryError),
}

impl From<DecodeStringError> for StoreLabeledConfigTypeError {
    fn from(value: DecodeStringError) -> Self {
        Self::CouldNotDecodeFileLocationInConfigTypeDescriptor(value)
    }
}

impl From<EnsureDirectoryError> for StoreLabeledConfigTypeError {
    fn from(value: EnsureDirectoryError) -> Self {
        Self::CouldNotCreateDirectory(value)
    }
}

#[derive(Debug, Error)]
pub enum LoadLabeledConfigTypeError {
    #[error("could not decode file location in config type descriptor, error = {0}")]
    CouldNotDecodeFileLocationInConfigTypeDescriptor(#[source] DecodeStringError),

    #[error("file location in config file descriptor does not contain a valid parent directory, path = {0}")]
    InvalidParentOfFileLocationInConfigTypeDescriptor(PathBuf),

    #[error("could not copy file, source_path = {source_path}, dest_path = {dest_path}, error = {io_error}")]
    CouldNotCopyFile {
        #[source]
        io_error: std::io::Error,
        source_path: PathBuf,
        dest_path: PathBuf,
    },

    #[error("could not create directory, error = {0}")]
    CouldNotCreateDirectory(#[source] EnsureDirectoryError),
}

impl From<DecodeStringError> for LoadLabeledConfigTypeError {
    fn from(value: DecodeStringError) -> Self {
        Self::CouldNotDecodeFileLocationInConfigTypeDescriptor(value)
    }
}

impl From<EnsureDirectoryError> for LoadLabeledConfigTypeError {
    fn from(value: EnsureDirectoryError) -> Self {
        Self::CouldNotCreateDirectory(value)
    }
}

#[derive(Debug, Error)]
pub enum ConfigTypeDirValidationError {
    #[error("config type dir does not exist, path = \"{path}\"")]
    DirectoryNotFound { path: PathBuf },

    #[error("given path is not a directory, path = \"{path}\"")]
    GivenPathIsNotADirectory { path: PathBuf },

    #[error("could not read descriptor, error = {0}")]
    CouldNotReadDescriptor(ReadConfigTypeDescriptorError),
}

impl From<EnsureDirectoryError> for ConfigTypeDirValidationError {
    fn from(value: EnsureDirectoryError) -> Self {
        match value {
            EnsureDirectoryError::CouldNotCreateDirectory { path, .. } => {
                ConfigTypeDirValidationError::GivenPathIsNotADirectory { path }
            }
            EnsureDirectoryError::PathDoesNotExist(path) => {
                ConfigTypeDirValidationError::DirectoryNotFound { path }
            }
            EnsureDirectoryError::PathIsNotADirectory(path) => {
                ConfigTypeDirValidationError::GivenPathIsNotADirectory { path }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum GetConfigTypeError {
    #[error("incorrect config type directory, config type = \"{config_type}\"")]
    IncorrectConfigTypeDir {
        config_type: String,

        #[source]
        validation_error: ConfigTypeDirValidationError,
    },

    #[error("config type not found = \"{config_type}\"")]
    ConfigTypeNotFound {
        config_type: String,

        #[source]
        validation_error: ConfigTypeDirValidationError,
    },
}

impl<T: Into<String>> From<(T, ConfigTypeDirValidationError)> for GetConfigTypeError {
    fn from((config_type, validation_error): (T, ConfigTypeDirValidationError)) -> Self {
        match &validation_error {
            ConfigTypeDirValidationError::DirectoryNotFound { .. } => {
                GetConfigTypeError::ConfigTypeNotFound {
                    config_type: config_type.into(),
                    validation_error,
                }
            }
            ConfigTypeDirValidationError::GivenPathIsNotADirectory { .. } => {
                GetConfigTypeError::IncorrectConfigTypeDir {
                    config_type: config_type.into(),
                    validation_error,
                }
            }
            ConfigTypeDirValidationError::CouldNotReadDescriptor(_) => {
                GetConfigTypeError::IncorrectConfigTypeDir {
                    config_type: config_type.into(),
                    validation_error,
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum CreateConfigTypeError {
    #[error("incorrect config type found, config type = \"{config_type}\"")]
    IncorrectConfigTypeFound {
        config_type: String,

        #[source]
        validation_error: ConfigTypeDirValidationError,
    },

    #[error("config type already exists, config type = \"{config_type}\"")]
    ConfigTypeAlreadyExists { config_type: String },

    #[error("could not create directory, path = \"{path}\"")]
    CouldNotCreateDirectory {
        path: PathBuf,

        #[source]
        new_directory_error: CreateNewDirectoryError,
    },

    #[error("could not write descriptor to file, error = {0}")]
    CouldNotWriteDescriptorToFile(WriteConfigTypeDescriptorError),
}

#[derive(Debug, Error)]
pub enum WriteConfigTypeDescriptorError {
    #[error("could open file")]
    CouldNotOpenFile(#[source] std::io::Error),

    #[error("could not write data to file")]
    CouldNotWriteDataToFile(#[source] std::io::Error),

    #[error("could not serialize data")]
    CouldNotSerializeData(
        #[source]
        #[from]
        serde_json::Error,
    ),
}

#[derive(Debug, Error)]
pub enum ReadConfigTypeDescriptorError {
    #[error("could open file")]
    CouldNotOpenFile(#[source] std::io::Error),

    #[error("could not write data to file")]
    CouldNotDeserializeData(
        #[source]
        #[from]
        serde_json::Error,
    ),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CollectBlocksFromStringError {
    #[error("opened block is note closed, block start offset = {block_start_offset}")]
    OpenedBlockIsNotClosed { block_start_offset: usize },
}

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Error)]
pub enum DecodeStringError {
    #[error("could not resolve variable, variable name = {variable_name}")]
    CouldNotResolveVariable { variable_name: String },

    #[error("could not collect blocks from string, error = {0}")]
    CollectBlocksFromStringError(#[source] CollectBlocksFromStringError),
}

impl From<CollectBlocksFromStringError> for DecodeStringError {
    fn from(value: CollectBlocksFromStringError) -> Self {
        Self::CollectBlocksFromStringError(value)
    }
}
