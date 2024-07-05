use clap::{Args, Subcommand};
use ecu_diag::uds::UDSClientSession;

mod ble;
mod cendric;
mod imx;
mod lizard;
mod lte;
mod rt;
mod soft_reset_ecu;
mod tm;
mod wifi;

pub(crate) use ble::ResetBleCmd;
pub(crate) use cendric::ResetCendricCmd;
pub(crate) use imx::ResetImxCmd;
pub(crate) use lizard::ResetLizardCmd;
pub(crate) use lte::ResetLteCmd;
pub(crate) use rt::ResetRtCmd;
pub(crate) use soft_reset_ecu::SoftResetEcuCmd;
pub(crate) use tm::ResetTmCmd;
pub(crate) use wifi::ResetWifiCmd;

#[derive(Args, Clone, Debug)]
pub struct ResetEcuServiceCmd {
    #[command(subcommand)]
    pub subcommand: ResetEcuServiceSubCmd,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ResetEcuServiceSubCmd {
    /// Soft Reset VCU
    SoftReset(SoftResetEcuCmd),
    /// Realtime 118
    Rt(ResetRtCmd),
    /// Telematic 148
    Tm(ResetTmCmd),
    /// IMX
    Imx(ResetImxCmd),
    /// ESP32 WIFI
    Wifi(ResetWifiCmd),
    /// ESP32 BLE
    Ble(ResetBleCmd),
    /// QUECTEL
    Lte(ResetLteCmd),
    /// LIZARD
    Lizard(ResetLizardCmd),
    /// CENDRIC
    Cendric(ResetCendricCmd),
}

#[allow(dead_code)]
impl ResetEcuServiceCmd {
    pub fn run(self, client: &mut UDSClientSession) {
        match self.subcommand {
            ResetEcuServiceSubCmd::SoftReset(c) => c.run(client),
            ResetEcuServiceSubCmd::Rt(c) => c.run(client),
            ResetEcuServiceSubCmd::Tm(c) => c.run(client),
            ResetEcuServiceSubCmd::Imx(c) => c.run(client),
            ResetEcuServiceSubCmd::Wifi(c) => c.run(client),
            ResetEcuServiceSubCmd::Ble(c) => c.run(client),
            ResetEcuServiceSubCmd::Lte(c) => c.run(client),
            ResetEcuServiceSubCmd::Lizard(c) => c.run(client),
            ResetEcuServiceSubCmd::Cendric(c) => c.run(client),
        }
    }
}
