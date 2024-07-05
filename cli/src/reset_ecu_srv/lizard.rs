use clap::Args;
use ecu_diag::api::UdsServiceProvider;
use ecu_diag::uds::ecu_reset::ResetType;
use ecu_diag::uds::UDSClientSession;

#[derive(Args, Clone, Debug)]
pub struct ResetLizardCmd {}

impl ResetLizardCmd {
    pub fn run(&self, client: &mut UDSClientSession) {
        client.invoke_reset_ecu_service(ResetType::LizardReset);
    }
}
