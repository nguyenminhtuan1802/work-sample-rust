use clap::{Args, Subcommand};
use ecu_diag::uds::UDSClientSession;

mod default_mode;

pub(crate) use default_mode::DefaultModeCmd;

#[derive(Args, Clone, Debug)]
pub struct DiagnosticSessionServiceCmd {
    #[command(subcommand)]
    pub subcommand: DiagnosticSessionServiceSubCmd,
}

#[derive(Subcommand, Clone, Debug)]
pub enum DiagnosticSessionServiceSubCmd {
    /// To disable simulate input and trigger output routine
    DefaultMode(DefaultModeCmd),
}

#[allow(dead_code)]
impl DiagnosticSessionServiceCmd {
    pub fn run(self, client: &mut UDSClientSession) {
        match self.subcommand {
            DiagnosticSessionServiceSubCmd::DefaultMode(c) => c.run(client),
        }
    }
}
