//!  Provides methods to manipulate the ECUs diagnostic session mode

use crate::uds::UDSClientSession;

use automotive_diag::uds::UdsCommand;

impl UDSClientSession {
    /// Requests the ECU to go into a specific diagnostic session mode
    pub fn uds_link_control(&mut self, mode: LinkControlMode, param: &[u8]) {
        let mut args: [u8; 8] = [0; 8];
        args[0] = mode as u8;
        args[2..(std::cmp::min(param.len(), 6) + 2)]
            .copy_from_slice(&param[..std::cmp::min(param.len(), 6)]);
        self.send_command_with_response(UdsCommand::LinkControl, &args);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LinkControlMode {
    VerifyModeTransitionWithFixedParameter = 0x01,
    VerifyModeTransitionWithSpecificParameter = 0x02,
    TransitionMode = 0x03,
}
