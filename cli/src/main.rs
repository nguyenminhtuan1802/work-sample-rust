#[allow(unused_imports)]
use clap::{CommandFactory, Parser, Subcommand};

pub(crate) mod diag_session_srv;
pub(crate) mod read_data_by_id_srv;
pub(crate) mod reset_ecu_srv;
pub(crate) mod routine_control_srv;

pub(crate) use diag_session_srv::DiagnosticSessionServiceCmd;
pub(crate) use read_data_by_id_srv::ReadDataServiceCmd;
pub(crate) use reset_ecu_srv::ResetEcuServiceCmd;
pub(crate) use routine_control_srv::RoutineControlServiceCmd;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "\x1B[94m _____ ____  _   _     _______  _                             _   _      \x1B[0m\r
\x1B[94m|_   _/ __ \\| \\ | |   / /  __ \\(_)                           | | (_)     \x1B[0m\r
\x1B[94m  | || |  | |  \\| |  / /| |  | |_  __ _  __ _ _ __   ___  ___| |_ _  ___ \x1B[0m\r
\x1B[94m  | || |  | | . ` | / / | |  | | |/ _` |/ _` | '_ \\ / _ \\/ __| __| |/ __|\x1B[0m\r
\x1B[94m _| || |__| | |\\  |/ /  | |__| | | (_| | (_| | | | | (_) \\__ \\ |_| | (__ \x1B[0m\r
\x1B[94m|_____\\____/|_| \\_/_/   |_____/|_|\\__,_|\\__, |_| |_|\\___/|___/\\__|_|\\___|\x1B[0m\r
\x1B[94m                                         __/ |                           \x1B[0m\r
\x1B[94m                                        |___/                            \x1B[0m\r
ION Diagnostic tool is a command line program to debug ION bike's VCU. \n
Comply with UDS (ISO-14229-1) and PCAN-ISO-TP (ISO 15765-2)"
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: UDSService,

    /// Stream output continuously
    #[arg(short, long, default_value_t = false)]
    stream: bool,
}

#[derive(Subcommand)]
enum UDSService {
    /// Read Data Service
    Read(ReadDataServiceCmd),
    /// Reset ECU Service
    Reset(ResetEcuServiceCmd),
    /// Routine Control Service
    Routine(RoutineControlServiceCmd),
    /// Set Diagnostic Session Mode
    SetMode(DiagnosticSessionServiceCmd),
}

fn main() {
    // let cli = Cli::parse();

    // let mut client = UDSClientSession::new_uds_client();

    // if cli.stream {
    //     let start_time = Instant::now(); // Record the starting time{
    //     loop {
    //         let cli = &cli;
    //         match &cli.command {
    //             UDSService::Read(c) => c.clone().run(&mut client),
    //             UDSService::Reset(_c) => {
    //                 let mut cmd = Cli::command();
    //                 cmd.error(
    //                     ErrorKind::ArgumentConflict,
    //                     "Reset ECU Service does not support streaming output.",
    //                 )
    //                 .exit();
    //             }
    //             UDSService::Routine(c) => c.clone().run(&mut client),
    //             UDSService::SetMode(_c) => {
    //                 let mut cmd = Cli::command();
    //                 cmd.error(
    //                     ErrorKind::ArgumentConflict,
    //                     "Set Diagnostic Service does not support streaming output.",
    //                 )
    //                 .exit();
    //             }
    //         }

    //         thread::sleep(Duration::from_secs(1));
    //         client.uds_tester_present(start_time.elapsed().as_millis());
    //     }
    // } else {
    //     match cli.command {
    //         UDSService::Read(c) => c.run(&mut client),
    //         UDSService::Reset(c) => c.run(&mut client),
    //         UDSService::Routine(c) => c.run(&mut client),
    //         UDSService::SetMode(c) => c.run(&mut client),
    //     }
    // }
}
