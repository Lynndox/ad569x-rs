#![no_std]

use embedded_hal::i2c::I2c;

/// AD569x commands
pub enum Command {
    /// No operation command.
    NOP = 0x00,
    /// Write to the input register.
    WriteInput = 0x10,
    /// Update the DAC register.
    UpdateDAC = 0x20,
    /// Write to the input register and update the DAC register.
    WriteDACAndInput = 0x30,
    /// Write to the control register.
    WriteControl = 0x40,
}

/// AD569x operating modes
pub enum OperatingMode {
    /// Normal operating mode.
    NormalMode = 0x00,
    /// 1k Ohm output impedance mode.
    Output1kImpedance = 0x01,
    /// 100k Ohm output impedance mode.
    Output100kImpedance = 0x02,
    /// Tristate output mode.
    OutputTristate = 0x03,
}

pub struct AdafruitAD569x<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C: I2c> AdafruitAD569x<I2C> {
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self { i2c, addr }
    }

    /// Initialize the AD569x chip for communication.
    ///
    /// Will perform a soft reset and configure for normal mode,
    /// with Vref on, and 1x gain output.
    pub fn begin(&mut self) -> Result<(), I2C::Error> {
        self.reset()?;
        self.set_mode(OperatingMode::NormalMode, true, false)?;

        Ok(())
    }

    /// Write a 16-bit value to the DAC register... does NOT output it!
    ///
    /// This function writes a 16-bit value to the input register of the AD569x chip.
    /// The data does not appear on the output of the DAC till you run `update_dac()`!
    pub fn write_dac(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write(Command::WriteInput, value)
    }

    /// Update the DAC register from the input register.
    ///
    /// This function sends the UPDATE_DAC command to the AD569x chip to update
    /// the DAC register based on the value stored in the input register.
    pub fn update_dac(&mut self) -> Result<(), I2C::Error> {
        self.write(Command::UpdateDAC, 0x00)
    }

    /// Write a 16-bit value to the input register and update the DAC
    /// register.
    ///
    /// This function writes a 16-bit value to the input register and then updates
    /// the DAC register of the AD569x chip in a single operation
    pub fn write_update_dac(&mut self, value: u16) -> Result<(), I2C::Error> {
        self.write(Command::WriteDACAndInput, value)
    }

    /// Soft-reset the AD569x chip.
    ///
    /// This function writes 0x8000 to the control register of the AD569x chip
    /// to perform a reset operation. Resets the DAC to zero-scale and
    /// resets the input, DAC, and control registers to their default values.
    ///
    /// Note: The original driver implies the write will return an error as it "resets before it naks".
    /// What that means, I have no idea.
    pub fn reset(&mut self) -> Result<(), I2C::Error> {
        self.write(Command::WriteControl, 0x8000)
    }

    /// Set the operating mode, reference, and gain for the AD569x chip.
    ///
    /// This function writes to the control register of the AD569x chip to set
    /// the operating mode, enable or disable the reference, and set the gain.
    pub fn set_mode(
        &mut self,
        mode: OperatingMode,
        enable_ref: bool,
        gain_2x: bool,
    ) -> Result<(), I2C::Error> {
        let data =
            0x0u16 | ((mode as u16) << 13) | ((enable_ref as u16) << 12) | (gain_2x as u16) << 11;

        self.write(Command::WriteControl, data)
    }
}

impl<I2C: I2c> AdafruitAD569x<I2C> {
    fn write(&mut self, command: Command, data: u16) -> Result<(), I2C::Error> {
        let [high_byte, low_byte] = data.to_be_bytes();

        self.i2c
            .write(self.addr, &[command as u8, high_byte, low_byte])
    }
}
