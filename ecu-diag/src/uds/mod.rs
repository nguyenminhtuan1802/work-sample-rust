//! Module for [Unified Diagnostic Services](https://en.wikipedia.org/wiki/Unified_Diagnostic_Services) - ISO-14229-1
//!

use crate::api::UdsServiceProvider;
use crate::api::{UdsMonitorViewResponse, UdsSericeResponseDetail, UdsServiceResponse};
use crate::core::channel::CanFrame;
use crate::core::dynamic_diag::{
    DiagServerAdvancedOptions, DiagServerBasicOptions, DiagSessionMode, TimeoutConfig,
};
use crate::hardware::isotp::IsoTpProtocol;

use crate::uds::diagnostic_session_control::UdsSessionType;
use crate::uds::ecu_reset::ResetType;
use crate::uds::read_data_by_id::DataId;
use crate::uds::routine_control::{RoutineControlSubfcn, RoutineId};
use automotive_diag::uds::UdsCommand;

use self::routine_control::ServiceRequest;
use self::security_access::SecurityLevelAccess;
use crate::uds::routine_control::ServiceResponse;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub mod communication_control;
pub mod control_dtc_setting;
pub mod diagnostic_session_control;
pub mod ecu_reset;
pub mod errors;
pub mod link_control;
pub mod read_data_by_id;
pub mod read_dtc_info;
pub mod routine_control;
pub mod security_access;
pub mod tester_present;

pub struct UDSClientSession {
    pub current_diag_mode: DiagSessionMode,
    pub protocol: IsoTpProtocol,
    pub basic_option: DiagServerBasicOptions,
    pub advanced_options: DiagServerAdvancedOptions,
    pub rx: UnboundedReceiver<ServiceResponse>,
    pub tx: UnboundedSender<ServiceRequest>,
}

// unsafe impl Send for UDSClientSession {}
// unsafe impl Sync for UDSClientSession {}

impl UDSClientSession {
    async fn async_default(
        tx: UnboundedSender<ServiceRequest>,
        rx: UnboundedReceiver<ServiceResponse>,
    ) -> Self {
        // If you're using PowerShell, you can set environment variables using the $env: prefix. For example:
        // $env:RUST_LOG="debug"
        env_logger::init();

        Self {
            current_diag_mode: DiagSessionMode {
                mode: UdsSessionType::Default,
                tp_require: false,
                sec_level: security_access::SecurityLevelAccess::None,
                name: String::from("UDS Client"),
            },
            protocol: IsoTpProtocol::new(),
            basic_option: DiagServerBasicOptions {
                send_id: 0x784,
                recv_id: 0x7F0,
                timeout_cfg: TimeoutConfig {
                    read_timeout_ms: 5000,
                    write_timeout_ms: 5000,
                },
            },
            advanced_options: DiagServerAdvancedOptions {
                global_tp_id: 0,
                tester_present_interval_ms: 40000, // current S3 timeout is 50 seconds
                tester_present_require_response: false,
                global_session_control: false,
                tp_ext_id: None,
                command_cooldown_ms: 0,
            },
            tx,
            rx,
        }
    }

    pub fn init(&mut self) {
        self.protocol = IsoTpProtocol::new();
        self.protocol.init();
    }

    pub fn check_can_connection_status(&mut self) -> bool {
        self.protocol.connection_status
    }

    /// Send a command to the ECU and await its response
    pub fn send_command_with_response<T: Into<u8>>(&mut self, cmd: T, args: &[u8]) -> Vec<u8> {
        let mut data: [u8; 8] = [0, cmd.into(), 0, 0, 0, 0, 0, 0];
        let arg_len = args.len();
        data[2..(std::cmp::min(arg_len, 6) + 2)]
            .copy_from_slice(&args[..std::cmp::min(arg_len, 6)]);
        data[0] = (arg_len + 1) as u8;
        let msg = CanFrame::new(0x784, &data, true);

        self.protocol.send_receive(msg)
    }

    /// Send a command to the ECU and await its response
    pub fn send_command_no_response<T: Into<u8>>(&mut self, cmd: T, args: &[u8]) {
        let mut data: [u8; 8] = [0, cmd.into(), 0, 0, 0, 0, 0, 0];
        let arg_len = args.len();
        data[2..(std::cmp::min(arg_len, 6) + 2)]
            .copy_from_slice(&args[..std::cmp::min(arg_len, 6)]);
        data[0] = (arg_len + 1) as u8;
        let msg = CanFrame::new(0x784, &data, true);

        self.protocol.send(msg)
    }

    // pub async fn get_service_response(&mut self) -> String {
    //     //let rx_clone = Arc::clone(&self.rx);
    //     let mut lock = self.rx.lock().unwrap();
    //     let res = lock.recv().await;
    //     match res {
    //         Some(result) => {
    //             return result.response;
    //         }
    //         None => {return format!("Error: Receive None").to_string();}
    //     }
    // }
}

impl UdsServiceProvider for UDSClientSession {
    async fn new_uds_client(
        tx: UnboundedSender<ServiceRequest>,
        rx: UnboundedReceiver<ServiceResponse>,
    ) -> UDSClientSession {
        let mut client = UDSClientSession::async_default(tx, rx).await;
        client.init();

        client.send_command_with_response(UdsCommand::TesterPresent, &[]);

        let resp = client.uds_read_data_by_id(DataId::DiagState);
        let mut session_state: u8 = 0;
        let mut security_state: u8 = 0;
        match resp {
            UdsServiceResponse::Success(data) => {
                for line in data.console_output.lines() {
                    let parts: Vec<&str> = line.split(": ").collect();
                    if parts.len() == 2 {
                        match parts[0] {
                            "Session State" => {
                                session_state = parts[1].trim().parse().unwrap_or_default();
                            }
                            "Security State" => {
                                security_state = parts[1].trim().parse().unwrap_or_default();
                            }
                            _ => {}
                        }
                    }
                }

                client.current_diag_mode.mode = UdsSessionType::from_byte(session_state);
                client.current_diag_mode.sec_level = SecurityLevelAccess::from_byte(security_state);
            }
            UdsServiceResponse::Fail(_fail) => {}
        }

        client
    }
    fn invoke_read_data_by_id_service(&mut self, data_id: DataId) -> UdsServiceResponse {
        self.uds_read_data_by_id_setup();
        self.uds_read_data_by_id(data_id)
    }
    fn invoke_read_data_by_id_service_return_struct(
        &mut self,
        data_id: DataId,
    ) -> UdsMonitorViewResponse {
        self.uds_read_data_by_id_setup();
        //self.uds_set_session_mode(UdsSessionType::StreamMode);
        self.uds_read_data_by_id_return_struct(data_id)
    }

    fn invoke_reset_ecu_service(&mut self, reset_mode: ResetType) -> UdsServiceResponse {
        self.uds_ecu_reset_setup();
        self.uds_ecu_reset(reset_mode)
    }
    async fn invoke_routine_control_service(
        &mut self,
        routine_subfcn: RoutineControlSubfcn,
        routine_id: RoutineId,
        routine_control_option: &[u8],
    ) -> UdsServiceResponse {
        self.uds_routine_control_setup();

        self.uds_routine_control(
            routine_subfcn,
            &routine_id.to_bytes(),
            routine_control_option,
        )
        .await
    }
    async fn invoke_routine_control_service_get_result(
        &mut self,
        routine_subfcn: RoutineControlSubfcn,
        routine_id: RoutineId,
        routine_control_option: &[u8],
    ) -> UdsServiceResponse {
        self.uds_routine_control_setup_get_result();

        self.uds_routine_control_get_result(
            routine_subfcn,
            &routine_id.to_bytes(),
            routine_control_option,
        )
        .await
    }
    fn invoke_set_session_mode(&mut self, session_mode: UdsSessionType) -> UdsServiceResponse {
        self.uds_set_session_mode(session_mode)
    }
}

// #[cfg(test)]
// pub mod test {
//     use super::ecu_reset::ResetType;
//     use super::read_data_by_id::DataId;
//     use super::routine_control::RoutineControlSubfcn;
//     use super::security_access::SecurityLevelAccess;
//     use super::UDSClientSession;
//     use automotive_diag::uds::UdsSessionType;
//     use std::thread;
//     use std::time::Duration;

//     // #[test]
//     // async fn test_diag_session_control() {
//     //     let mut client = UDSClientSession::async_default().await;
//     //     client.init();
//     //     client.uds_set_session_mode(UdsSessionType::Extended.into());
//     //     client.uds_tester_present(1000000);
//     // }

//     // #[test]
//     // fn test_routine_control() {
//     //     let mut client = UDSClientSession::default();
//     //     client.init();
//     //     let routine_id = [0x03, 0xFF];
//     //     client.uds_routine_control(RoutineControlSubfcn::StartRoutine, &routine_id, &[]);
//     // }

//     // #[test]
//     // fn test_ecu_reset() {
//     //     let mut client = UDSClientSession::default();
//     //     client.init();
//     //     client.uds_ecu_reset(ResetType::HardReset.into());
//     // }

//     // #[test]
//     // fn test_security_access() {
//     //     let mut client = UDSClientSession::default();
//     //     client.init();

//     //     let key = client.uds_security_access_request_seed(SecurityLevelAccess::Level1RequestSeed);
//     //     client.uds_security_access_send_key(SecurityLevelAccess::Level1SendKey, &key);
//     //     thread::sleep(Duration::from_secs(1));

//     //     client.uds_ecu_reset(ResetType::HardReset.into());
//     // }

//     // #[test]
//     // fn test_read_data_by_id() {
//     //     let mut client = UDSClientSession::default();
//     //     client.init();
//     //     client.uds_read_data_by_id(DataId::SwitchGear);
//     //     client.uds_read_data_by_id(DataId::BikeState);
//     //     client.uds_read_data_by_id(DataId::ComponentError);
//     //     client.uds_read_data_by_id(DataId::ImuRaw);
//     //     client.uds_read_data_by_id(DataId::KeyfobState);
//     //     client.uds_read_data_by_id(DataId::PerformanceVehicle1);
//     //     client.uds_read_data_by_id(DataId::PerformanceVehicle2);
//     //     client.uds_read_data_by_id(DataId::AdcVoltage);
//     // }
// }
