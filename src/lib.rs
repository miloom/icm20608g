#[cfg(feature = "visualize")]
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use rppal::i2c::Error;
use rppal::i2c::I2c;

pub mod structs;

use structs::Vec3;

pub type Result<T> = std::result::Result<T, Error>;

/// Print the device ID
///
/// # Panics
/// Will panic if unable to set slave address or communicate with the device
pub fn who_am_i(i2c: &mut I2c) {
    i2c.set_slave_address(0x68).unwrap();
    let mut device_id = [0; 1];
    i2c.write_read(&[0x75], &mut device_id).unwrap();
    println!("Read: {device_id:?}");
}

/// Get accelerometer data
///
/// # Errors
/// Will error if unable to set slave address or communicate with the device
#[allow(clippy::similar_names)]
pub fn accel_data(i2c: &mut I2c) -> Result<Vec3<u16>> {
    i2c.set_slave_address(0x68)?;
    let mut accel_x_h = [0, 1];
    i2c.write_read(&[0x3B], &mut accel_x_h)?;
    let mut accel_x_l = [0, 1];
    i2c.write_read(&[0x3C], &mut accel_x_l)?;
    let accel_x = (u16::from(accel_x_h[0]) << 8) | u16::from(accel_x_l[0]);

    let mut accel_y_h = [0, 1];
    i2c.write_read(&[0x3D], &mut accel_y_h)?;
    let mut accel_y_l = [0, 1];
    i2c.write_read(&[0x3E], &mut accel_y_l)?;
    let accel_y = (u16::from(accel_y_h[0]) << 8) | u16::from(accel_y_l[0]);

    let mut accel_z_h = [0, 1];
    i2c.write_read(&[0x3F], &mut accel_z_h)?;
    let mut accel_z_l = [0, 1];
    i2c.write_read(&[0x40], &mut accel_z_l)?;
    let accel_z = (u16::from(accel_z_h[0]) << 8) | u16::from(accel_z_l[0]);

    Ok(Vec3 {
        x: accel_x,
        y: accel_y,
        z: accel_z,
    })
}
