//!  Provides methods to access different security levels

use crate::uds::UDSClientSession;

use automotive_diag::uds::UdsCommand;

impl UDSClientSession {
    /// Requests the ECU to go into a specific diagnostic session mode
    pub fn uds_security_access_request_seed(&mut self, sub_fcn: SecurityLevelAccess) -> Vec<u8> {
        let resp = self.send_command_with_response(UdsCommand::SecurityAccess, &[sub_fcn as u8]);

        if !resp.is_empty() && resp[0] == (UdsCommand::SecurityAccess as u8 + 0x40) {
            // Ensure that the length of `resp` is 8
            assert_eq!(resp.len(), 7);

            return resp[2..7].to_vec();
        }
        vec![]
    }

    /// Requests the ECU to go into a specific diagnostic session mode
    pub fn uds_security_access_send_key(&mut self, sub_fcn: SecurityLevelAccess, key: &[u8]) {
        let mut args: Vec<u8> = Vec::new();
        args.push(sub_fcn as u8);
        args.extend_from_slice(key);

        let resp = self.send_command_with_response(UdsCommand::SecurityAccess, &args);

        if !resp.is_empty() {
            if resp[0] == (UdsCommand::SecurityAccess as u8 + 0x40) {
                // Positive response, set security level
                self.current_diag_mode.sec_level = sub_fcn;
                self.current_diag_mode.tp_require = true;
            }
        } else {
            println!("Receive empty response for security access");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum SecurityLevelAccess {
    None = 0x01,
    Level1RequestSeed = 0x03,
    Level1SendKey = 0x04,
    Level2RequestSeed = 0x05,
    Level2SendKey = 0x06,
}

impl SecurityLevelAccess {
    pub fn from_byte(byte_value: u8) -> Self {
        match byte_value {
            0x01 => SecurityLevelAccess::None,
            0x03 => SecurityLevelAccess::Level1RequestSeed,
            0x04 => SecurityLevelAccess::Level1SendKey,
            0x05 => SecurityLevelAccess::Level2RequestSeed,
            0x06 => SecurityLevelAccess::Level2SendKey,
            _ => SecurityLevelAccess::None,
        }
    }
}
