use clap::{Args, ValueEnum};
use ecu_diag::api::UdsServiceProvider;
use ecu_diag::uds::routine_control::SimulateInputOption;
use ecu_diag::uds::routine_control::{RoutineControlSubfcn, RoutineId};
use ecu_diag::uds::UDSClientSession;

#[derive(Args, Clone, Debug)]
pub struct SimulateInputCmd {
    #[arg(short, long)]
    action: Action,
    #[arg(short, long)]
    input: Input,
}

#[allow(unused_must_use)]
impl SimulateInputCmd {
    pub fn run(&self, client: &mut UDSClientSession) {
        let action = match self.action {
            Action::Enable => RoutineControlSubfcn::StartRoutine,
            Action::Disable => RoutineControlSubfcn::StopRoutine,
        };
        match self.input {
            Input::RightBrakeSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::RightBrakeSwitch as u8],
            ),
            Input::LeftBrakeSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::LeftBrakeSwitch as u8],
            ),
            Input::KillSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::KillSwitch as u8],
            ),
            Input::PowerSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::PowerSwitch as u8],
            ),
            Input::PowerSwShortPress => {
                client.invoke_routine_control_service(
                    RoutineControlSubfcn::StartRoutine,
                    RoutineId::SimulateInput,
                    &[SimulateInputOption::PowerSwitch as u8],
                );

                client.invoke_routine_control_service(
                    RoutineControlSubfcn::StopRoutine,
                    RoutineId::SimulateInput,
                    &[SimulateInputOption::PowerSwitch as u8],
                )
            }
            Input::StartSwShortPress => {
                client.invoke_routine_control_service(
                    RoutineControlSubfcn::StartRoutine,
                    RoutineId::SimulateInput,
                    &[SimulateInputOption::StartSwitch as u8],
                );

                client.invoke_routine_control_service(
                    RoutineControlSubfcn::StopRoutine,
                    RoutineId::SimulateInput,
                    &[SimulateInputOption::StartSwitch as u8],
                )
            }
            Input::ReverseSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::ReverseSwitch as u8],
            ),
            Input::SeatSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::SeatSwitch as u8],
            ),
            Input::SideStandSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::SideStandSwitch as u8],
            ),
            Input::TripSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::TripSwitch as u8],
            ),
            Input::RideModeSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::RideModeSwitch as u8],
            ),
            Input::HazardSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::HazardSwitch as u8],
            ),
            Input::HornSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::HornSwitch as u8],
            ),
            Input::RightIndicatorSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::RightIndicatorSwitch as u8],
            ),
            Input::LeftIndicatorSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::LeftIndicatorSwitch as u8],
            ),
            Input::PassingSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::PassingSwitch as u8],
            ),
            Input::HighbeamSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::HighBeamSwitch as u8],
            ),
            Input::StartSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::StartSwitch as u8],
            ),
            Input::BackSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::BackSwitch as u8],
            ),
            Input::SelectSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::SelectSwitch as u8],
            ),
            Input::DownSw => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::DownSwitch as u8],
            ),
            Input::KeyfobShortPress => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::KeyfobShortPress as u8],
            ),
            Input::KeyfobLongPress => client.invoke_routine_control_service(
                action,
                RoutineId::SimulateInput,
                &[SimulateInputOption::KeyfobLongPress as u8],
            ),
        };
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Input {
    /// Right brake switch
    RightBrakeSw,
    /// Left brake swtich
    LeftBrakeSw,
    /// Kill switch
    KillSw,
    /// Power switch
    PowerSw,
    /// Power switch short press
    PowerSwShortPress,
    /// Reverse switch
    ReverseSw,
    /// Seat switch
    SeatSw,
    /// Side stand switch
    SideStandSw,
    /// Trip switch
    TripSw,
    /// Ride mode switch
    RideModeSw,
    /// Hazard switch
    HazardSw,
    /// Horn switch
    HornSw,
    /// Right indicator switch
    RightIndicatorSw,
    /// Left indicator switch
    LeftIndicatorSw,
    /// Passing switch
    PassingSw,
    /// High beam switch
    HighbeamSw,
    /// Start switch
    StartSw,
    /// Start switch short press
    StartSwShortPress,
    /// Back switch
    BackSw,
    /// Select switch
    SelectSw,
    /// Down switch
    DownSw,
    /// Keyfob short press
    KeyfobShortPress,
    /// Keyfob long press
    KeyfobLongPress,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Action {
    /// Enable
    Enable,
    /// Disable
    Disable,
}
