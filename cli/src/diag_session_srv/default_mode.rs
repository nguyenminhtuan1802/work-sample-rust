use clap::Args;
use ecu_diag::api::UdsServiceProvider;
use ecu_diag::uds::diagnostic_session_control::UdsSessionType;
use ecu_diag::uds::UDSClientSession;

#[derive(Args, Clone, Debug)]
pub struct DefaultModeCmd {}

impl DefaultModeCmd {
    pub fn run(&self, client: &mut UDSClientSession) {
        client.invoke_set_session_mode(UdsSessionType::Default);
    }
}
