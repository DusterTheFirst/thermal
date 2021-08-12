use std::{env, thread::sleep, time::Duration};

use color_eyre::eyre::WrapErr;
use thermal::{Justification, PaperType, Printer, SlipSide};

fn main() -> color_eyre::Result<()> {
    env::set_var("RUST_BACKTRACE", "full");
    color_eyre::install()?;

    // let context = Context::new().wrap_err("Failed to get libusb context")?;

    // let printer_details = PrinterProfile::builder(0x1a86, 0x7584)
    //     // .with_width((180.0 * 2.83) as u16)
    //     .with_font_width(Font::FontA, 42)
    //     .with_timeout(Duration::from_secs(10000))
    //     .build();

    // let printer = Printer::with_context(&context, printer_details)?.wrap_err("No printer found")?;

    let printer = Printer::builder()
        .connect("/dev/usb/lp4")
        .wrap_err("Failed to connect to the printer")?;

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
