use color_eyre::eyre::{ContextCompat, WrapErr};
// use escpos_rs::{command::Font, Instruction, PrintData, Printer, PrinterProfile};
use indoc::indoc;
// use libusb::Context;
use std::{
    borrow::BorrowMut,
    cell::Cell,
    env,
    fmt::format,
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
    sync::RwLock,
    thread::sleep,
    time::Duration,
};

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

    let printer = Printer::connect("/dev/usb/lp4").wrap_err("Failed to connect to the printer")?;

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

    printer.barcode_upc_a(b"69420000000")?;

    printer.justify(Justification::Left)?;

    printer.raw([0x1D, 0x54, 0x01])?; // GS T (Print data in the current print buffer)

    printer.cut()?;

    // printer.raw([0x1B, 0x53])?; // Select Standard mode: ESC S
    printer.paper_type(PaperType::Slip)?;
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
    //     sleep(Duration::from_secs(1));
    // }

    // printer.println("Hello")?;

    // printer.raw([0x1B, 0x63, 0x31, PaperType::Validation as u8])?; // ESC c 1
    // printer.raw([0x1B, 0x63, 0x30, PaperType::Validation as u8])?;

    // printer.println("World")?;

    printer.paper_type(PaperType::Roll)?;

    Ok(())
}

enum Justification {
    Left = 0x00,
    Center = 0x01,
    Right = 0x02,
}

enum PaperType {
    Roll = 0b0011,
    Slip = 0b0100,
    Validation = 0b1000,
}

struct Printer(RwLock<File>);

impl Printer {
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        OpenOptions::new()
            .append(true)
            .read(true)
            .open(path)
            .map(RwLock::new)
            .map(Self)
    }
}

impl Printer {
    const ESC: u8 = 0x1B;
    const GS: u8 = 0x1D;

    fn raw<D: AsRef<[u8]>>(&self, data: D) -> io::Result<()> {
        self.0.write().unwrap().write_all(data.as_ref())
    }

    pub fn init(&self) -> io::Result<()> {
        self.raw([Self::ESC, b'@'])
    }

    // TODO: cut pad, cut default pad, cut no pad
    pub fn cut(&self) -> io::Result<()> {
        // Feeds paper to (cutting position + [n × vertical motion unit]) and
        // executes a full cut (cuts the paper completely).
        self.raw([Self::GS, b'V', 65, 0x96])
    }

    pub fn justify(&self, justification: Justification) -> io::Result<()> {
        self.raw([Self::ESC, b'a', justification as u8])
    }

    pub fn paper_type(&self, paper_type: PaperType) -> io::Result<()> {
        self.raw([Self::ESC, b'c', b'0', paper_type as u8])
    }

    pub fn barcode_upc_a(&self, data: &[u8; 11]) -> io::Result<()> {
        for (i, &c) in data.iter().enumerate() {
            match c {
                b'0'..=b'9' => {}
                _ => panic!(
                    "Invalid data '{}' at position {}. Must be one of: 0~9",
                    char::from_u32(c as u32).unwrap_or('?'),
                    i
                ), // FIXME: error, not panic
            }
        }

        // Set print position to the beginning of print line
        self.raw([0x1D, 0x54, 0x01])?; // GS T (Print data in the current print buffer)

        // Select print position of HRI characters
        self.raw([0x1D, 0x48, 0x02])?; // GS H (Below the bar code)

        // Print bar code
        self.raw([0x1D, 0x6B, 65, data.len() as u8])?; // GS k UPC-A (B) n

        // Bar code data
        self.raw(data)?; // d1 ... dn

        self.justify(Justification::Left)?;

        Ok(())
    }
}
