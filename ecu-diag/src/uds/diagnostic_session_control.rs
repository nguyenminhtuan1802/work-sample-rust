//!  Provides methods to manipulate the ECUs diagnostic session mode

use crate::uds::UDSClientSession;
use crate::uds::UdsSericeResponseDetail;
use crate::uds::UdsServiceResponse;

use automotive_diag::uds::UdsCommand;

impl UDSClientSession {
    /// Requests the ECU to go into a specific diagnostic session mode
    pub fn uds_set_session_mode(&mut self, session_mode: UdsSessionType) -> UdsServiceResponse {
        let resp = self.send_command_with_response(
            UdsCommand::DiagnosticSessionControl,
            &[session_mode as u8],
        );

        if !resp.is_empty() {
            if resp[0] == (UdsCommand::DiagnosticSessionControl as u8 + 0x40) {
                self.current_diag_mode.mode = session_mode;
                // Success
                return UdsServiceResponse::Success(UdsSericeResponseDetail {
                    console_output: String::from("SUCCESS"),
                });
            } else {
                // Fail
                return UdsServiceResponse::Fail(UdsSericeResponseDetail {
                    console_output: String::from("FAIL\nNEGATIVE UDS RESPONSE"),
                });
            }
        }

        // Fail
        UdsServiceResponse::Fail(UdsSericeResponseDetail {
            console_output: String::from("FAIL\nEMPTY UDS RESPONSE"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UdsSessionType {
    /// Default diagnostic session mode (ECU is normally in this mode on startup)
    /// This session type does not require the diagnostic server to sent TesterPresent messages
    Default = 0x01,

    /// This diagnostic session mode enables all diagnostic services related to flashing or programming
    /// the ECU
    Programming = 0x02,

    /// This diagnostic session mode enabled all diagnostic services and allows adjusting
    /// ECU values
    Extended = 0x03,

    /// This diagnostic session enables all diagnostic services required to support safety system-related functions
    SafetySystem = 0x04,

    StreamMode = 0x08,

    Invalid = 0xFF,
}

impl UdsSessionType {
    pub fn from_byte(byte_value: u8) -> Self {
        match byte_value {
            0x01 => UdsSessionType::Default,
            0x02 => UdsSessionType::Programming,
            0x03 => UdsSessionType::Extended,
            0x04 => UdsSessionType::SafetySystem,
            0x08 => UdsSessionType::StreamMode,
            _ => UdsSessionType::Invalid,
        }
    }
}

impl From<UdsSessionType> for i32 {
    fn from(session_type: UdsSessionType) -> Self {
        match session_type {
            UdsSessionType::Default => UdsSessionType::Default as i32,
            UdsSessionType::Programming => UdsSessionType::Programming as i32,
            UdsSessionType::Extended => UdsSessionType::Extended as i32,
            UdsSessionType::SafetySystem => UdsSessionType::SafetySystem as i32,
            UdsSessionType::StreamMode => UdsSessionType::StreamMode as i32,
            UdsSessionType::Invalid => UdsSessionType::Invalid as i32,
        }
    }
}
