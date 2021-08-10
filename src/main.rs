use color_eyre::eyre::{ContextCompat, WrapErr};
use escpos_rs::{command::Font, Instruction, PrintData, Printer, PrinterProfile};
use indoc::indoc;
use libusb::Context;
use std::{env, fmt::format, thread::sleep, time::Duration};

fn main() -> color_eyre::Result<()> {
    env::set_var("RUST_BACKTRACE", "full");
    color_eyre::install()?;

    let context = Context::new().wrap_err("Failed to get libusb context")?;

    let printer_details = PrinterProfile::builder(0x1a86, 0x7584)
        // .with_width((180.0 * 2.83) as u16)
        .with_font_width(Font::FontA, 42)
        .with_timeout(Duration::from_secs(10000))
        .build();

    let printer = Printer::with_context(&context, printer_details)?.wrap_err("No printer found")?;

    printer.raw([0x1B, 0x63, 0x30, PaperType::Roll as u8])?;

    printer.table_2([
        ("Name", "Age"),
        ("Nate", "19"),
        ("Zach", "18"),
        ("Evelyn", "14"),
    ])?;

    printer.instruction(
        &Instruction::DuoTable {
            font: Font::FontA,
            header: ("A".to_string(), "B".to_string()),
            name: "A".to_string(),
        },
        Some(
            &PrintData::builder()
                .add_duo_table(
                    "A",
                    [("C", "D")]
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect(),
                )
                .build(),
        ),
    )?;

    code93_barcode(&printer, b"Hello World!")?;

    printer.cut()?;

    enum PaperType {
        Roll = 0b0011,
        Slip = 0b0100,
        Validation = 0b1000,
    }

    // printer.raw([0x1B, 0x63, 0x30, PaperType::Slip as u8])?; // ESC c 0

    // printer.println(indoc! {"
    //     123456789012345678901234567890123456789012345
    //     ---------------------------------------------
    // "})?;
    // for (var, val) in env::vars().take(10) {
    //     printer.println(format!("{} = {}", var, val))?;
    //     sleep(Duration::from_secs(1));
    // }

    // printer.println("Hello")?;

    // printer.raw([0x1B, 0x63, 0x31, PaperType::Validation as u8])?; // ESC c 1
    // printer.raw([0x1B, 0x63, 0x30, PaperType::Validation as u8])?;

    // printer.println("World")?;

    printer.raw([0x1B, 0x63, 0x30, PaperType::Roll as u8])?;

    Ok(())
}

fn code93_barcode(printer: &Printer, data: &[u8]) -> Result<(), escpos_rs::Error> {
    assert!(data.len() >= 1, "Data must have at least one item");
    assert!(data.len() <= 255, "Data cannot have more than 255 items");

    assert!(data.iter().all(|c| matches!(c, 0x00..=0x7F)));

    printer.raw([0x1D, 0x54, 0x01])?; // GS T

    printer.raw([0x1D, 0x6B, 0x48, data.len() as u8])?; // GS k

    printer.raw(data)?;

    Ok(())
}
