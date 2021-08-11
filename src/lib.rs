use std::{
    cell::RefCell,
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

use codepage_437::{ToCp437, CP437_CONTROL};
use error::{BarcodeError, PrinterError, TextError};

mod error;

/// Text justification
#[repr(u8)]
pub enum Justification {
    Left = 0x00,
    Center = 0x01,
    Right = 0x02,
}

/// The type of paper to print to
#[repr(u8)]
pub enum PaperType {
    Roll = 0b0011,
    Slip = 0b0100,
    Validation = 0b1000, // TODO: Unsupported on my models
}

/// The side of the slip paper to print to
#[repr(u8)]
pub enum SlipSide {
    Face = 0x04,
    Back = 0x44,
}

pub struct PrinterBuilder {}

impl PrinterBuilder {
    /// Finalize the builder and connect to the printer at the given path
    pub fn connect<P: AsRef<Path>>(self, path: P) -> io::Result<Printer> {
        let device = OpenOptions::new()
            .append(true)
            .read(true)
            .open(path)
            .map(RefCell::new)?;

        Ok(Printer { device })
    }
}

pub struct Printer {
    device: RefCell<File>,
}

impl Printer {
    /// Create a printer builder to configure printer settings
    pub fn builder() -> PrinterBuilder {
        PrinterBuilder {}
    }
}

impl Printer {
    const ESC: u8 = 0x1B;
    const GS: u8 = 0x1D;
    const FS: u8 = 0x1C;

    /// Write raw bytes to the printer
    fn raw<D: AsRef<[u8]>>(&self, data: D) -> Result<(), PrinterError> {
        self.device.borrow_mut().write_all(data.as_ref())?;

        Ok(())
    }

    // Print some text
    pub fn print<S: AsRef<str> + Debug>(&self, text: S) -> Result<(), TextError<S>> {
        let cp437 = text.as_ref().to_cp437(&CP437_CONTROL);

        match cp437 {
            Ok(data) => Ok(self.raw(&data)?),
            Err(err) => Err(TextError::Cp437 { err, text }),
        }
    }

    // Print some text followed by a newline
    pub fn println<S: AsRef<str> + Debug>(&self, text: S) -> Result<(), TextError<S>> {
        self.print(text)?;

        self.raw([b'\n'])?;

        Ok(())
    }

    /// Initialize printer
    ///
    /// Clears the data in the print buffer and resets the printer modes to the modes
    /// that were in effect when the power was turned on.
    /// Keeps the following data:
    /// - Offline response setting.
    /// - Contents defined for the NV graphics (NV bit image).
    /// - Contents stored in the NV user memory.
    /// - Setting value specified with GS ( E.
    /// - Maintenance counter value .
    /// - Customizing assignment for the ASB status bit.
    /// - Macro definition data.
    /// - The image scanning results in the NV memory for image data storage.
    ///
    /// ASCII: ESC @
    pub fn init(&self) -> Result<(), PrinterError> {
        self.raw([Self::ESC, b'@'])
    }

    /// Select cut mode and cut paper
    ///
    /// Executes paper cutting
    ///
    /// ASCII: GS V **m** **n**
    // TODO: cut pad, cut default pad, cut no pad
    pub fn cut(&self) -> Result<(), PrinterError> {
        // Feeds paper to (cutting position + [n × vertical motion unit]) and
        // executes a full cut (cuts the paper completely).
        self.raw([Self::GS, b'V', 65, 0x96])
    }

    /// Select justification
    ///
    /// In standard mode, aligns all the data in one line to the selected layout
    ///
    /// ASCII: ESC a **n**
    pub fn justify(&self, justification: Justification) -> Result<(), PrinterError> {
        self.raw([Self::ESC, b'a', justification as u8])
    }

    /// Select paper type(s) for printing
    ///
    /// Selects the active sheet(s) for printing
    ///
    /// ASCII: ESC c 0 **n**
    pub fn paper_type(&self, paper_type: PaperType) -> Result<(), PrinterError> {
        self.raw([Self::ESC, b'c', b'0', paper_type as u8])
    }

    /// Select the side off the slip (face or back)
    ///
    /// Selects slip as the active sheet and selects the side of the slip to be printed
    ///
    /// ASCII: GS ( G **pL** **pH** **fn** **m** <Function 48>
    pub fn slip_side(&self, slip_side: SlipSide) -> Result<(), PrinterError> {
        self.raw([Self::GS, b'(', b'G', 0x02, 0x00, 48, slip_side as u8])
    }

    /// Set print position to the beginning of print line
    ///
    /// In standard mode, moves the print position to the beginning of the print line after performing the operation
    ///
    /// ASCII: GS T **n**
    pub fn carriage_reset(&self, cancel_print_buffer: bool) -> Result<(), PrinterError> {
        self.raw([
            Self::GS,
            b'T',
            if cancel_print_buffer { 0x00 } else { 0x01 },
        ])
    }

    /// Select standard mode
    ///
    /// Switches from page mode or double-density page mode to standard mode.
    ///
    /// ASCII: ESC S
    pub fn standard_mode(&self) -> Result<(), PrinterError> {
        self.raw([Self::ESC, b'S'])
    }

    /// Select page mode
    ///
    /// Switches from standard mode to page mode.
    ///
    /// ASCII: ESC L
    pub fn page_mode(&self) -> Result<(), PrinterError> {
        self.raw([Self::ESC, b'L'])
    }

    /// Select double-density page mode
    ///
    /// Switches from standard mode to double-density page mode.
    ///
    /// ASCII: FS L
    pub fn double_density_page_mode(&self) -> Result<(), PrinterError> {
        self.raw([Self::FS, b'L'])
    }

    pub fn barcode_upc_a<D: AsRef<[u8]> + Debug>(&self, data: D) -> Result<(), BarcodeError<D>> {
        let data_len = data.as_ref().len();

        if !(11..=12).contains(&data_len) {
            return Err(BarcodeError::InvalidSize {
                expected: 11..=12,
                provided: data_len,
            });
        }

        for (i, &c) in data.as_ref().iter().enumerate() {
            match c {
                b'0'..=b'9' => {}
                _ => {
                    return Err(BarcodeError::InvalidChar {
                        data,
                        position: i,
                        range: b'0'..=b'9',
                    })
                }
            }
        }

        self.carriage_reset(false)?;

        // Select print position of HRI characters
        self.raw([0x1D, 0x48, 0x02])?; // GS H (Below the bar code)

        // Print bar code
        self.raw([0x1D, 0x6B, 65, data_len as u8])?; // GS k UPC-A (B) n

        // Bar code data
        self.raw(data.as_ref())?; // d1 ... dn

        Ok(())
    }
}
