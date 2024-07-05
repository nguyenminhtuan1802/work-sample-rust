use clap::Args;
use ecu_diag::api::UdsServiceProvider;
use ecu_diag::uds::routine_control::{RoutineControlSubfcn, RoutineId};
use ecu_diag::uds::UDSClientSession;

#[derive(Args, Clone, Debug)]
pub struct EnableImxHmiCmd {}

#[allow(unused_must_use)]
impl EnableImxHmiCmd {
    #[allow(dead_code)]
    pub fn run(&self, client: &mut UDSClientSession) {
        client.invoke_routine_control_service(
            RoutineControlSubfcn::StartRoutine,
            RoutineId::EnableImxHmi,
            &[],
        );
    }
}
