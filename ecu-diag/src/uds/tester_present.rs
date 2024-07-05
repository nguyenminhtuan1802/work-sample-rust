//!  Provides methods to ping ECU server

use crate::uds::UDSClientSession;

use automotive_diag::uds::UdsCommand;

impl UDSClientSession {
    /// Requests the ECU to go into a specific diagnostic session mode
    pub fn uds_tester_present(&mut self, elapsed_time: u128) {
        if self.current_diag_mode.tp_require
            && elapsed_time > self.advanced_options.tester_present_interval_ms as u128
        {
            self.send_command_with_response(UdsCommand::TesterPresent, &[]);
        }
    }
}
