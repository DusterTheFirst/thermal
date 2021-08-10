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

    barcode_upc_a(&printer, b"69420000000")?;

    printer.raw([0x1D, 0x54, 0x01])?; // GS T (Print data in the current print buffer)

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

fn barcode_upc_a(printer: &Printer, data: &[u8; 11]) -> Result<(), escpos_rs::Error> {
    for (i, &c) in data.iter().enumerate() {
        match c {
            b'0'..=b'9' => {}
            _ => panic!(
                "Invalid data '{}' at position {}. Must be one of: 0~9",
                char::from_u32(c as u32).unwrap_or('?'),
                i
            ),
        }
    }

    // Set print position to the beginning of print line
    printer.raw([0x1D, 0x54, 0x01])?; // GS T (Print data in the current print buffer)

    printer.justify(Justification::Center)?;

    // Select print position of HRI characters
    printer.raw([0x1D, 0x48, 0x02])?; // GS H (Below the bar code)

    // Print bar code
    printer.raw([0x1D, 0x6B, 65, data.len() as u8])?; // GS k UPC-A (B) n

    // Bar code data
    printer.raw(data)?; // d1 ... dn

    printer.justify(Justification::Left)?;

    Ok(())
}

enum Justification {
    Left = 0x00,
    Center = 0x01,
    Right = 0x02,
}

trait PrinterExt {
    const ESC: u8 = 0x1B;

    fn justify(&self, justification: Justification) -> Result<(), escpos_rs::Error>;
}

impl PrinterExt for Printer<'_> {
    fn justify(&self, justification: Justification) -> Result<(), escpos_rs::Error> {
        self.raw([Self::ESC, b'a', justification as u8])
    }
}
