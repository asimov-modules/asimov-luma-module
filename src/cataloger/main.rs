// This is free and unencumbered software released into the public domain.

use asimov_luma_module::api::Client;
use asimov_module::SysexitsError::{self, *};
use clap::{Parser, Subcommand};
use clientele::StandardOptions;
use colored_json::ToColoredJson;
use std::error::Error;

/// asimov-luma-cataloger
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(about = "List featured calendars.")]
    ListFeaturedCalendars,

    #[clap(about = "List categories.")]
    ListCategories,

    #[clap(about = "List places.")]
    ListPlaces,
}

#[tokio::main]
pub async fn main() -> Result<SysexitsError, Box<dyn Error>> {
    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    // let client = Client::new();
    // match options.command {
    //     Commands::ListFeaturedCalendars => {
    //         let calendars = client.get_featured_calendars().await?;
    //         let json = serde_json::to_string_pretty(&calendars)?;
    //         println!("{}", json.to_colored_json_auto().unwrap());
    //     },
    //     Commands::ListCategories => {
    //         let categories = client.list_categories().await?;
    //         println!("{}", categories.to_colored_json_auto().unwrap());
    //     },
    //     Commands::ListPlaces => {
    //         let places = client.list_places().await?;
    //         println!("{}", places.to_colored_json_auto().unwrap());
    //     },
    // }

    Ok(EX_OK)
}
