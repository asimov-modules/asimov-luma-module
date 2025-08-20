// This is free and unencumbered software released into the public domain.

use asimov_luma_module::api::Client;
use asimov_module::SysexitsError::{self, *};
use clap::Parser;
use clientele::StandardOptions;
use colored_json::ToColoredJson;
use serde_json::json;
use std::error::Error;

mod target;
use target::*;

mod jq;

/// asimov-luma-cataloger
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The maximum number of resources to list.
    #[arg(value_name = "COUNT", short = 'n', long)]
    limit: Option<usize>,

    /// The output format.
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    resource: String,
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

    let client = Client::new();
    let filter = jq::filter();

    let target = parse_fetch_url(options.resource).ok_or("invalid resource")?;
    match target {
        FetchTarget::Discover => {
            let (categories, calendars, places) = tokio::try_join!(
                client.list_categories(),
                client.get_featured_calendars(),
                client.list_places()
            )?;

            let result = json!({
                "categories": categories.entries,
                "calendars": calendars.infos,
                "places": places.infos,
            });

            println!("{}", serde_json::to_string_pretty(&result)?);
        },
        _ => unimplemented!(),
    };

    Ok(EX_OK)
}
