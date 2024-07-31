use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct StoreParams {
    #[arg(short, long)]
    pub(crate) name: String,

    #[arg(short, long)]
    pub(crate) label: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct LoadParams {
    #[arg(short, long)]
    pub(crate) name: String,

    #[arg(short, long)]
    pub(crate) label: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ListParams {
    #[arg(short, long)]
    pub(crate) name: Option<String>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CreateTypeParams {
    #[arg(short, long)]
    pub(crate) name: String,
}

// #[derive(Parser)]
// #[command(version, about, long_about = None)]
// pub struct CopyLabelParams {
//     #[arg(short, long)]
//     pub(crate) name: String,

//     #[arg(short, long)]
//     pub(crate) label: String,

//     #[arg(short, long)]
//     pub(crate) new_label: String,
// }

// #[derive(Parser)]
// #[command(version, about, long_about = None)]
// pub struct WhichParams {
//     #[arg(short, long)]
//     pub(crate) name: String,
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
