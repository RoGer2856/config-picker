use std::{
    fs::{create_dir, DirEntry, ReadDir},
    path::{Path, PathBuf},
};

use crate::error::{CreateNewDirectoryError, EnsureDirectoryError};

pub fn ensure_directory(
    path: impl AsRef<Path>,
    create_if_not_exists: bool,
) -> Result<(), EnsureDirectoryError> {
    let path = path.as_ref().to_path_buf();

    let mut ancestor_path = PathBuf::new();
    for component in path.components() {
        ancestor_path.push(component);
        if ancestor_path.exists() {
            if !ancestor_path.is_dir() {
                return Err(EnsureDirectoryError::PathIsNotADirectory(ancestor_path));
            }
        } else if create_if_not_exists {
            create_dir(&path).map_err(|e| EnsureDirectoryError::CouldNotCreateDirectory {
                path: path.clone(),
                error: e,
            })?;
        } else {
            return Err(EnsureDirectoryError::PathDoesNotExist(ancestor_path));
        }
    }

    Ok(())
}

pub fn create_new_directory(path: impl AsRef<Path>) -> Result<(), CreateNewDirectoryError> {
    let path = path.as_ref().to_path_buf();

    if path.exists() {
        Err(CreateNewDirectoryError::PathAlreadyExists(
            path.to_path_buf(),
        ))
    } else {
        create_dir(&path).map_err(|e| CreateNewDirectoryError::CouldNotCreateDirectory {
            path: path.clone(),
            error: e,
        })?;

        Ok(())
    }
}

pub struct SubDirectoryIterator {
    paths: ReadDir,
}

impl SubDirectoryIterator {
    pub fn new(parent_dir: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let paths = std::fs::read_dir(&parent_dir)?;
        Ok(Self { paths })
    }
}

impl Iterator for SubDirectoryIterator {
    type Item = Result<DirEntry, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        for dir_entry in self.paths.by_ref() {
            match dir_entry {
                Ok(dir_entry) => {
                    if dir_entry.path().is_dir() {
                        return Some(Ok(dir_entry));
                    }
                }
                Err(e) => {
                    return Some(Err(e));
                }
            }
        }

        None
    }
}
