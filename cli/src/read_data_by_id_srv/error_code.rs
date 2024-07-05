use clap::Args;
use ecu_diag::api::UdsServiceProvider;
use ecu_diag::uds::read_data_by_id::DataId;
use ecu_diag::uds::UDSClientSession;

#[derive(Args, Clone, Debug)]
pub struct ErrorCodeCmd {}

impl ErrorCodeCmd {
    pub fn run(&self, client: &mut UDSClientSession) {
        client.invoke_read_data_by_id_service(DataId::ComponentError);
    }
}
