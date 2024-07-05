use super::{Action, AppUi};
use ecu_diag::api::UdsMonitorViewResponse;
use ecu_diag::uds::routine_control::TriggerOutputOption;
use slint::ComponentHandle;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::sync::Mutex;

use crate::{ServiceRequest, ServiceResponse};
use ecu_diag::api::UdsServiceProvider;
use ecu_diag::api::UdsServiceResponse;
use ecu_diag::uds::diagnostic_session_control::UdsSessionType;
use ecu_diag::uds::ecu_reset::ResetType;
use ecu_diag::uds::read_data_by_id::DataId;
use ecu_diag::uds::routine_control::Domain;
use ecu_diag::uds::routine_control::SimulateInputOption;
use ecu_diag::uds::routine_control::{RoutineControlSubfcn, RoutineId};
use ecu_diag::uds::UDSClientSession;

use std::str::FromStr;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use tokio::task::JoinHandle;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[allow(dead_code)]
pub enum UdsMessage {
    Quit,
    Action { action: Action },
}

#[allow(dead_code)]
pub struct UdsWorker {
    pub channel: UnboundedSender<UdsMessage>,
    pub receive_channel: UnboundedReceiver<UdsServiceResponse>,
    pub uds_client: Arc<Mutex<UDSClientSession>>,
    worker_thread: JoinHandle<()>,

    pub channel_monitor_view: UnboundedSender<UdsMessage>,
    pub receive_channel_monitor_view: UnboundedReceiver<UdsMonitorViewResponse>,
    monitor_view_thread: JoinHandle<()>,
}

impl UdsWorker {
    //pub async fn new(app_ui: &AppUi) _> Self {

    pub async fn new(
        app_ui: &AppUi,
        rx_res: UnboundedReceiver<ServiceResponse>,
        tx_req: UnboundedSender<ServiceRequest>,
        stop_rx_uds_worker: oneshot::Receiver<()>,
        stop_rx_monitor: oneshot::Receiver<()>,
    ) -> Self {
        let (channel, r) = tokio::sync::mpsc::unbounded_channel();
        let (s, receive_channel) = tokio::sync::mpsc::unbounded_channel();

        let uds_client = UDSClientSession::new_uds_client(tx_req, rx_res).await;
        app_ui.set_diagnostics_session_state(uds_client.current_diag_mode.mode.into());

        // Create a new instance of UDSClientSession
        let uds_client_arc = Arc::new(Mutex::new(uds_client));

        // Capture a clone of Arc<Mutex<UDSClientSession>> in the closure
        let uds_client_clone = Arc::clone(&uds_client_arc);

        let worker_thread = tokio::spawn({
            let handle_weak = app_ui.as_weak();
            async move {
                spawn_worker_thread(r, s, handle_weak, uds_client_clone, stop_rx_uds_worker).await;
            }
        });

        let (channel_monitor_view, r_monitor_view) = tokio::sync::mpsc::unbounded_channel();
        let (s_monitor_view, receive_channel_monitor_view) = tokio::sync::mpsc::unbounded_channel();
        let uds_client_clone2 = Arc::clone(&uds_client_arc);

        let monitor_view_thread = tokio::spawn({
            let handle_weak = app_ui.as_weak();
            async move {
                spawn_monitor_view_thread(
                    r_monitor_view,
                    s_monitor_view,
                    handle_weak,
                    uds_client_clone2,
                    stop_rx_monitor,
                )
                .await;
            }
        });

        Self {
            channel,
            receive_channel,
            uds_client: uds_client_arc,
            worker_thread,
            channel_monitor_view,
            receive_channel_monitor_view,
            monitor_view_thread,
        }
    }

    #[allow(dead_code)]
    pub fn join(self) {
        let _ = self.channel.send(UdsMessage::Quit);
        self.worker_thread.abort();
    }
}

pub async fn spawn_monitor_view_thread(
    mut r: UnboundedReceiver<UdsMessage>,
    s: UnboundedSender<UdsMonitorViewResponse>,
    _handle: slint::Weak<AppUi>,
    client: Arc<Mutex<UDSClientSession>>,
    mut stop_rx_monitor: oneshot::Receiver<()>,
) {
    tokio::spawn(async move {
        log::debug!("Running monitor view thread");
        println!("Running monitor view thread");
        loop {
            tokio::select! {
                res = r.recv() => {
                    match res {
                        Some(msg) => {
                            match msg {
                                UdsMessage::Quit => {
                                    println!("Quit!");
                                    return;
                                }
                                UdsMessage::Action { action } => {
                                    println!(
                                        "Perform action from monitor view thread: {}, {}, {}, {}",
                                        action.service, action.option1, action.option2, action.option3
                                    );

                                    let mut guard = client.lock().await;

                                    if action.service == "Read Monitor View" {
                                        let res = guard
                                            .invoke_read_data_by_id_service_return_struct(DataId::Dashboard);

                                        s.send(res).unwrap();
                                    }
                                }
                            }
                        }
                        None => {
                            return;
                        },
                    }
                }
                _ = &mut stop_rx_monitor => {
                    log::debug!("Monitor task received stop signal");
                    println!("Monitor task received stop signal.");
                    return; // Exit the task on stop signal
                },
            }
        }
    })
    .await
    .expect("The spawned task has panicked or been cancelled");
}

pub async fn spawn_worker_thread(
    mut r: UnboundedReceiver<UdsMessage>,
    s: UnboundedSender<UdsServiceResponse>,
    _handle: slint::Weak<AppUi>,
    client: Arc<Mutex<UDSClientSession>>,
    mut stop_rx_uds_worker: oneshot::Receiver<()>,
) {
    tokio::spawn(async move {
        log::debug!("Running worker thread");
        println!("Running worker thread");
        loop {
            tokio::select! {
                res = r.recv() => {
                    match res {
                        Some(msg) => {
                            match msg {
                                UdsMessage::Quit => {
                                    println!("Quit!");
                                    return;
                                }
                                UdsMessage::Action { action } => {
                                    let mut guard = client.lock().await;

                                    println!(
                                        "Perform action: {}, {}, {}, {}",
                                        action.service, action.option1, action.option2, action.option3
                                    );

                                    if action.service == "Read" {
                                        if action.option1 == "Bike State and Bike Lock" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::BikeState);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Switch Gear" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::SwitchGear);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Error Code" {
                                            let res =
                                                guard.invoke_read_data_by_id_service(DataId::ComponentError);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Inertial measurement unit (IMU)" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::ImuRaw);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Keyfob Data" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::KeyfobState);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Vehicle Metrics 1" {
                                            let res = guard
                                                .invoke_read_data_by_id_service(DataId::PerformanceVehicle1);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Vehicle Metrics 2" {
                                            let res = guard
                                                .invoke_read_data_by_id_service(DataId::PerformanceVehicle2);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Charge Metrics" {
                                            let res =
                                                guard.invoke_read_data_by_id_service(DataId::PerformanceCharge);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Firmware Version" {
                                            let res =
                                                guard.invoke_read_data_by_id_service(DataId::FirmwareVersion);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Analog-Digital Converter Voltage" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::AdcVoltage);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Battery management system (BMS) Data1" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::Bms1);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Battery management system (BMS) Data2" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::Bms2);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Battery management system (BMS) Data3" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::Bms3);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Temmperature Sensors" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::TempSensors);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Diag State" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::DiagState);

                                            s.send(res).unwrap();
                                        } else if action.option1 == "Dashboard" {
                                            let res = guard.invoke_read_data_by_id_service(DataId::Dashboard);

                                            s.send(res).unwrap();
                                        }
                                    } else if action.service == "Routine" {
                                        if action.option1 == "Simulate VCU Input" {
                                            if action.option2 == "Right Brake Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::RightBrakeSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::RightBrakeSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Left Brake Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::LeftBrakeSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::LeftBrakeSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Kill Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::KillSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::KillSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Power Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::PowerSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::PowerSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Reverse Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::ReverseSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::ReverseSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Side Stand Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::SideStandSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::SideStandSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Ride Mode Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::RideModeSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::RideModeSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Hazard Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::HazardSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::HazardSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Horn Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::HornSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::HornSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Right Indicator Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::RightIndicatorSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::RightIndicatorSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Left Indicator Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::LeftIndicatorSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::LeftIndicatorSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "High Beam Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::HighBeamSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::HighBeamSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Start Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::StartSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::StartSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Seat Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::SeatSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::SeatSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Trip Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::TripSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::TripSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Down Switch" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::DownSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::DownSwitch as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Keyfob Short Press" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::KeyfobShortPress as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::KeyfobShortPress as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Keyfob Long Press" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::KeyfobLongPress as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::SimulateInput,
                                                            &[SimulateInputOption::KeyfobLongPress as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            }
                                        } else if action.option1 == "Trigger VCU Output" {
                                            if action.option2 == "Rear Right Indicator" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssRearRightIndicator as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssRearRightIndicator as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Rear Left Indicator" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssRearLeftIndicator as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssRearLeftIndicator as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Brake Light" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssBrakeLight as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssBrakeLight as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Horn" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssHorn as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssHorn as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "High Beam" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssHighBeam as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssHighBeam as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Low Beam" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssLowBeam as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssLowBeam as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "License Plate" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssLicensePlate as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssLicensePlate as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Front Left Indicator" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssFrontLeftIndicator as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssFrontLeftIndicator as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Front Right Indicator" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssFrontRightIndicator
                                                                as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssFrontRightIndicator
                                                                as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Tail Light" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssTailLight as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssTailLight as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Seat Lock" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssSeatLock as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssSeatLock as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "BMS Enable" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssBmsEnable as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssBmsEnable as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Motor Enable" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssMotorEnable as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssMotorEnable as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Steer Lock" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssSteerLock as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssSteerLock as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "DRL" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssDrl as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssDrl as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Tire Pressure Monitoring System (TPMS)"
                                            {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssTpms as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssTpms as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            } else if action.option2 == "Side Stand Power" {
                                                if action.option3 == "Enable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StartRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssSideStandPower as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                } else if action.option3 == "Disable" {
                                                    let res = guard
                                                        .invoke_routine_control_service(
                                                            RoutineControlSubfcn::StopRoutine,
                                                            RoutineId::TriggerOutput,
                                                            &[TriggerOutputOption::HssSideStandPower as u8],
                                                        )
                                                        .await;
                                                    s.send(res).unwrap();
                                                }
                                            }
                                        } else if action.option1 == "Open Debug Screen" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::OpenDebugScreen,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Close Debug Screen" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::CloseDebugScreen,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Toggle Off BMS Voltage" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::ToggleOffBMSVoltage,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Toggle On BMS Voltage" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::ToggleOnBMSVoltage,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Bike Force Unlock" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::BikeForceUnlock,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Bike Force Lock" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::BikeForceLock,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        }
                                    } else if action.service == "Connectivity" {
                                        if action.option1 == "Wifi Scan" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::WifiScan,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Wifi Scan Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::WifiScan,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Wifi Check IP" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::WifiCheckIp,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Wifi Check IP Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::WifiCheckIp,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Wifi Restart App" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::WifiRestartApp,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Wifi Restart App Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::WifiRestartApp,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "GPS Check Log" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::GpsCheckLog,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "GPS Check Log Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::GpsCheckLog,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Check IP" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::LteCheckIp,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Check IP Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::LteCheckIp,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Check Ping" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::LteCheckPing,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Check Ping Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::LteCheckPing,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Check Enable Signal" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::LteCheckEnableSignal,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Check Enable Signal Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::LteCheckEnableSignal,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Get Modem Info" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::LteGetModemInfo,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Get Modem Info Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::LteGetModemInfo,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Get Signal Strength" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::LteGetSignalStrength,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Get Signal Strength Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::LteGetSignalStrength,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Enable LTE/GPS" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::EnableImxLte,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LTE Disable LTE/GPS" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::DisableImxLte,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "BLE Restart App" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::BleRestartApp,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "BLE Restart App Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::BleRestartApp,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "BLE Check Pair" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::BleCheckPair,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "BLE Check Pair Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::BleCheckPair,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "IMX Check Service Status" {
                                            let res = guard
                                                .invoke_routine_control_service(
                                                    RoutineControlSubfcn::StartRoutine,
                                                    RoutineId::ImxCheckServiceStatus,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        } else if action.option1 == "IMX Check Service Status Get Result" {
                                            let res = guard
                                                .invoke_routine_control_service_get_result(
                                                    RoutineControlSubfcn::RequestRoutineResults,
                                                    RoutineId::ImxCheckServiceStatus,
                                                    &[],
                                                )
                                                .await;
                                            s.send(res).unwrap();
                                        }
                                    } else if action.service == "Reset" {
                                        if action.option1 == "Realtime 118" {
                                            let res = guard.invoke_reset_ecu_service(ResetType::RealtimeReset);
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Telematic 148" {
                                            let res = guard.invoke_reset_ecu_service(ResetType::TelematicReset);
                                            s.send(res).unwrap();
                                        } else if action.option1 == "IMX" {
                                            let res = guard.invoke_reset_ecu_service(ResetType::ImxReset);
                                            s.send(res).unwrap();
                                        } else if action.option1 == "ESP32 WIFI" {
                                            let res = guard.invoke_reset_ecu_service(ResetType::Esp32WifiReset);
                                            s.send(res).unwrap();
                                        } else if action.option1 == "ESP32 BLE" {
                                            let res = guard.invoke_reset_ecu_service(ResetType::Esp32BleReset);
                                            s.send(res).unwrap();
                                        } else if action.option1 == "QUECTEL" {
                                            let res = guard.invoke_reset_ecu_service(ResetType::QuectelReset);
                                            s.send(res).unwrap();
                                        } else if action.option1 == "LIZARD" {
                                            let res = guard.invoke_reset_ecu_service(ResetType::LizardReset);
                                            s.send(res).unwrap();
                                        } else if action.option1 == "CENDRIC" {
                                            let res = guard.invoke_reset_ecu_service(ResetType::CendricReset);
                                            s.send(res).unwrap();
                                        }
                                    } else if action.service == "Set Mode" {
                                        if action.option1 == "User Mode" {
                                            let res = guard.invoke_set_session_mode(UdsSessionType::Default);
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Debug Mode" {
                                            let res =
                                                guard.invoke_set_session_mode(UdsSessionType::Programming);
                                            s.send(res).unwrap();
                                        } else if action.option1 == "Stream Mode" {
                                            let res = guard.invoke_set_session_mode(UdsSessionType::StreamMode);
                                            s.send(res).unwrap();
                                        }
                                    }
                                }
                            }

                        }
                        None => {
                            return;
                        },
                    }
                }
                _ = &mut stop_rx_uds_worker => {
                    log::debug!("UDS worker received stop signal");
                    println!("UDS worker received stop signal.");
                    return; // Exit the task on stop signal
                },
            }
        }
    })
    .await
    .expect("The spawned task has panicked or been cancelled");
}

pub async fn start_tcp_listener(
    reader: Arc<Mutex<tokio::io::ReadHalf<tokio::net::TcpStream>>>,
    sender: UnboundedSender<ServiceResponse>,
    _handle: slint::Weak<AppUi>,
    mut stop_rx_tcp: oneshot::Receiver<()>,
) {
    tokio::spawn(async move {
        loop {
            let mut lock = reader.lock().await;
            let _handle_clone = _handle.clone();

            let mut buf = [0; 4096];

            tokio::select! {
                res = lock.read(&mut buf) => {

                    match res {
                        Ok(0) => {
                            log::debug!("Client disconnected");
                            println!("Client disconnected.");

                            let _ = slint::invoke_from_event_loop(move || {
                                _handle_clone.unwrap().set_tcp_connection_state(2);
                                _handle_clone.unwrap().set_tcp_connection_state1(2);
                            });
                            return;
                        }
                        Ok(n) => {
                            match serde_json::from_slice::<serde_json::Value>(&buf[..n]) {
                                Ok(msg) => {
                                    //println!("Received message: {}", msg);

                                    let domain = match msg.get("domain") {
                                        Some(v) => v.clone().as_str().unwrap_or_default().to_owned(),
                                        None => String::default(),
                                    };

                                    let command = match msg.get("command") {
                                        Some(v) => v.clone().as_u64().unwrap_or_default() as u8,
                                        None => 0,
                                    };

                                    let response = match msg.get("response") {
                                        Some(v) => v.clone().as_str().unwrap_or_default().to_owned(),
                                        None => String::default(),
                                    };

                                    let mut service_response = ServiceResponse {
                                        domain: Domain::from_str(&domain).unwrap(),
                                        command,
                                        response,
                                    };
                                    service_response.parse_response();

                                    if sender.send(service_response).is_err() {
                                        println!("receiver dropped");
                                        return;
                                    }
                                }
                                Err(e) => println!("Failed to parse received message: {:?}", e),
                            }
                        }
                        Err(_) => {
                            println!("Receiving task encountered an error.");

                            let _ = slint::invoke_from_event_loop(move || {
                                _handle_clone.unwrap().set_tcp_connection_state(2);
                                _handle_clone.unwrap().set_tcp_connection_state1(2);
                            });
                            return;
                        }
                    }

                },
                _ = &mut stop_rx_tcp => {
                    log::debug!("TCP task received stop signal");
                    println!("TCP task received stop signal.");
                    return; // Exit the task on stop signal
                },
            }
        }
    })
    .await
    .expect("The spawned task has panicked or been cancelled");
}

pub async fn start_service_listener(
    writer: Arc<Mutex<tokio::io::WriteHalf<tokio::net::TcpStream>>>,
    mut receiver: UnboundedReceiver<ServiceRequest>,
    _handle: slint::Weak<AppUi>,
    mut stop_rx_service: oneshot::Receiver<()>,
) {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                res = receiver.recv() => {
                    match res {
                        Some(req) => {
                            // Convert data to JSON
                            let json_str =
                                serde_json::to_string(&req).expect("Failed to serialize data to JSON");

                            let result = writer.lock().await.write_all(json_str.as_bytes()).await;
                            match result {
                                Ok(()) => {
                                    // Write operation successful
                                    println!("Write operation successful");
                                }
                                Err(err) => {
                                    // Error occurred during write operation
                                    println!("Error occurred during write operation: {}", err);
                                    let _handle_clone = _handle.clone();
                                    let _ = slint::invoke_from_event_loop(move || {
                                        _handle_clone.unwrap().set_tcp_connection_state(2);
                                        _handle_clone.unwrap().set_tcp_connection_state1(2);
                                    });
                                    return;
                                }
                            }
                        }
                        None => {
                            return;
                        },
                    }
                },
                _ = &mut stop_rx_service => {
                    log::debug!("Service task received stop signal");
                    println!("Service task received stop signal.");
                    return; // Exit the task on stop signal
                },
            }
        }
    })
    .await
    .expect("The spawned task has panicked or been cancelled");
}
