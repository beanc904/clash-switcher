use clap::Parser;

/// The default behaviour is:
/// launch docker image, when image is down;
/// toggle system proxy, when image is up.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Launch the docker image
    #[arg(short, long)]
    pub launch: bool,

    /// Turn on the system proxy
    #[arg(long, conflicts_with = "off")]
    pub on: bool,

    /// Turn off the system proxy
    #[arg(long, conflicts_with = "on")]
    pub off: bool,

    /// Check the status of the system proxy
    #[arg(short, long)]
    pub status: bool,

    /// Open/Close Dashboard
    #[arg(short, long)]
    pub dashboard: bool,

    /// Kill the docker image process
    #[arg(short, long, conflicts_with_all = ["launch", "on", "off", "status", "dashboard"])]
    pub kill: bool,
}
