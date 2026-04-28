// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

#![no_std]

pub mod aes;
pub mod alarm;
pub mod apds9960;
pub mod app_flash_driver;
pub mod app_loader;
pub mod appid;
pub mod ble;
pub mod bme280;
pub mod bmm150;
pub mod bmp280;
pub mod button;
pub mod ccs811;
pub mod cdc;
pub mod chirp_i2c_moisture;
pub mod console;
pub mod ctap;
pub mod date_time;
pub mod debug_writer;
pub mod dfrobot_rainfall_sensor;
pub mod dynamic_binary_storage;
pub mod eui64;
pub mod flash;
pub mod fm25cl;
pub mod fxos8700;
pub mod gpio;
pub mod hmac;
pub mod hs3003;
pub mod hts221;
pub mod humidity;
pub mod i2c;

pub mod isolated_nonvolatile_storage;
pub mod keyboard_hid;
pub mod l3gd20;
pub mod led;
pub mod led_matrix;
pub mod loader;
pub mod nonvolatile_storage;
pub mod nrf51822;
pub mod panic_button;
pub mod pressure;
pub mod process_array;
pub mod process_console;
pub mod process_info_driver;
pub mod process_printer;
pub mod proximity;
pub mod rainfall;

pub mod rng;
pub mod sched;

pub mod segger_rtt;
pub mod spi;
pub mod storage_permissions;
pub mod temperature;
pub mod test;
pub mod tickv;
pub mod usb;
pub mod virtual_scheduler_timer;
