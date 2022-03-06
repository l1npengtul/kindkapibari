use clap::Parser;

#[derive(Parser)]
#[clap(name = "coconutpak")]
#[clap(bin_name = "coconutpak")]
enum CoconutPak {
    Version(Version),
    Init(Init),
    New(New),
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
