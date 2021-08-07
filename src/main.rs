use color_eyre::eyre::eyre;
use indoc::indoc;
use std::{env, fs::OpenOptions, io::Write};

use crate::encoding::PrinterString;

mod encoding;

fn main() -> color_eyre::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    color_eyre::install()?;

    const PRINTER_DEVICE_PATH: &str = "/dev/usb/lp4";

    let mut printer_file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(PRINTER_DEVICE_PATH)?;

    // const TEST_TEXT: &str = indoc! {"
    //     abcdefghijklmnopqrstuvwxyz
    //     ABCDEFGHIJKLMNOPQRSTUVWXYZ

    //     123456789

    //     !@#$%^&*()_+-={}|[]\\:\";'<>?,./~`
    //     \rCarriage Return Test
    //     Backspace TEST\u{0008}\u{0008}\u{0008}\u{0008}FUN!
    // "};

    // print!("{}", TEXT_TEXT);
    // printer_file.write_all(TEXT_TEXT.as_bytes())?;
    // writeln!(printer_file, "\u{2588}")?;
    // println!("\u{2588}");
    // printer_file.write_all(&[0xdb, 13, 13, 13, 13, 13, 13, 13, 13, 13])?;

    // println!("{:?}", PrinterString::from_str(&TEST_TEXT));
    // println!(
    //     "{:?}",
    //     PrinterString::from_str(&TEST_TEXT)
    //         .as_ref()
    //         .map(PrinterString::bytes)
    //         .map(Iterator::collect::<Vec<_>>)
    // );

    printer_file.write_all(
        &PrinterString::from_str(indoc! {"
            ╔════════════════════════════════════════╗
            ║     ÜÜÜ        amongus²        ÜÜÜ     ║
            ╚════════════════════════════════════════╝
        "})
        .ok_or_else(|| eyre!("Failed to convert text to printer compatible text"))?
        .bytes()
        .collect::<Vec<_>>(),
    )?;

    Ok(())
}
