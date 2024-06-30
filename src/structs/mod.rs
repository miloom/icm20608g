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
        let write_buf = u8::from(self.device_reset) << 7
            | u8::from(self.sleep) << 6
            | u8::from(self.accel_cycle) << 5
            | u8::from(self.gyro_standby) << 4
            | u8::from(self.temperature_disabled) << 3
            | (self.clock_select & 0b111);
        i2c.smbus_write_byte(Self::ADDRESS, write_buf)?;
        Ok(())
    }
}
impl ReadRegister for PowerManagement1 {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
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
        let write_buf = u8::from(self.fifo_lp) << 7
            | u8::from(self.stby_xaccel) << 5
            | u8::from(self.stby_yaccel) << 4
            | u8::from(self.stby_zaccel) << 3
            | u8::from(self.stby_xgyro) << 2
            | u8::from(self.stby_ygyro) << 1
            | u8::from(self.stby_zgyro);
        i2c.smbus_write_byte(Self::ADDRESS, write_buf)?;
        Ok(())
    }
}
impl ReadRegister for PowerManagement2 {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
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
        let write_buf = (u8::from(self.fifo_mode) << 6)
            | ((self.ext_sync_set & 0b111) << 3)
            | (self.dlpf_cfg & 0b111);
        i2c.smbus_write_byte(Self::ADDRESS, write_buf)?;
        Ok(())
    }
}
impl ReadRegister for Config {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self {
            fifo_mode: (read_buf >> 6) & 1 != 0,
            ext_sync_set: (read_buf >> 3) & 0b111,
            dlpf_cfg: read_buf & 0b111,
        })
    }
}

#[derive(PrintTable)]
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
        let write_buf = (u8::from(self.x_st) << 7)
            | (u8::from(self.y_st) << 6)
            | (u8::from(self.z_st) << 5)
            | ((self.full_scale_select & 0b11) << 3)
            | (self.fchoice_b & 0b11);
        i2c.smbus_write_byte(Self::ADDRESS, write_buf)?;
        Ok(())
    }
}
impl ReadRegister for GyroConfig {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self {
            x_st: (read_buf >> 7) != 0,
            y_st: ((read_buf >> 6) & 1) != 0,
            z_st: ((read_buf >> 5) & 1) != 0,
            full_scale_select: (read_buf >> 3) & 0b11,
            fchoice_b: (read_buf & 0b11),
        })
    }
}

#[derive(PrintTable)]
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
        let write_buf = (u8::from(self.x_st) << 7)
            | (u8::from(self.y_st) << 6)
            | (u8::from(self.z_st) << 5)
            | ((self.full_scale_select & 0b11) << 3);
        i2c.smbus_write_byte(Self::ADDRESS, write_buf)?;
        Ok(())
    }
}
impl ReadRegister for AccelConfig1 {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self {
            x_st: (read_buf >> 7) != 0,
            y_st: ((read_buf >> 6) & 1) != 0,
            z_st: ((read_buf >> 5) & 1) != 0,
            full_scale_select: (read_buf >> 3) & 0b11,
        })
    }
}

#[derive(PrintTable)]
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
        let write_buf = ((self.dec2_cfg & 0b11) << 4)
            | (u8::from(self.accel_fchoice_b) << 3)
            | (self.dlpf_cfg & 0b111);
        i2c.smbus_write_byte(Self::ADDRESS, write_buf)?;
        Ok(())
    }
}
impl ReadRegister for AccelConfig2 {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self {
            dec2_cfg: ((read_buf >> 4) & 0b11),
            accel_fchoice_b: ((read_buf >> 3) & 1) != 0,
            dlpf_cfg: (read_buf & 0b111),
        })
    }
}

#[derive(PrintTable)]
pub struct GyroOffset {
    // X offset to gyro to remove DC bias. Applied before write to register.
    pub xg_offs: i16,
    // Y offset to gyro to remove DC bias. Applied before write to register.
    pub yg_offs: i16,
    // Z offset to gyro to remove DC bias. Applied before write to register.
    pub zg_offs: i16,
}
impl GyroOffset {
    const ADDRESS_XH: u8 = 0x13;
    const ADDRESS_XL: u8 = 0x14;
    const ADDRESS_YH: u8 = 0x15;
    const ADDRESS_YL: u8 = 0x16;
    const ADDRESS_ZH: u8 = 0x17;
    const ADDRESS_ZL: u8 = 0x18;
}
#[allow(clippy::cast_sign_loss)]
impl WriteRegister for GyroOffset {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let x_high = (self.xg_offs >> 8) as u8;
        let x_low = (self.xg_offs & 0xFF) as u8;
        i2c.smbus_write_byte(Self::ADDRESS_XH, x_high)?;
        i2c.smbus_write_byte(Self::ADDRESS_XL, x_low)?;
        let y_high = (self.yg_offs >> 8) as u8;
        let y_low = (self.yg_offs & 0xFF) as u8;
        i2c.smbus_write_byte(Self::ADDRESS_YH, y_high)?;
        i2c.smbus_write_byte(Self::ADDRESS_YL, y_low)?;
        let z_high = (self.zg_offs >> 8) as u8;
        let z_low = (self.zg_offs & 0xFF) as u8;
        i2c.smbus_write_byte(Self::ADDRESS_ZH, z_high)?;
        i2c.smbus_write_byte(Self::ADDRESS_ZL, z_low)?;
        Ok(())
    }
}

impl ReadRegister for GyroOffset {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let x_high = i2c.smbus_read_byte(Self::ADDRESS_XH)?;
        let x_low = i2c.smbus_read_byte(Self::ADDRESS_XL)?;
        let y_high = i2c.smbus_read_byte(Self::ADDRESS_YH)?;
        let y_low = i2c.smbus_read_byte(Self::ADDRESS_YL)?;
        let z_high = i2c.smbus_read_byte(Self::ADDRESS_ZH)?;
        let z_low = i2c.smbus_read_byte(Self::ADDRESS_ZL)?;
        Ok(Self {
            xg_offs: (i16::from(x_high) << 8) | i16::from(x_low),
            yg_offs: (i16::from(y_high) << 8) | i16::from(y_low),
            zg_offs: (i16::from(z_high) << 8) | i16::from(z_low),
        })
    }
}

pub struct SampleRateDivider {
    pub smplrt_div: u8,
}
impl SampleRateDivider {
    const ADDRESS: u8 = 0x19;
}
impl WriteRegister for SampleRateDivider {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        i2c.smbus_write_byte(Self::ADDRESS, self.smplrt_div)?;
        Ok(())
    }
}
impl ReadRegister for SampleRateDivider {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let smplrt_div = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self { smplrt_div })
    }
}

pub struct LowPowerModeConf {
    // When set to true low-power gyroscope mode is enabled
    pub gyro_cycle: bool,
    // Averaging filter configuration for low-power gyroscope mode.
    pub g_avgcfg: u8,
    // Low-power accel output data rate
    // 0 0.24
    // 1 0.49
    // 2 0.98
    // 3 1.95
    // 4 3.91
    // 5 7.81
    // 6 15.63
    // 7 31.25
    // 8 62.50
    // 9 125
    // 10 250
    // 11 500
    pub lposc_clksel: u8,
}
impl LowPowerModeConf {
    const ADDRESS: u8 = 0x1E;
}
impl WriteRegister for LowPowerModeConf {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let write_buf = (u8::from(self.gyro_cycle) << 7)
            | ((self.g_avgcfg & 0b111) << 4)
            | (self.lposc_clksel & 0b1111);
        i2c.smbus_write_byte(Self::ADDRESS, write_buf)?;
        Ok(())
    }
}
impl ReadRegister for LowPowerModeConf {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self {
            gyro_cycle: ((read_buf >> 7) != 0),
            g_avgcfg: ((read_buf >> 4) & 0b111),
            lposc_clksel: (read_buf & 0b1111),
        })
    }
}

pub struct WakeOnMotion {
    // Threshold value for the Wake on Motion Interrupt for accelerometer
    pub wom_thr: u8,
}
impl WakeOnMotion {
    const ADDRESS: u8 = 0x1F;
}
impl WriteRegister for WakeOnMotion {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        i2c.smbus_write_byte(Self::ADDRESS, self.wom_thr)?;
        Ok(())
    }
}
impl ReadRegister for WakeOnMotion {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let wom_thr = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self { wom_thr })
    }
}

#[allow(clippy::struct_excessive_bools)]
pub struct FifoEnable {
    // Write TEMP_OUT_H and TEMP_OUT_L to the FIFO at the sample rate
    // If enabled, buffering of data occurs even if data path is in standby.
    pub temp_fifo_en: bool,
    // Write GYRO_XOUT_H and GYRO_XOUT_L to the FIFO at the sample rate
    // If enabled, buffering of data occurs even if data path is in standby.
    pub xg_fifo_en: bool,
    // Write GYRO_YOUT_H and GYRO_YOUT_L to the FIFO at the sample rate
    // If enabled, buffering of data occurs even if data path is in standby.
    pub yg_fifo_en: bool,
    // Write GYRO_ZOUT_H and GYRO_ZOUT_L to the FIFO at the sample rate
    // If enabled, buffering of data occurs even if data path is in standby.
    pub zg_fifo_en: bool,
    // write ACCEL_XOUT_H, ACCEL_XOUT_L, ACCEL_YOUT_H, ACCEL_YOUT_L,
    // ACCEL_ZOUT_H, and ACCEL_ZOUT_L to the FIFO at the sample rate;
    pub accel_fifo_en: bool,
}
impl FifoEnable {
    const ADDRESS: u8 = 0x23;
}
impl WriteRegister for FifoEnable {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let write_buf = (u8::from(self.temp_fifo_en) << 7)
            | (u8::from(self.xg_fifo_en) << 6)
            | (u8::from(self.yg_fifo_en) << 5)
            | (u8::from(self.zg_fifo_en) << 4)
            | (u8::from(self.accel_fifo_en) << 3);
        i2c.smbus_write_byte(Self::ADDRESS, write_buf)?;
        Ok(())
    }
}
impl ReadRegister for FifoEnable {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self {
            temp_fifo_en: (read_buf >> 7) != 0,
            xg_fifo_en: ((read_buf >> 6) & 1) != 0,
            yg_fifo_en: ((read_buf >> 5) & 1) != 0,
            zg_fifo_en: ((read_buf >> 4) & 1) != 0,
            accel_fifo_en: ((read_buf >> 3) & 1) != 0,
        })
    }
}

struct FsyncInterrupt {
    // This bit automatically sets to 1 when a FSYNC interrupt has been generated.
    // The bit clears to 0 after the register has been read.
    pub fsync_int: bool,
}
impl FsyncInterrupt {
    const ADDRESS: u8 = 0x36;
}
impl ReadRegister for FsyncInterrupt {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self {
            fsync_int: (read_buf >> 7) != 0,
        })
    }
}

#[allow(clippy::struct_excessive_bools)]
struct InterruptPinConfig {
    // 1 – The logic level for INT/DRDY pin is active low.
    // 0 – The logic level for INT/DRDY pin is active high.
    pub int_level: bool,
    // 1 – INT/DRDY pin is configured as open drain.
    // 0 – INT/DRDY pin is configured as push-pull.
    pub int_open: bool,
    // 1 – INT/DRDY pin level held until interrupt status is cleared.
    // 0 – INT/DRDY pin indicates interrupt pulse’s width is 50us.
    pub latch_int_en: bool,
    // 1 – Interrupt status is cleared if any read operation is performed.
    // 0 – Interrupt status is cleared only by reading INT_STATUS register
    pub int_rd_clear: bool,
    // 1 – The logic level for the FSYNC pin as an interrupt is active low.
    // 0 – The logic level for the FSYNC pin as an interrupt is active high
    pub fsync_int_level: bool,
    // When this bit is equal to 1, the FSYNC pin will trigger an interrupt when it
    // transitions to the level specified by FSYNC_INT_LEVEL. When this bit is
    // equal to 0, the FSYNC pin is disabled from causing an interrupt.
    pub fsync_int_mode_en: bool,
}
impl InterruptPinConfig {
    const ADDRESS: u8 = 0x37;
}
impl WriteRegister for InterruptPinConfig {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let write_buf = (u8::from(self.int_level) << 7)
            | (u8::from(self.int_open) << 6)
            | (u8::from(self.latch_int_en) << 5)
            | (u8::from(self.int_rd_clear) << 4)
            | (u8::from(self.fsync_int_level) << 3)
            | (u8::from(self.fsync_int_mode_en) << 2);
        i2c.smbus_write_byte(Self::ADDRESS, write_buf)?;
        Ok(())
    }
}
impl ReadRegister for InterruptPinConfig {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self {
            int_level: (read_buf >> 7) != 0,
            int_open: ((read_buf >> 6) & 1) != 0,
            latch_int_en: ((read_buf >> 5) & 1) != 0,
            int_rd_clear: ((read_buf >> 4) & 1) != 0,
            fsync_int_level: ((read_buf >> 3) & 1) != 0,
            fsync_int_mode_en: ((read_buf >> 2) & 1) != 0,
        })
    }
}

#[allow(clippy::struct_excessive_bools)]
struct InterruptEnable {
    // 1 – Enable WoM interrupt on accelerometer.
    // 0 – Disable WoM interrupt on accelerometer.
    pub wom_int_en: bool,
    // 1 – Enables a FIFO buffer overflow to generate an interrupt.
    // 0 – Function is disabled.
    pub fifo_oflow_en: bool,
    // Gyroscope Drive System Ready interrupt enable
    pub gdrive_int_en: bool,
    // Data ready interrupt enable
    pub data_rdy_int_en: bool,
}
impl InterruptEnable {
    const ADDRESS: u8 = 0x38;
}
impl WriteRegister for InterruptEnable {
    fn write(&self, i2c: &mut rppal::i2c::I2c) -> Result<()> {
        let write_buf = (u8::from(self.wom_int_en) << 7)
            | (u8::from(self.wom_int_en) << 6)
            | (u8::from(self.wom_int_en) << 5)
            | (u8::from(self.fifo_oflow_en) << 4)
            | (u8::from(self.gdrive_int_en) << 2)
            | u8::from(self.data_rdy_int_en);
        i2c.smbus_write_byte(Self::ADDRESS, write_buf)?;
        Ok(())
    }
}
impl ReadRegister for InterruptEnable {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self {
            wom_int_en: (read_buf >> 7) != 0,
            fifo_oflow_en: ((read_buf >> 4) & 1) != 0,
            gdrive_int_en: ((read_buf >> 2) & 1) != 0,
            data_rdy_int_en: (read_buf & 1) != 0,
        })
    }
}

#[allow(clippy::struct_excessive_bools)]
struct InterruptStatus {
    // Accelerometer WoM interrupt status. Cleared on Read.
    // 111 – WoM interrupt on acceleromete.
    pub wom_int: bool,
    // This bit automatically sets to 1 when a FIFO buffer overflow has been
    // generated. The bit clears to 0 after the register has been read.
    pub fifo_oflow_int: bool,
    // Gyroscope Drive System Ready interrupt.
    pub gdrive_int: bool,
    // This bit automatically sets to 1 when a Data Ready interrupt is generated. The
    // bit clears to 0 after the register has been read.
    pub data_rdy_int: bool,
}
impl InterruptStatus {
    const ADDRESS: u8 = 0x3A;
}
impl ReadRegister for InterruptStatus {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let read_buf = i2c.smbus_read_byte(Self::ADDRESS)?;
        Ok(Self {
            wom_int: (read_buf >> 7) != 0,
            fifo_oflow_int: ((read_buf >> 4) & 1) != 0,
            gdrive_int: ((read_buf >> 2) & 1) != 0,
            data_rdy_int: (read_buf & 1) != 0,
        })
    }
}

struct AccelMeasurements {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
impl AccelMeasurements {
    const ADDRESS_XH: u8 = 0x3B;
    const ADDRESS_XL: u8 = 0x3C;
    const ADDRESS_YH: u8 = 0x3D;
    const ADDRESS_YL: u8 = 0x3E;
    const ADDRESS_ZH: u8 = 0x3F;
    const ADDRESS_ZL: u8 = 0x40;
}
impl ReadRegister for AccelMeasurements {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let x_high = i2c.smbus_read_byte(Self::ADDRESS_XH)?;
        let x_low = i2c.smbus_read_byte(Self::ADDRESS_XL)?;
        let y_high = i2c.smbus_read_byte(Self::ADDRESS_YH)?;
        let y_low = i2c.smbus_read_byte(Self::ADDRESS_YL)?;
        let z_high = i2c.smbus_read_byte(Self::ADDRESS_ZH)?;
        let z_low = i2c.smbus_read_byte(Self::ADDRESS_ZL)?;
        Ok(Self {
            x: (i16::from(x_high) << 8) | i16::from(x_low),
            y: (i16::from(y_high) << 8) | i16::from(y_low),
            z: (i16::from(z_high) << 8) | i16::from(z_low),
        })
    }
}

struct TemperatureMeasurements {
    // TEMP_degC = ((TEMP_OUT – RoomTemp_Offset)/Temp_Sensitivity) + 25degC
    pub temp_out: i16,
}
impl TemperatureMeasurements {
    const ADDRESS_H: u8 = 0x41;
    const ADDRESS_L: u8 = 0x42;
}
impl ReadRegister for TemperatureMeasurements {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let temp_high = i2c.smbus_read_byte(Self::ADDRESS_H)?;
        let temp_low = i2c.smbus_read_byte(Self::ADDRESS_L)?;
        Ok(Self {
            temp_out: (i16::from(temp_high) << 8) | i16::from(temp_low),
        })
    }
}

struct GyroscopeMeasurements {
    // GYRO_XOUT = Gyro_Sensitivity * X_angular_rate
    // Nominal      FS_SEL = 0
    // Conditions   Gyro_Sensitivity = 131 LSB/(º/s)
    x: i16,
    // GYRO_YOUT = Gyro_Sensitivity * Y_angular_rate
    // Nominal      FS_SEL = 0
    // Conditions   Gyro_Sensitivity = 131 LSB/(º/s)
    y: i16,
    // GYRO_ZOUT = Gyro_Sensitivity * Z_angular_rate
    // Nominal      FS_SEL = 0
    // Conditions   Gyro_Sensitivity = 131 LSB/(º/s)
    z: i16,
}
impl GyroscopeMeasurements {
    const ADDRESS_XH: u8 = 0x43;
    const ADDRESS_XL: u8 = 0x44;
    const ADDRESS_YH: u8 = 0x45;
    const ADDRESS_YL: u8 = 0x46;
    const ADDRESS_ZH: u8 = 0x47;
    const ADDRESS_ZL: u8 = 0x48;
}
impl ReadRegister for GyroscopeMeasurements {
    fn new(i2c: &mut rppal::i2c::I2c) -> Result<Self>
    where
        Self: Sized,
    {
        let x_high = i2c.smbus_read_byte(Self::ADDRESS_XH)?;
        let x_low = i2c.smbus_read_byte(Self::ADDRESS_XL)?;
        let y_high = i2c.smbus_read_byte(Self::ADDRESS_YH)?;
        let y_low = i2c.smbus_read_byte(Self::ADDRESS_YL)?;
        let z_high = i2c.smbus_read_byte(Self::ADDRESS_ZH)?;
        let z_low = i2c.smbus_read_byte(Self::ADDRESS_ZL)?;
        Ok(Self {
            x: (i16::from(x_high) << 8) | i16::from(x_low),
            y: (i16::from(y_high) << 8) | i16::from(y_low),
            z: (i16::from(z_high) << 8) | i16::from(z_low),
        })
    }
}
