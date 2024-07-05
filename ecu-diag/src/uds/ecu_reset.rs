//!  Provides methods to manipulate the ECUs diagnostic session mode

use crate::uds::UDSClientSession;
use crate::uds::UdsSericeResponseDetail;
use crate::uds::UdsServiceResponse;

use automotive_diag::uds::UdsCommand;

use super::security_access::SecurityLevelAccess;

impl UDSClientSession {
    /// Requests the ECU to go into a specific diagnostic session mode
    pub fn uds_ecu_reset(&mut self, reset_mode: ResetType) -> UdsServiceResponse {
        self.send_command_no_response(UdsCommand::ECUReset, &[reset_mode as u8]);
        UdsServiceResponse::Success(UdsSericeResponseDetail {
            console_output: String::from("SUCCESS"),
        })
    }

    pub fn uds_ecu_reset_setup(&mut self) {
        if self.current_diag_mode.sec_level == SecurityLevelAccess::None {
            let key = self.uds_security_access_request_seed(SecurityLevelAccess::Level1RequestSeed);
            self.uds_security_access_send_key(SecurityLevelAccess::Level1SendKey, &key);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResetType {
    HardReset = 0x01,
    KeyOffReset = 0x02,
    SoftReset = 0x03,
    EnableRapidPowerShutDown = 0x04,
    DisableRapidPowerShutDown = 0x05,
    RealtimeReset = 0x40,
    TelematicReset = 0x41,
    ImxReset = 0x42,
    Esp32WifiReset = 0x43,
    Esp32BleReset = 0x44,
    QuectelReset = 0x45,
    LizardReset = 0x46,
    CendricReset = 0x47,
}
