use rppal::i2c::Error;
// #[cfg(feature = "visualize")]
use cli_table::{print_stdout, Cell, Style, Table};
// #[cfg(feature = "visualize")]
use rppal::i2c::I2c;
// #[cfg(feature = "visualize")]
use visualize::PrintTable;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub trait WriteRegister {
    /// Will write the value from self into device register
    ///
    /// # Errors
    /// Will error if unable to communicate with the device
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()>;
}

pub trait ReadRegister {
    /// Will read the value of the register from the device and return the new object
    ///
    /// # Errors
    /// Will error if unable to communicate with the device
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized;
}

#[allow(clippy::struct_excessive_bools)]
#[derive(PrintTable)]
pub struct PowerManagement1 {
    pub device_reset: bool,
    pub sleep: bool,
    pub accel_cycle: bool,
    pub gyro_standby: bool,
    pub temperature_disabled: bool,
    pub clock_select: u8,
}
impl PowerManagement1 {
    pub const ADDRESS: u8 = 0x6B;
}

impl WriteRegister for PowerManagement1 {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let write_buf = [u8::from(self.device_reset) << 7
            | u8::from(self.sleep) << 6
            | u8::from(self.accel_cycle) << 5
            | u8::from(self.gyro_standby) << 4
            | u8::from(self.temperature_disabled) << 3
            | (self.clock_select & 0b111); 1];
        i2c.block_write(Self::ADDRESS, &write_buf)?;
        Ok(())
    }
}
impl ReadRegister for PowerManagement1 {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let mut read_buf = [0; 1];
        i2c.block_read(Self::ADDRESS, &mut read_buf)?;
        let read_buf = read_buf[0];
        println!("Read {read_buf:b}");
        Ok(Self {
            device_reset: (read_buf >> 7) != 0,
            sleep: ((read_buf >> 6) & 1) != 0,
            accel_cycle: ((read_buf >> 5) & 1) != 0,
            gyro_standby: ((read_buf >> 4) & 1) != 0,
            temperature_disabled: ((read_buf >> 3) & 1) != 0,
            clock_select: read_buf & 0b111,
        })
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(PrintTable)]
pub struct PowerManagement2 {
    pub fifo_lp: bool,
    pub stby_xaccel: bool,
    pub stby_yaccel: bool,
    pub stby_zaccel: bool,
    pub stby_xgyro: bool,
    pub stby_ygyro: bool,
    pub stby_zgyro: bool,
}
impl PowerManagement2 {
    pub const ADDRESS: u8 = 0x6C;
}
impl WriteRegister for PowerManagement2 {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let write_buf = [u8::from(self.fifo_lp) << 7
            | u8::from(self.stby_xaccel) << 5
            | u8::from(self.stby_yaccel) << 4
            | u8::from(self.stby_zaccel) << 3
            | u8::from(self.stby_xgyro) << 2
            | u8::from(self.stby_ygyro) << 1
            | u8::from(self.stby_zgyro); 1];
        i2c.block_write(0x6C, &write_buf)?;
        Ok(())
    }
}
impl ReadRegister for PowerManagement2 {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let mut read_buf = [0; 1];
        i2c.block_read(Self::ADDRESS, &mut read_buf)?;
        let read_buf = read_buf[0];
        Ok(Self {
            fifo_lp: (read_buf >> 7) != 0,
            stby_xaccel: ((read_buf >> 5) & 1) != 0,
            stby_yaccel: ((read_buf >> 4) & 1) != 0,
            stby_zaccel: ((read_buf >> 3) & 1) != 0,
            stby_xgyro: ((read_buf >> 2) & 1) != 0,
            stby_ygyro: ((read_buf >> 1) & 1) != 0,
            stby_zgyro: (read_buf & 1) != 0,
        })
    }
}

#[derive(PrintTable)]
pub struct Config {
    // FirstInFirstOut mode.
    // 1 - when buffer is full additional writes will not be written,
    // 0 - when buffer is full additional writes will overwrite oldes data
    pub fifo_mode: bool,
    // Enables the FSYNC pin data to be sampled.
    // 0 - function disabled
    // 1 - TEMP_OUT_L[0]
    // 2 - GYRO_XOUT_L[0]
    // 3 - GYRO_YOUT_L[0]
    // 4 - GYRO_ZOUT_L[0]
    // 5 - ACCEL_XOUT_L[0]
    // 6 - ACCEL_YOUT_L[0]
    // 7 - ACCEL_ZOUT_L[0]
    pub ext_sync_set: u8,
    // Data low pass filter configuration
    pub dlpf_cfg: u8,
}
impl Config {
    const ADDRESS: u8 = 0x1A;
}
impl WriteRegister for Config {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let write_buf = [(u8::from(self.fifo_mode) << 6)
            | ((self.ext_sync_set & 0b111) << 3)
            | (self.dlpf_cfg & 0b111); 1];
        i2c.block_write(Self::ADDRESS, &write_buf)?;
        Ok(())
    }
}
impl ReadRegister for Config {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let mut read_buf = [0; 1];
        i2c.block_read(Self::ADDRESS, &mut read_buf)?;
        let read_buf = read_buf[0];
        Ok(Self {
            fifo_mode: (read_buf >> 6) & 1 != 0,
            ext_sync_set: (read_buf >> 3) & 0b111,
            dlpf_cfg: read_buf & 0b111,
        })
    }
}

pub struct GyroConfig {
    pub x_st: bool,
    pub y_st: bool,
    pub z_st: bool,
    // Gyro Full Scale Select:
    // 00 = ±250dps
    // 01 = ±500dps
    // 10 = ±1000dps
    // 11 = ±2000dps
    pub full_scale_select: u8,
    pub fchoice_b: u8,
}
impl GyroConfig {
    const ADDRESS: u8 = 0x1B;
}
impl WriteRegister for GyroConfig {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let write_buf = [(u8::from(self.x_st) << 7)
            | (u8::from(self.y_st) << 6)
            | (u8::from(self.z_st) << 5)
            | ((self.full_scale_select & 0b11) << 3)
            | (self.fchoice_b & 0b11); 1];
        i2c.block_write(Self::ADDRESS, &write_buf)?;
        Ok(())
    }
}
impl ReadRegister for GyroConfig {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let mut read_buf = [0; 1];
        i2c.block_read(Self::ADDRESS, &mut read_buf)?;
        let read_buf = read_buf[0];
        Ok(Self {
            x_st: (read_buf >> 7) != 0,
            y_st: ((read_buf >> 6) & 1) != 0,
            z_st: ((read_buf >> 5) & 1) != 0,
            full_scale_select: (read_buf >> 3) & 0b11,
            fchoice_b: (read_buf & 0b11),
        })
    }
}

pub struct AccelConfig1 {
    // X accel self-test
    pub x_st: bool,
    // Y accel self-test
    pub y_st: bool,
    // Z accel self-test
    pub z_st: bool,
    // Accel Full Scale Select:
    // 00 = ±2g
    // 01 = ±4g
    // 10 = ±8g
    // 11 = ±16g
    pub full_scale_select: u8,
}
impl AccelConfig1 {
    const ADDRESS: u8 = 0x1C;
}
impl WriteRegister for AccelConfig1 {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let write_buf = [(u8::from(self.x_st) << 7)
            | (u8::from(self.y_st) << 6)
            | (u8::from(self.z_st) << 5)
            | ((self.full_scale_select & 0b11) << 3); 1];
        i2c.block_write(Self::ADDRESS, &write_buf)?;
        Ok(())
    }
}
impl ReadRegister for AccelConfig1 {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let mut read_buf = [0; 1];
        i2c.block_read(Self::ADDRESS, &mut read_buf)?;
        let read_buf = read_buf[0];
        Ok(Self {
            x_st: (read_buf >> 7) != 0,
            y_st: ((read_buf >> 6) & 1) != 0,
            z_st: ((read_buf >> 5) & 1) != 0,
            full_scale_select: (read_buf >> 3) & 0b11,
        })
    }
}

pub struct AccelConfig2 {
    // Averaging filter settings for Low Power Accelerometer mode:
    // 0b00 = Average 4 samples
    // 0b01 = Average 8 samples
    // 0b10 = Average 16 samples
    // 0b11 = Average 32 samples
    pub dec2_cfg: u8,
    // Used to bypass DLPF
    pub accel_fchoice_b: bool,
    // Accelerometer low pass filter setting
    pub dlpf_cfg: u8,
}
impl AccelConfig2 {
    const ADDRESS: u8 = 0x1D;
}
impl WriteRegister for AccelConfig2 {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let write_buf = [((self.dec2_cfg & 0b11) << 4)
            | (u8::from(self.accel_fchoice_b) << 3)
            | (self.dlpf_cfg & 0b111); 1];
        i2c.block_write(Self::ADDRESS, &write_buf)?;
        Ok(())
    }
}
impl ReadRegister for AccelConfig2 {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let mut read_buf = [0; 1];
        i2c.block_read(Self::ADDRESS, &mut read_buf)?;
        let read_buf = read_buf[0];
        Ok(Self {
            dec2_cfg: ((read_buf >> 4) & 0b11),
            accel_fchoice_b: ((read_buf >> 3) & 1) != 0,
            dlpf_cfg: (read_buf & 0b111),
        })
    }
}
