mod cli;
mod config_storage;
mod config_type_descriptor;
mod directories;
mod error;
mod utils;
mod variable_resolver;

use clap::Parser;

use ::directories::BaseDirs;
use cli::{Cli, CreateTypeParams, ListParams, LoadParams, StoreParams};
use config_storage::ConfigStorage;
use variable_resolver::VariableResolver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let base_dirs = BaseDirs::new().unwrap();
    let mut storage_root_dir = base_dirs.home_dir().to_path_buf();
    storage_root_dir.push(".config-picker");
    let config_storage = ConfigStorage::new(VariableResolver::new(base_dirs), storage_root_dir)?;

    match cli {
        Cli::Store(params) => store(config_storage, params)?,
        Cli::Load(params) => load(config_storage, params)?,
        Cli::List(params) => list(config_storage, params)?,
        Cli::CreateType(params) => create_config_type(config_storage, params)?,
        // Cli::CopyLabel(params) => todo!(),
        // Cli::Which(params) => todo!(),
    }

    Ok(())
}

fn store(
    config_storage: ConfigStorage,
    params: StoreParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_type_storage = config_storage.get_config_type_storage(params.name)?;
    config_type_storage.store(params.label)?;
    Ok(())
}

fn load(
    config_storage: ConfigStorage,
    params: LoadParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_type_storage = config_storage.get_config_type_storage(params.name)?;
    config_type_storage.load(params.label)?;
    Ok(())
}

fn create_config_type(
    config_storage: ConfigStorage,
    params: CreateTypeParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_type_storage = config_storage.create_config_type(&params.name)?;

    println!(
        "Config type created, config type = \"{}\", descriptor file = {:?}",
        params.name,
        config_type_storage.descriptor_path()
    );

    Ok(())
}

fn list(
    config_storage: ConfigStorage,
    params: ListParams,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(config_type) = params.name {
        let config_type_storage = config_storage.get_config_type_storage(config_type)?;

        for label in config_type_storage.iter_labels()? {
            println!("{}", label?);
        }

        Ok(())
    } else {
        for config_type in config_storage.iter_config_types()? {
            println!("{}", config_type?);
        }

        Ok(())
    }
}