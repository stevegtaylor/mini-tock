// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

#![forbid(unsafe_code)]
#![no_std]

pub mod test;
pub mod tutorials;

#[macro_use]

pub mod apds9960;
pub mod app_flash_driver;
pub mod app_loader;
pub mod at24c_eeprom;
pub mod ble_advertising_driver;
pub mod bme280;
pub mod bmm150;
pub mod bmp280;
pub mod ccs811;
pub mod chirp_i2c_moisture;
pub mod cycle_count;
pub mod date_time;
pub mod debug_process_restart;
pub mod dfrobot_rainfall_sensor;
pub mod distance;
pub mod eui64;
pub mod fm25cl;
pub mod fxos8700cq;
pub mod gpio_async;
pub mod hc_sr04;
pub mod hmac;
pub mod hmac_sha256;
pub mod hs3003;
pub mod hts221;
pub mod humidity;
pub mod isl29035;
pub mod isolated_nonvolatile_storage_driver;
pub mod l3gd20;
pub mod led_matrix;
pub mod log;
pub mod nonvolatile_storage_driver;
pub mod nonvolatile_to_pages;
pub mod nrf51822_serialization;
pub mod panic_button;
pub mod pca9544a;
pub mod pressure;
pub mod process_info_driver;
pub mod proximity;
pub mod rainfall;
pub mod read_only_state;
pub mod rf233_const;
pub mod sdcard;
pub mod sdi12_ents;
pub mod symmetric_encryption;
pub mod temperature;
pub mod tickv;
pub mod tsl2561;
pub mod usb;
pub mod usb_hid_driver;
pub mod virtualizers;
