// This is free and unencumbered software released into the public domain.

use asimov_luma_module::api::Client;
use asimov_module::SysexitsError::{self, *};
use clap::{Parser, Subcommand};
use clientele::StandardOptions;
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

    #[clap(about = "Get category info by slug.")]
    GetCategory { slug: String },

    #[clap(about = "Get calendar info by id.")]
    GetCalendar { id: String },

    #[clap(about = "Get events for a calendar by id.")]
    GetCalendarEvents {
        id: String,
        cursor: Option<String>,
        limit: Option<u32>,
    },

    #[clap(about = "Get place info by id.")]
    GetPlaceById { id: String },

    #[clap(about = "Get place info by slug.")]
    GetPlaceBySlug { slug: String },

    #[clap(about = "Get events for a place by id.")]
    GetPlaceEvents {
        id: String,
        cursor: Option<String>,
        limit: Option<u32>,
    },

    #[clap(about = "Get event info by id.")]
    GetEvent { id: String },

    #[clap(about = "Get nearby events.")]
    GetNearbyEvents,

    #[clap(about = "Get nearby events for category by slug.")]
    GetNearbyEventsForCategory { slug: String },
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
    match options.command {
        Commands::ListFeaturedCalendars => {
            let result = client.get_featured_calendars().await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::ListCategories => {
            let result = client.list_categories().await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::ListPlaces => {
            let result = client.list_places().await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::GetCategory { slug } => {
            let result = client.get_category_by_slug(slug).await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::GetCalendar { id } => {
            let result = client.get_calendar_by_id(id).await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::GetCalendarEvents { id, cursor, limit } => {
            let result = client.get_calendar_events(id, cursor, limit).await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::GetPlaceById { id } => {
            let result = client.get_place_by_id(id).await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::GetPlaceBySlug { slug } => {
            let result = client.get_place_by_slug(slug).await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::GetPlaceEvents { id, cursor, limit } => {
            let result = client.get_place_events(id, cursor, limit).await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::GetEvent { id } => {
            let result = client.get_event(id).await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::GetNearbyEvents => {
            let result = client.get_nearby_events::<String>(None).await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
        Commands::GetNearbyEventsForCategory { slug } => {
            let result = client.get_nearby_events(Some(slug)).await?;
            println!("{}", colored_json::to_colored_json_auto(&result)?);
        },
    }

    Ok(EX_OK)
}
