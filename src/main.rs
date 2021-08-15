use std::env;

use color_eyre::eyre::{ContextCompat, WrapErr};
use serenity::Client;
use thermal::{rusb::Context, Printer};
use tokio::runtime::Builder;
use tracing::{debug, info, instrument, warn};
use tracing_subscriber::EnvFilter;

use crate::handler::Handler;

mod handler;

const APPLICATION_ID: u64 = 876269040316850198;

const PRINTER_VID: u16 = 0x1a86;
const PRINTER_PID: u16 = 0x7584;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    setup_environment()?;

    let context = Context::new().wrap_err("Failed to get libusb context")?;

    let printer = Printer::builder(PRINTER_VID, PRINTER_PID)
        .connect(&context)
        .wrap_err("Failed to connect to the printer")?
        .wrap_err("No printer found")?;

    info!("Connected to printer!");

    printer.init()?;

    debug!("Printer Initializing...");

    warn!("Assuming printer connected, no way of knowing...");
    info!("Starting up discord bot...");

    let token = env::var("DISCORD_TOKEN").wrap_err("DISCORD_TOKEN must be set")?;

    let mut client: Client = Client::builder(token)
        .event_handler(Handler)
        .application_id(APPLICATION_ID)
        .await
        .expect("Error creating client");

    ctrlc::set_handler({
        let shard = client.shard_manager.clone();

        move || {
            let shard = shard.clone();

            let rt = Builder::new_current_thread().enable_all().build().unwrap();

            info!("Shutting down shard");

            rt.block_on(async move { shard.lock().await.shutdown_all().await })
        }
    })
    .wrap_err("Failed to set the ctrl+c handler")?;

    client.start().await?;

    // printer.justify(Justification::Center)?;

    // printer.barcode_upc_a(&b"69420000000")?;

    // printer.justify(Justification::Left)?;

    // printer.carriage_reset(false)?;

    // printer.cut()?;

    // printer.raw([0x1B, 0x53])?; // Select Standard mode: ESC S
    // printer.paper_type(PaperType::Slip)?;
    // printer.slip_side(SlipSide::Face)?;
    // printer.raw([0x1B, 0x63, 0x31, 1 >> 6])?; // ESC c 1

    // printer.justify(Justification::Center)?;
    // printer.raw([0x0C])?; // Form feed
    // printer.println("Test")?;
    // barcode_upc_a(&printer, b"69420000000")?;

    // printer.println(indoc! {"
    //     123456789012345678901234567890123456789012345
    //     ---------------------------------------------
    // "})?;
    // for (var, val) in env::vars().take(10) {
    //     printer.println(format!("{} = {}", var, val))?;
    //     // sleep(Duration::from_secs(1));
    // }

    // printer.slip_side(SlipSide::Back)?;

    // printer.println("Hello")?;
    // printer.println("World")?;

    Ok(())
}

#[instrument]
fn setup_environment() -> color_eyre::Result<()> {
    if env::var("RUST_LIB_BACKTRACE").is_err() {
        env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    color_eyre::install()?;

    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            if cfg!(debug_assertions) {
                "info,thermal=trace,discord_thermal=trace"
            } else {
                "info"
            },
        );
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
