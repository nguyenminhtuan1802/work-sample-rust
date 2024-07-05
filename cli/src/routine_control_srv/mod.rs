use clap::{Args, Subcommand};
use ecu_diag::uds::UDSClientSession;

mod disable_imx_hmi;
mod disable_imx_lte;
mod enable_imx_hmi;
mod enable_imx_lte;
mod simulate_input;
mod switch_usb_otg_usb_host;
mod trigger_output;

#[allow(unused_imports)]
pub(crate) use disable_imx_hmi::DisableImxHmiCmd;
#[allow(unused_imports)]
pub(crate) use disable_imx_lte::DisableImxLteCmd;
#[allow(unused_imports)]
pub(crate) use enable_imx_hmi::EnableImxHmiCmd;
#[allow(unused_imports)]
pub(crate) use enable_imx_lte::EnableImxLteCmd;
pub(crate) use simulate_input::SimulateInputCmd;
#[allow(unused_imports)]
pub(crate) use switch_usb_otg_usb_host::SwitchUsbOtgUsbHostCmd;
pub(crate) use trigger_output::TriggerOutputCmd;

#[derive(Args, Clone, Debug)]
pub struct RoutineControlServiceCmd {
    #[command(subcommand)]
    pub subcommand: RoutineControlServiceSubCmd,
}

// #[derive(Subcommand, Clone, Debug)]
// pub enum RoutineControlServiceSubCmd {
//     /// Enable IMX: LTE
//     EnableImxLte(EnableImxLteCmd),
//     /// Disable IMX: LTE
//     DisableImxLte(DisableImxLteCmd),
//     /// Enable IMX: HMI APP
//     EnableImxHmi(EnableImxHmiCmd),
//     /// Disable IMX: HMI APP
//     DisableImxHmi(DisableImxHmiCmd),
//     /// Simulate VCU Input
//     SimulateInput(SimulateInputCmd),
//     /// Trigger VCU Output
//     TriggerOutput(TriggerOutputCmd),
//     /// Switch USB OTG and USB Host
//     SwitchUsbOtgUsbHost(SwitchUsbOtgUsbHostCmd),
// }

#[derive(Subcommand, Clone, Debug)]
pub enum RoutineControlServiceSubCmd {
    /// Simulate VCU Input
    SimulateInput(SimulateInputCmd),
    /// Trigger VCU Output
    TriggerOutput(TriggerOutputCmd),
}

#[allow(unused_must_use)]
#[allow(dead_code)]
impl RoutineControlServiceCmd {
    pub fn run(self, client: &mut UDSClientSession) {
        match self.subcommand {
            // RoutineControlServiceSubCmd::EnableImxLte(c) => c.run(client),
            // RoutineControlServiceSubCmd::DisableImxLte(c) => c.run(client),
            // RoutineControlServiceSubCmd::EnableImxHmi(c) => c.run(client),
            // RoutineControlServiceSubCmd::DisableImxHmi(c) => c.run(client),
            RoutineControlServiceSubCmd::SimulateInput(c) => c.run(client),
            RoutineControlServiceSubCmd::TriggerOutput(c) => c.run(client),
            // RoutineControlServiceSubCmd::SwitchUsbOtgUsbHost(c) => c.run(client),
        }
    }
}
