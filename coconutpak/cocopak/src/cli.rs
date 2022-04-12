use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "cocopak")]
#[clap(bin_name = "cocopak")]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        name: Option<String>,
        #[clap(default_value_t = false)]
        no_text: bool,
    },
    New {
        directory: Option<String>,
        name: Option<String>,
        #[clap(default_value_t = false)]
        no_text: bool,
    },
    Clean {
        directory: Option<String>,
    },
    Build,
    Login {
        registry: Option<String>,
        key: Option<String>,
    },
    Publish {
        registry: Option<String>,
    },
}
