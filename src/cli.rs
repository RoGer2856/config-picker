use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct StoreParams {
    #[arg(index = 1)]
    pub(crate) config_type_name: String,

    #[arg(index = 2)]
    pub(crate) label: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct LoadParams {
    #[arg(index = 1)]
    pub(crate) config_type_name: String,

    #[arg(index = 2)]
    pub(crate) label: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ListParams {
    #[arg(index = 1)]
    pub(crate) config_type_name: Option<String>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CreateTypeParams {
    #[arg(index = 1)]
    pub(crate) config_type_name: String,
}

// #[derive(Parser)]
// #[command(version, about, long_about = None)]
// pub struct CopyLabelParams {
//     #[arg(index = 1)]
//     pub(crate) config_type_name: String,

//     #[arg(index = 2)]
//     pub(crate) label: String,

//     #[arg(index = 3)]
//     pub(crate) new_label: String,
// }

// #[derive(Parser)]
// #[command(version, about, long_about = None)]
// pub struct WhichParams {
//     #[arg(index = 1)]
//     pub(crate) config_type_name: String,
// }

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Cli {
    Store(StoreParams),
    Load(LoadParams),
    List(ListParams),
    CreateType(CreateTypeParams),
    // CopyLabel(CopyLabelParams),
    // Which(WhichParams),
}
