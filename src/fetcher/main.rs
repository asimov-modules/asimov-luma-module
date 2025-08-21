// This is free and unencumbered software released into the public domain.

use asimov_luma_module::api::Client;
use asimov_module::{
    SysexitsError::{self, *},
    tracing,
};

use clap::Parser;
use clientele::StandardOptions;
use serde_json::json;
use std::error::Error;

mod target;
use target::*;

mod jq;
use jq::JsonFilter;

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

/// asimov-luma-fetcher
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

pub async fn print_json(value: serde_json::Value, filter: &JsonFilter) -> Result<()> {
    match filter.filter_json(value) {
        Ok(filtered) => println!("{}", colored_json::to_colored_json_auto(&filtered)?),
        Err(jq::JsonFilterError::NoOutput) => (),
        Err(err) => tracing::error!(?err, "Filter failed"),
    }
    Ok(())
}

pub async fn print_discover(client: &Client) -> Result<()> {
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

    let filter = jq::discover();
    print_json(result, filter).await?;
    Ok(())
}

pub async fn print_category(value: serde_json::Value) -> Result<()> {
    let filter = jq::category();
    print_json(value, filter).await?;
    Ok(())
}

pub async fn print_place(value: serde_json::Value) -> Result<()> {
    let filter = jq::place();
    print_json(value, filter).await?;
    Ok(())
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

    let target = parse_fetch_url(options.resource).ok_or("invalid resource")?;
    match target {
        FetchTarget::Discover => {
            print_discover(&client).await?;
        },
        FetchTarget::Category(name) => {
            let result = client.get_category_by_slug(&name).await?;
            print_category(result).await?;
        },
        FetchTarget::Calendar(_) => {
            // let places = client.list_places().await?;
            // let Some(place_id) = places.infos.iter().find_map(|info| {
            //     let place = info.get("place")?;
            //     let slug = place.get("slug").and_then(|x| x.as_str())?;
            //     if slug != name {
            //         return None;
            //     }
            //
            //     place.get("api_id").and_then(|x| x.as_str())
            // }) else {
            //     eprintln!("No such place");
            //     return Ok(EX_OK);
            // };

            unimplemented!();
        },
        FetchTarget::Place(name) => {
            let result = client.get_place_by_slug(&name).await?;
            print_place(result).await?;
        },
        // FetchTarget::Event(_) => {
        //     unimplemented!();
        // },
        FetchTarget::Unknown(resource) => {
            if let Ok(result) = client.get_category_by_slug(&resource).await {
                print_category(result).await?;
                return Ok(EX_OK);
            }

            if let Ok(result) = client.get_place_by_slug(&resource).await {
                print_place(result).await?;
                return Ok(EX_OK);
            }

            eprintln!("Unknown resource: {resource}");
            return Ok(EX_USAGE);
        },
    };

    Ok(EX_OK)
}
