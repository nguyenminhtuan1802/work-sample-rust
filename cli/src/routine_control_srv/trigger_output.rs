use clap::{Args, ValueEnum};
use ecu_diag::api::UdsServiceProvider;
use ecu_diag::uds::routine_control::TriggerOutputOption;
use ecu_diag::uds::routine_control::{RoutineControlSubfcn, RoutineId};
use ecu_diag::uds::UDSClientSession;

#[derive(Args, Clone, Debug)]
pub struct TriggerOutputCmd {
    #[arg(short, long)]
    action: Action,
    #[arg(short, long)]
    output: Output,
}

#[allow(unused_must_use)]
impl TriggerOutputCmd {
    pub fn run(&self, client: &mut UDSClientSession) {
        let action = match self.action {
            Action::Enable => RoutineControlSubfcn::StartRoutine,
            Action::Disable => RoutineControlSubfcn::StopRoutine,
        };
        match self.output {
            Output::HssRearRightIndicator => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssRearRightIndicator as u8],
            ),
            Output::HssRearLeftIndicator => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssRearLeftIndicator as u8],
            ),
            Output::HssBrakeLight => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssBrakeLight as u8],
            ),
            Output::HssHorn => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssHorn as u8],
            ),
            Output::HssHighBeam => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssHighBeam as u8],
            ),
            Output::HssLowBeam => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssLowBeam as u8],
            ),
            Output::HssLicensePlate => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssLicensePlate as u8],
            ),
            Output::HssFrontLeftIndicator => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssFrontLeftIndicator as u8],
            ),
            Output::HssFrontRightIndicator => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssFrontRightIndicator as u8],
            ),
            Output::HssTailLight => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssTailLight as u8],
            ),
            Output::HssSeatLock => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssSeatLock as u8],
            ),
            Output::HssBmsEnable => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssBmsEnable as u8],
            ),
            Output::HssMotorEnable => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssMotorEnable as u8],
            ),
            Output::HssSteerLock => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssSteerLock as u8],
            ),
            Output::HssDrl => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssDrl as u8],
            ),
            Output::HssTpms => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssTpms as u8],
            ),
            Output::HssSideStandPower => client.invoke_routine_control_service(
                action,
                RoutineId::TriggerOutput,
                &[TriggerOutputOption::HssSideStandPower as u8],
            ),
        };
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Output {
    /// HSS Rear Right Indicator
    HssRearRightIndicator,
    /// HSS Rear Left Indicator
    HssRearLeftIndicator,
    /// HSS Brake Light
    HssBrakeLight,
    /// HSS Horn
    HssHorn,
    /// HSS High Beam
    HssHighBeam,
    /// HSS Low Beam
    HssLowBeam,
    /// HSS License Plate
    HssLicensePlate,
    /// HSS Front Left Indicator
    HssFrontLeftIndicator,
    /// HSS Front Right Indicator
    HssFrontRightIndicator,
    /// HSS Tail Light
    HssTailLight,
    /// HSS Seat Lock
    HssSeatLock,
    /// HSS BMS Enable
    HssBmsEnable,
    /// HSS Motor Enable
    HssMotorEnable,
    /// HSS Steer Lock
    HssSteerLock,
    /// HSS DRL?
    HssDrl,
    /// HSS Tire Pressure Monitoring System (TPMS)
    HssTpms,
    /// HSS Side Stand Power
    HssSideStandPower,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Action {
    /// Enable
    Enable,
    /// Disable
    Disable,
}
