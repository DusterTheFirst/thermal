use std::env;

use color_eyre::eyre::{ContextCompat, WrapErr};
use rusb::Context;
use thermal::{Justification, PaperType, Printer, SlipSide};
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> color_eyre::Result<()> {
    if env::var("RUST_LIB_BACKTRACE").is_err() {
        env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    color_eyre::install()?;

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let context = Context::new().wrap_err("Failed to get libusb context")?;

    // let printer_details = PrinterProfile::builder(0x1a86, 0x7584)
    //     // .with_width((180.0 * 2.83) as u16)
    //     .with_font_width(Font::FontA, 42)
    //     .with_timeout(Duration::from_secs(10000))
    //     .build();

    // let printer = Printer::with_context(&context, printer_details)?.wrap_err("No printer found")?;

    let printer = Printer::builder(0x1a86, 0x7584)
        .connect(&context)
        .wrap_err("Failed to connect to the printer")?
        .wrap_err("No printer found")?;

    info!("Connected!"); // TODO: more logging
    // TODO: UI

    printer.init()?;

    // printer.raw([0x1B, 0x63, 0x30, PaperType::Roll as u8])?;

    // printer.table_2([
    //     ("Name", "Age"),
    //     ("Nate", "19"),
    //     ("Zach", "18"),
    //     ("Evelyn", "14"),
    // ])?;

    // printer.instruction(
    //     &Instruction::DuoTable {
    //         font: Font::FontA,
    //         header: ("A".to_string(), "B".to_string()),
    //         name: "A".to_string(),
    //     },
    //     Some(
    //         &PrintData::builder()
    //             .add_duo_table(
    //                 "A",
    //                 [("C", "D")]
    //                     .iter()
    //                     .map(|(a, b)| (a.to_string(), b.to_string()))
    //                     .collect(),
    //             )
    //             .build(),
    //     ),
    // )?;

    printer.justify(Justification::Center)?;

    printer.barcode_upc_a(&b"69420000000")?;

    printer.justify(Justification::Left)?;

    printer.carriage_reset(false)?; // GS T (Print data in the current print buffer)

    printer.cut()?;

    // printer.raw([0x1B, 0x53])?; // Select Standard mode: ESC S
    printer.paper_type(PaperType::Slip)?;
    printer.slip_side(SlipSide::Face)?;
    // printer.raw([0x1B, 0x63, 0x31, 1 >> 6])?; // ESC c 1

    // printer.justify(Justification::Center)?;
    // printer.raw([0x0C])?; // Form feed
    // printer.println("Test")?;
    // barcode_upc_a(&printer, b"69420000000")?;

    // printer.println(indoc! {"
    //     123456789012345678901234567890123456789012345
    //     ---------------------------------------------
    // "})?;
    for (var, val) in env::vars().take(10) {
        printer.println(format!("{} = {}", var, val))?;
        // sleep(Duration::from_secs(1));
    }

    printer.slip_side(SlipSide::Back)?;

    printer.println("Hello")?;
    printer.println("World")?;

    Ok(())
}
