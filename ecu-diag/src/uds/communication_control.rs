//!  Provides methods to manipulate the ECUs diagnostic session mode

use crate::uds::UDSClientSession;

use automotive_diag::uds::{encode_communication_type, CommunicationLevel, UdsCommand};
use automotive_diag::uds::{CommunicationType, Subnet};

impl UDSClientSession {
    /// Requests the ECU to go into a specific diagnostic session mode
    pub fn uds_communication_control(
        &mut self,
        communication_type: CommunicationType,
        subnet: Subnet,
        comm_level: CommunicationLevel,
    ) {
        let level: u8 = comm_level.into();
        let communication_type = encode_communication_type(communication_type, subnet);
        self.send_command_with_response(
            UdsCommand::CommunicationControl,
            &[level, communication_type],
        );
    }
}
