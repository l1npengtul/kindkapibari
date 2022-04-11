use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "coconutpakman")]
#[clap(bin_name = "coconutpakman")]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Version(Version),
    Init(Init),
    New(New),
    Clean(Clean),
    Build(Build),
    Login(Login),
    Publish(Publish),
    Yank(Yank),
}

#[derive(clap::Args)]
struct Version {}

#[derive(clap::Args)]
struct Init {}

#[derive(clap::Args)]
struct New {}

#[derive(clap::Args)]
struct Clean {}

#[derive(clap::Args)]
struct Build {}

#[derive(clap::Args)]
struct Login {}

#[derive(clap::Args)]
struct Publish {}

#[derive(clap::Args)]
struct Yank {}
