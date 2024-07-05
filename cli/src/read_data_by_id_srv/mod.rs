use clap::{Args, Subcommand};
use ecu_diag::uds::UDSClientSession;

mod adc;
mod bike_state;
mod bms;
mod error_code;
mod firmware_version;
mod imu;
mod keyfob;
mod performance;
mod switch_gear;

pub(crate) use adc::AdcCmd;
pub(crate) use bike_state::BikeStateCmd;
pub(crate) use bms::BmsCmd;
pub(crate) use error_code::ErrorCodeCmd;
pub(crate) use firmware_version::FirmwareVersionCmd;
pub(crate) use imu::ImuCmd;
pub(crate) use keyfob::KeyfobCmd;
pub(crate) use performance::PerformanceCmd;
pub(crate) use switch_gear::SwitchGearCmd;

#[derive(Args, Clone, Debug)]
pub struct ReadDataServiceCmd {
    #[command(subcommand)]
    pub subcommand: ReadDataServiceSubCmd,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ReadDataServiceSubCmd {
    /// Bike State and Bike Lock
    BikeState(BikeStateCmd),
    /// Switch Gear
    SwitchGear(SwitchGearCmd),
    /// Error Code
    ErrorCode(ErrorCodeCmd),
    /// Inertial measurement unit (IMU) Raw Data
    Imu(ImuCmd),
    /// Keyfob Data
    Keyfob(KeyfobCmd),
    /// Performance
    Performance(PerformanceCmd),
    /// Firmware Version
    FirmwareVersion(FirmwareVersionCmd),
    /// Analog-Digital Converter Voltage
    Adc(AdcCmd),
    /// Battery management system (BMS) Data
    Bms(BmsCmd),
}

#[allow(dead_code)]
impl ReadDataServiceCmd {
    pub fn run(self, client: &mut UDSClientSession) {
        match self.subcommand {
            ReadDataServiceSubCmd::BikeState(c) => c.run(client),
            ReadDataServiceSubCmd::SwitchGear(c) => c.run(client),
            ReadDataServiceSubCmd::ErrorCode(c) => c.run(client),
            ReadDataServiceSubCmd::Imu(c) => c.run(client),
            ReadDataServiceSubCmd::Keyfob(c) => c.run(client),
            ReadDataServiceSubCmd::Performance(c) => c.run(client),
            ReadDataServiceSubCmd::FirmwareVersion(c) => c.run(client),
            ReadDataServiceSubCmd::Adc(c) => c.run(client),
            ReadDataServiceSubCmd::Bms(c) => c.run(client),
        }
    }
}
