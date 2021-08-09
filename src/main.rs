use color_eyre::eyre::{ContextCompat, WrapErr};
use escpos_rs::{command::Font, Instruction, PrintData, Printer, PrinterProfile};
use indoc::indoc;
use libusb::{ConfigDescriptor, Context, Device, DeviceHandle};
use std::{
    env,
    fs::OpenOptions,
    io::{stdout, Read, Write},
    iter,
    time::Duration,
};

fn main() -> color_eyre::Result<()> {
    env::set_var("RUST_BACKTRACE", "full");
    color_eyre::install()?;

    let context = Context::new().wrap_err("Failed to get libusb context")?;

    // let printer_details = PrinterProfile::builder(0x1a86, 0x7584)
    //     // .with_width((180.0 * 2.83) as u16)
    //     .with_font_width(Font::FontA, 42)
    //     .build();

    // let printer = Printer::with_context(&context, printer_details)?.wrap_err("No printer found")?;

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

    // printer.cut()?;

    let (vid, pid) = (0x1a86, 0x7584);
    let device: Device = context
        .devices()?
        .iter()
        .find(|dev| {
            let descriptor = dev.device_descriptor().unwrap();

            descriptor.vendor_id() == vid && descriptor.product_id() == pid
        })
        .wrap_err("No device found")?;

    let mut handle: DeviceHandle = device.open()?;

    const IF: u8 = 0;

    if handle.kernel_driver_active(IF)? {
        handle.detach_kernel_driver(IF)?;
    }

    handle.claim_interface(IF)?;

    let config_descriptor: ConfigDescriptor = device.active_config_descriptor()?;

    let mut endpoint_in = None;
    let mut endpoint_out = None;

    for interface in config_descriptor.interfaces() {
        for descriptor in interface.descriptors() {
            for endpoint in descriptor.endpoint_descriptors() {
                match (endpoint.transfer_type(), endpoint.direction()) {
                    (libusb::TransferType::Bulk, libusb::Direction::Out) => {
                        endpoint_out.get_or_insert(endpoint.address());
                    }
                    (libusb::TransferType::Bulk, libusb::Direction::In) => {
                        endpoint_in.get_or_insert(endpoint.address());
                    }
                    _ => (),
                };
            }
        }
    }

    let (endpoint_in, endpoint_out) = dbg!(
        endpoint_in.wrap_err("Could not find a suitable input endpoint")?,
        endpoint_out.wrap_err("Could not find a suitable output endpoint")?,
    );

    const TIMEOUT: Duration = Duration::from_secs(3);

    // handle.write_bulk(endpoint_out, &[0x10, 0x04, 0x01], TIMEOUT)?; // DLE EOT 1
    // handle.write_bulk(endpoint_out, &[0x1D, 0x72, 0x01], TIMEOUT)?; // Transmit Status: GS r n
    // handle.write_bulk(endpoint_out, &[0x1d, 0x56, 0x41, 0x96], TIMEOUT)?; // CUT
    // handle.write_bulk(endpoint_out, &[0x1B, 0x40], TIMEOUT)?;
    // handle.write_bulk(endpoint_out, &[0x1D, 0x49, 0x1], TIMEOUT)?; // Transmit Printer ID: GS I

    // let mut buffer = [0; 32];

    // let len = handle.read_bulk(endpoint_in, &mut buffer, TIMEOUT)?;

    // println!("{:#x?}", &buffer[..len]);

    enum PaperType {
        Roll = 0b0011,
        Slip = 0b0100,
        Validation = 0b1000,
    }

    // ESC c 0
    handle.write_bulk(
        endpoint_out,
        &[0x1B, 0x63, 0x30, PaperType::Slip as u8],
        TIMEOUT,
    )?;

    handle.write_bulk(endpoint_out, b"Hello", TIMEOUT)?;

    // handle.write_bulk(
    //     endpoint_out,
    //     &[0x1B, 0x63, 0x30, PaperType::Validation as u8],
    //     TIMEOUT,
    // )?;

    // handle.write_bulk(endpoint_out, b"World", TIMEOUT)?;

    // handle.write_bulk(
    //     endpoint_out,
    //     &[0x1B, 0x63, 0x30, PaperType::Roll as u8],
    //     TIMEOUT,
    // )?;

    Ok(())
}
