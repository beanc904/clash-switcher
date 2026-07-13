use std::{collections::HashMap, process::Command};

use bollard::{Docker, plugin::ContainerSummary, query_parameters::ListContainersOptions};
use clap::Parser;
use colored::Colorize;
use sysproxy::Sysproxy;

mod args;

const CONTAINER_NAME: &'static str = "clash-verge";
const IMAGE_NAME: &'static str = "clash-verge-rev:v2.5.1";
const PROGRAME_NAME: &'static str = "clash-verge";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = args::Args::parse();
    let sysproxy = Sysproxy::get_system_proxy().unwrap();
    #[cfg(target_os = "linux")]
    let docker = Docker::connect_with_local_defaults()?;
    #[cfg(target_os = "macos")]
    let docker = Docker::connect_with_unix(
        "/Users/coffeebean/.colima/default/docker.sock",
        120,
        bollard::API_DEFAULT_VERSION,
    )?;

    // ANCHOR: check the docker image
    let result = docker.inspect_image(IMAGE_NAME).await;
    match result {
        Ok(_) => {
            log::debug!("{} image exist", IMAGE_NAME);
        }
        Err(bollard::errors::Error::DockerResponseServerError {
            status_code: 404, ..
        }) => {
            log::debug!("image does not exist, trying to pull the image");
            println!("{}", "Image is not exist. Trying to pull one.".red());
            // pull image from github

            println!("{}", "Finishing pulling the image.".green());
        }
        Err(e) => {
            log::error!("other error: {e}");
        }
    }
    // ANCHOR_END: check the docker image

    // ANCHOR: get the status of docker container
    let mut filters = HashMap::new();
    filters.insert("name".to_string(), vec![CONTAINER_NAME.to_string()]);

    let containers = docker
        .list_containers(Some(ListContainersOptions {
            all: true,
            filters: Some(filters),
            ..Default::default()
        }))
        .await?;

    if containers.is_empty() {
        log::debug!("container does not exist, trying to up the container");
        println!("{}", "Container is not exist. Trying to up one.".red());
        // trying to up a clash-verge container

        println!("{}", "Finishing up the container.".green());
        return Ok(());
    }

    let container = &containers[0];
    // ANCHOR_END: get the status of docker container

    match args {
        args::Args {
            launch: false,
            on: false,
            off: false,
            status: false,
            dashboard: false,
            kill: false,
        } => {
            log::debug!("Run without any args.");
            Ok(())
        }
        _ => {
            if args.launch {
                log::debug!("Use launch arg.");
                Command::new("docker")
                    .args(["start", CONTAINER_NAME])
                    .status()?;
            }

            if args.on {
                log::debug!("Use on arg.");
            }

            if args.off {
                log::debug!("Use off arg.");
            }

            if args.status {
                log::debug!("Use status arg.");
                println!("{}", status(&sysproxy, &container));
            }

            if args.dashboard {
                log::debug!("Use dashboard arg.");
                Command::new("docker")
                    .args(["exec", CONTAINER_NAME, PROGRAME_NAME, "--dashboard-toggle"])
                    .status()?;
            }

            if args.kill {
                log::debug!("Use kill arg.");
                Command::new("docker")
                    .args(["kill", CONTAINER_NAME])
                    .status()?;
            }

            Ok(())
        }
    }
}

fn status(sysproxy: &Sysproxy, summary: &ContainerSummary) -> String {
    let enable = if sysproxy.enable {
        &"on".green()
    } else {
        &"off".red()
    };
    let host = &sysproxy.host;
    let port = sysproxy.port;
    let bypass = &sysproxy.bypass;

    let state = match &summary.state {
        Some(state) => format!("{}", state).blue(),
        None => format!("Error").red(),
    };
    let status = match &summary.status {
        Some(status) => format!("{}", status).blue(),
        None => format!("Error").red(),
    };

    format!(
        r#"
>>> Docker Clash Status <<<
System proxy: {enable}
Clash container state: {state}
Clash container status: {status}
Proxy [host:port]: {host}:{port}
Bypass: {bypass}
"#
    )
}
