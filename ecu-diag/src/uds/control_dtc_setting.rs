//!  Provides methods to manipulate the ECUs diagnostic session mode

use crate::uds::UDSClientSession;

use automotive_diag::uds::UdsCommand;

impl UDSClientSession {
    /// Requests the ECU to go into a specific diagnostic session mode
    pub fn uds_control_dtc_setting(&mut self, sub_fcn: DTCSettingSubfcn) {
        self.send_command_with_response(UdsCommand::ControlDTCSetting, &[sub_fcn as u8]);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DTCSettingSubfcn {
    On = 0x01,
    Off = 0x02,
}
