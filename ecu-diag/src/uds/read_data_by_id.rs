//!  Provides methods to read data by identifier

use crate::api::UdsMonitorViewResponse;
use crate::api::UdsMonitorViewResponseDetail;
use crate::uds::UDSClientSession;
use crate::uds::UdsSericeResponseDetail;
use crate::uds::UdsServiceResponse;

use automotive_diag::uds::UdsCommand;

impl UDSClientSession {
    /// Requests the ECU to go into a specific diagnostic session mode
    pub fn uds_read_data_by_id(&mut self, sub_fcn: DataId) -> UdsServiceResponse {
        // Convert u16 to bytes using big-endian (most significant byte first)
        let sub_fcn_bytes = sub_fcn.to_bytes();

        // Prepare the payload by concatenating the sub_fcn bytes
        let payload = vec![sub_fcn_bytes[0], sub_fcn_bytes[1]];
        let resp = self.send_command_with_response(UdsCommand::ReadDataByIdentifier, &payload);

        if !resp.is_empty() {
            if resp[0] == (UdsCommand::ReadDataByIdentifier as u8 + 0x40) {
                // Success
                return UdsServiceResponse::Success(UdsSericeResponseDetail {
                    console_output: sub_fcn.parse_result(&resp[3..]),
                });
            } else {
                // Fail
                return UdsServiceResponse::Fail(UdsSericeResponseDetail {
                    console_output: String::from("FAIL\nNEGATIVE UDS RESPONSE"),
                });
            }
        }

        // Fail
        UdsServiceResponse::Fail(UdsSericeResponseDetail {
            console_output: String::from("FAIL\nEMPTY UDS RESPONSE"),
        })
    }

    pub fn uds_read_data_by_id_return_struct(&mut self, sub_fcn: DataId) -> UdsMonitorViewResponse {
        // Convert u16 to bytes using big-endian (most significant byte first)
        let sub_fcn_bytes = sub_fcn.to_bytes();

        // Prepare the payload by concatenating the sub_fcn bytes
        let payload = vec![sub_fcn_bytes[0], sub_fcn_bytes[1]];
        let resp = self.send_command_with_response(UdsCommand::ReadDataByIdentifier, &payload);

        if resp[0] == (UdsCommand::ReadDataByIdentifier as u8 + 0x40) {
            // Success
            let detail = sub_fcn.parse_result_to_struct(&resp[3..]);
            UdsMonitorViewResponse::Success(detail)
        } else {
            // Fail
            UdsMonitorViewResponse::Fail
        }
    }

    pub fn uds_read_data_by_id_setup(&mut self) {
        // if self.current_diag_mode.sec_level == SecurityLevelAccess::None {
        //     let key = self.uds_security_access_request_seed(SecurityLevelAccess::Level1RequestSeed);
        //     self.uds_security_access_send_key(SecurityLevelAccess::Level1SendKey, &key);
        // }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DataId {
    BikeState = 0x0100,
    SwitchGear = 0x0101,
    ComponentError = 0x0102,
    ImuRaw = 0x0103,
    KeyfobState = 0x0104,
    PerformanceVehicle1 = 0x0105,
    FirmwareVersion = 0x0106,
    AdcVoltage = 0x0107,
    Bms1 = 0x0108,
    Dashboard = 0x0109,
    PerformanceCharge = 0x010A,
    Bms2 = 0x010B,
    Bms3 = 0x010C,
    PerformanceVehicle2 = 0x010D,
    TempSensors = 0x010E,
    Obc = 0x010F,
    DiagState = 0x0110,
}

impl DataId {
    #[allow(clippy::wrong_self_convention)]
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            DataId::BikeState
            | DataId::SwitchGear
            | DataId::ComponentError
            | DataId::ImuRaw
            | DataId::KeyfobState
            | DataId::PerformanceVehicle1
            | DataId::PerformanceVehicle2
            | DataId::PerformanceCharge
            | DataId::FirmwareVersion
            | DataId::Bms1
            | DataId::Bms2
            | DataId::Bms3
            | DataId::AdcVoltage
            | DataId::TempSensors
            | DataId::Obc
            | DataId::DiagState
            | DataId::Dashboard => (*self as u16).to_be_bytes().to_vec(),
        }
    }

    #[allow(unreachable_patterns)]
    fn parse_result(&self, result: &[u8]) -> String {
        let mut output = String::new();
        match self {
            DataId::BikeState => {
                print!("Bike State len: {}, ", result.len());
                if result.len() == 2 {
                    print!("Bike State: {}, ", result[0]);
                    output.push_str(&format!("Bike State: {}, ", result[0]));

                    println!("Bike Lock: {}", result[1]);
                    output.push_str(&format!("Bike Lock: {}", result[1]));
                } else {
                    println!("Invalid data length for BikeState");
                    output.push_str("Invalid data length for BikeState");
                }
            }
            DataId::SwitchGear => {
                print!("Switch gear len: {}, ", result.len());
                if result.len() == 19 {
                    print!("Right Brake Switch: {}, ", result[0]);
                    output.push_str(&format!("Right Brake Switch: {}, ", result[0]));

                    print!("Left Brake Switch: {}, ", result[1]);
                    output.push_str(&format!("Left Brake Switch: {}, ", result[1]));

                    print!("Kill Switch: {}, ", result[2]);
                    output.push_str(&format!("Kill Switch: {}, ", result[2]));

                    print!("Power Switch: {}, ", result[3]);
                    output.push_str(&format!("Power Switch: {}, ", result[3]));

                    print!("Reverse Switch: {}, ", result[4]);
                    output.push_str(&format!("Reverse Switch: {}, ", result[4]));

                    //print!("Seat Switch: {}, ", result[5]);
                    //output.push_str(&format!("Seat Switch: {}, ", result[5]));

                    print!("Side Stand Switch: {}, ", result[6]);
                    output.push_str(&format!("Side Stand Switch: {}, ", result[6]));

                    //print!("Trip Switch: {}, ", result[7]);
                    //output.push_str(&format!("Trip Switch: {}, ", result[7]));

                    print!("Ride Mode Switch: {}, ", result[8]);
                    output.push_str(&format!("Ride Mode Switch: {}, ", result[8]));

                    print!("Hazard Switch: {}, ", result[9]);
                    output.push_str(&format!("Hazard Switch: {}, ", result[9]));

                    print!("Horn Switch: {}, ", result[10]);
                    output.push_str(&format!("Horn Switch: {}, ", result[10]));

                    print!("Right Indicator Switch: {}, ", result[11]);
                    output.push_str(&format!("Right Indicator Switch: {}, ", result[11]));

                    print!("Left Indicator Switch: {}, ", result[12]);
                    output.push_str(&format!("Left Indicator Switch: {}, ", result[12]));

                    //print!("Passing Switch: {}, ", result[13]);
                    //output.push_str(&format!("Passing Switch: {}, ", result[13]));

                    print!("High Beam Switch: {}, ", result[14]);
                    output.push_str(&format!("High Beam Switch: {}, ", result[14]));

                    print!("Start Switch: {}, ", result[15]);
                    output.push_str(&format!("Start Switch: {}, ", result[15]));

                    print!("Seat Switch: {}, ", result[16]);
                    output.push_str(&format!("Seat Switch: {}, ", result[16]));

                    print!("Trip Switch: {}, ", result[17]);
                    output.push_str(&format!("Trip Switch: {}, ", result[17]));

                    println!("Down Switch: {}", result[18]);
                    output.push_str(&format!("Down Switch: {}", result[18]));
                } else {
                    println!("Invalid data length for SwitchGear");
                    output.push_str("Invalid data length for SwitchGear");
                }
            }
            DataId::ComponentError => {
                println!("DTC result len: {}", result.len());

                if result.len() == 18 {
                    print!("System Component: {}, ", result[0]);
                    output.push_str(&format!("System Component: {}, ", result[0]));

                    print!("System Fault Code: {}, ", result[1]);
                    output.push_str(&format!("System Fault Code: {}, ", result[1]));

                    print!("System Level: {}, ", result[2]);
                    output.push_str(&format!("System Level: {}, ", result[2]));

                    print!("BMS Component: {}, ", result[3]);
                    output.push_str(&format!("BMS Component: {}, ", result[3]));

                    print!("BMS Fault Code: {}, ", result[4]);
                    output.push_str(&format!("BMS Fault Code: {}, ", result[4]));

                    print!("BMS Level: {}, ", result[5]);
                    output.push_str(&format!("BMS Level: {}, ", result[5]));

                    print!("MC Component: {}, ", result[6]);
                    output.push_str(&format!("MC Component: {}, ", result[6]));

                    print!("MC Fault Code: {}, ", result[7]);
                    output.push_str(&format!("MC Fault Code: {}, ", result[7]));

                    print!("MC Level: {}, ", result[8]);
                    output.push_str(&format!("MC Level: {}, ", result[8]));

                    print!("OBC Component: {}, ", result[9]);
                    output.push_str(&format!("OBC Component: {}, ", result[9]));

                    print!("OBC Fault Code: {}, ", result[10]);
                    output.push_str(&format!("OBC Fault Code: {}, ", result[10]));

                    print!("OBC Level: {}, ", result[11]);
                    output.push_str(&format!("OBC Level: {}, ", result[11]));

                    print!("Output Component: {}, ", result[12]);
                    output.push_str(&format!("Output Component: {}, ", result[12]));

                    print!("Output Fault Code: {}, ", result[13]);
                    output.push_str(&format!("Output Fault Code: {}, ", result[13]));

                    println!("Output Level: {}", result[14]);
                    output.push_str(&format!("Output Level: {}", result[14]));

                    print!("Feature Component: {}, ", result[15]);
                    output.push_str(&format!("Feature Component: {}, ", result[15]));

                    print!("Feature Fault Code: {}, ", result[16]);
                    output.push_str(&format!("Feature Fault Code: {}, ", result[16]));

                    println!("Feature Level: {}", result[17]);
                    output.push_str(&format!("Feature Level: {}", result[17]));
                } else {
                    println!("Invalid data length for Component error");
                    output.push_str("Invalid data length for Component error");
                }
            }
            DataId::ImuRaw => {
                println!("IMU len: {}, ", result.len());
                if result.len() == 24 {
                    let templates = [
                        "ACC X: ", "ACC Y: ", "ACC Z: ", "GYR X: ", "GYR Y: ", "GYR Z: ",
                    ];
                    let ends = [", ", ", ", ", ", ", ", ", ", "\n"];
                    for (i, chunk) in result.chunks_exact(4).enumerate() {
                        let mut bytes: [u8; 4] = [0; 4];
                        bytes.copy_from_slice(chunk);
                        output
                            .push_str(&format!("{}\n", print_float(templates[i], ends[i], bytes)));
                    }
                } else {
                    println!("Invalid data length for IMU");
                    output.push_str("Invalid data length for IMU");
                }
            }
            DataId::KeyfobState => {
                print!("keyfob len: {}, ", result.len());

                if result.len() == 3 {
                    print!("RKE: {}, ", result[0]);
                    output.push_str(&format!("RKE: {}, ", result[0]));

                    print!("PKE: {}, ", result[1]);
                    output.push_str(&format!("PKE: {}, ", result[1]));

                    println!("PKE Distance: {}", result[2]);
                    output.push_str(&format!("PKE Distance: {}", result[2]));
                } else {
                    println!("Invalid data length for Keyfob");
                    output.push_str("Invalid data length for Keyfob");
                }
            }
            DataId::PerformanceCharge => {
                println!("Performace Charge result len: {}", result.len());

                if result.len() == 12 {
                    let _bytes: [u8; 4] = [0; 4];
                    let mut i = 0;

                    print!("Target Charge SOC PCT: {}, ", result[i]);
                    output.push_str(&format!("Target Charge SOC PCT: {}\n", result[i]));

                    i += 1;

                    print!("Tartget Charge Hours Rem: {}, ", result[i]);
                    output.push_str(&format!("Tartget Charge Hours Rem: {}\n", result[i]));

                    i += 1;

                    print!("Tartget Charge Min Rem: {}, ", result[i]);
                    output.push_str(&format!("Tartget Charge Min Rem: {}\n", result[i]));

                    i += 1;

                    let mut bytes_u16: [u8; 2] = [0; 2];
                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Tartget Charge Range: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    print!("Charge Complete: {}, ", result[i]);
                    output.push_str(&format!("Charge Complete: {}\n", result[i]));

                    i += 1;

                    print!("SOC Limit: {}, ", result[i]);
                    output.push_str(&format!("SOC Limit: {}\n", result[i]));

                    i += 1;

                    print!("SOC Limit Selection Page: {}, ", result[i]);
                    output.push_str(&format!("SOC Limit Selection Page: {}\n", result[i]));

                    i += 1;
                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!("{} ", print_uint16("VA Limit: ", "\n", bytes_u16)));
                    i += 2;

                    print!("VA Limit Selection Page: {}, ", result[i]);
                    output.push_str(&format!("VA Limit Selection Page: {}\n", result[i]));

                    i += 1;

                    println!("Store Cable Noti: {}", result[i]);
                    output.push_str(&format!("Store Cable Noti: {}\n", result[i]));
                } else {
                    println!("Invalid data length for Performance");
                    output.push_str("Invalid data length for Performance");
                }
            }
            DataId::PerformanceVehicle1 => {
                println!("Performace Vehicle1 result len: {}", result.len());

                if result.len() == 17 {
                    let mut bytes: [u8; 4] = [0; 4];
                    let mut i = 0;

                    print!("Persist: {}, ", result[i]);
                    output.push_str(&format!("Persist: {}\n", result[i]));

                    i += 1;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_uint32("Odometer: ", "\n", bytes)));

                    i += 4;

                    // println!("ODOMETER BYTES: ");
                    // for byte in &bytes {
                    //     println!("{:02X}", byte);
                    // }

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_uint32("TripA: ", "\n", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_uint32("TripB: ", "\n", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_uint32("Last Charge: ", "\n", bytes)));
                    i += 4;

                    println!("Last index {} ", i);
                } else {
                    println!("Invalid data length for Performance");
                    output.push_str("Invalid data length for Performance");
                }
            }
            DataId::PerformanceVehicle2 => {
                println!("Performace Vehicle2 result len: {}", result.len());

                if result.len() == 15 {
                    // println!("Performace Vehicle2 BYTES: ");
                    // for byte in result {
                    //     println!("{:02X}", byte);
                    // }

                    let mut bytes: [u8; 4] = [0; 4];
                    let mut i = 0;
                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Efficiency: ", "\n", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Power PCT: ", "\n", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Speed: ", "\n", bytes)));

                    i += 4;

                    print!("TripID: {}, ", result[i]);
                    output.push_str(&format!("TripID: {}\n", result[i]));

                    i += 1;

                    print!("Trip Action: {}, ", result[i]);
                    output.push_str(&format!("Trip Action: {}\n", result[i]));

                    i += 1;

                    print!("Range: {}, ", result[i]);
                    output.push_str(&format!("Range: {}\n", result[i]));
                    i += 1;

                    println!("Last index {} ", i);
                } else {
                    println!("Invalid data length for Performance");
                    output.push_str("Invalid data length for Vehicle Metrics 2");
                }
            }
            DataId::FirmwareVersion => {
                print!("Firmware len: {}, ", result.len());
                if result.len() == 6 {
                    let mut bytes_u16: [u8; 2] = [0; 2];
                    bytes_u16.copy_from_slice(&result[0..2]);
                    output.push_str(&format!("{} ", print_uint16("148 Major Version: ", "\n", bytes_u16)));

                    bytes_u16.copy_from_slice(&result[2..4]);
                    output.push_str(&format!("{} ", print_uint16("148 Minor Version: ", "\n", bytes_u16)));

                    output.push_str(&format!("118 Major Version: {}\n", result[4]));
                    output.push_str(&format!("118 Minor Version: {}\n", result[5]));

                } else {
                    println!("Invalid data length for Firmware Version");
                    output.push_str("Invalid data length for Firmware Version");
                }
            }
            DataId::AdcVoltage => {
                print!("ADC len: {}, ", result.len());

                if result.len() == 20 {
                    let templates = [
                        "Volt 12V: ",
                        "Volt 5V: ",
                        "Volt 3V: ",
                        "Throttle PCT: ",
                        "Throttle Filt: ",
                    ];
                    let ends = ["\n", "\n", "\n", "\n", "\n"];
                    for (i, chunk) in result.chunks_exact(4).enumerate() {
                        let mut bytes: [u8; 4] = [0; 4];
                        bytes.copy_from_slice(chunk);
                        output.push_str(&format!("{} ", print_float(templates[i], ends[i], bytes)));
                    }
                } else {
                    println!("Invalid data length for ADC");
                    output.push_str("Invalid data length for ADC");
                }
            }
            DataId::Bms1 => {
                println!("BMS1 result len: {}", result.len());
                if result.len() == 18 {
                    let mut bytes: [u8; 4] = [0; 4];
                    let mut i = 0;

                    print!("BMS Status: {}, ", result[i]);
                    output.push_str(&format!("BMS Status: {}\n", result[i]));

                    i += 1;

                    print!("Pre-discharge relay: {}, ", result[i]);
                    output.push_str(&format!("Pre-discharge relay: {}\n", result[i]));

                    i += 1;

                    print!("Discharge relay: {}, ", result[i]);
                    output.push_str(&format!("Discharge relay: {}\n", result[i]));

                    i += 1;

                    print!("Charging relay: {}, ", result[i]);
                    output.push_str(&format!("Charging relay: {}\n", result[i]));

                    i += 1;

                    print!("DC-DC enable: {}, ", result[i]);
                    output.push_str(&format!("DC-DC enable: {}\n", result[i]));

                    i += 1;

                    print!("Charger: {}, ", result[i]);
                    output.push_str(&format!("Charger: {}\n", result[i]));

                    i += 1;

                    print!("SOC PCT: {}, ", result[i]);
                    output.push_str(&format!("SOC PCT: {}\n", result[i]));

                    i += 1;

                    print!("SOH PCT: {}, ", result[i]);
                    output.push_str(&format!("SOH PCT: {}\n", result[i]));

                    i += 1;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("BMS Voltage: ", "\n", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("BMS Current: ", "\n", bytes)));
                    i += 4;

                    print!("Alive counter: {}\n, ", result[i]);
                    output.push_str(&format!("Alive counter: {}\n", result[i]));
                    i += 1;

                    print!("DC-DC enable status: {}\n, ", result[i]);
                    output.push_str(&format!("DC-DC enable status: {}\n", result[i]));
                    i += 1;

                    println!("Last index is: {}", i);
                } else {
                    println!("Invalid data length for Bms");
                    output.push_str("Invalid data length for Bms1");
                }
            }
            DataId::Bms2 => {
                println!("BMS2 result len: {}", result.len());
                if result.len() == 8 {
                    let mut i = 0;

                    let mut bytes_u16: [u8; 2] = [0; 2];
                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Max discharge current: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Max regen current: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Highest cell voltage: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Lowest cell voltage: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    println!("Last index is: {}", i);

                    // This print skip priting errors for now
                } else {
                    println!("Invalid data length for Bms");
                    output.push_str("Invalid data length for Bms");
                }
            }
            DataId::Bms3 => {
                println!("BMS3 result len: {}", result.len());
                if result.len() == 6 {
                    let mut i = 0;
                    let mut bytes_u16: [u8; 2] = [0; 2];

                    print!("Max temp: {}, ", result[i]);
                    output.push_str(&format!("Max temp: {}\n", result[i]));

                    i += 1;

                    print!("Max temp number: {}, ", result[i]);
                    output.push_str(&format!("Max temp number: {}\n", result[i]));

                    i += 1;

                    print!("Min temp: {}, ", result[i]);
                    output.push_str(&format!("Min temp: {}\n", result[i]));

                    i += 1;

                    print!("Min temp number: {}, ", result[i]);
                    output.push_str(&format!("Min temp number: {}\n", result[i]));

                    i += 1;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Charge discharge cycles: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    println!("Last index is: {}", i);

                    // This print skip priting errors for now
                } else {
                    println!("Invalid data length for Bms3");
                    output.push_str("Invalid data length for Bms3");
                }
            }
            DataId::Obc => {
                println!("OBC result len: {}", result.len());
                if result.len() == 15 {
                    let mut i = 0;
                    let mut bytes_u16: [u8; 2] = [0; 2];

                    print!("Activation Status: {}, ", result[i]);
                    output.push_str(&format!("Activation Status: {}\n", result[i]));
                    i += 1;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Output DC Volt: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Output DC Current: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    print!("Max Temp: {}, ", result[i]);
                    output.push_str(&format!("Max Temp: {}\n", result[i]));
                    i += 1;

                    print!("AC Input Volt: {}, ", result[i]);
                    output.push_str(&format!("AC Input Volt: {}\n", result[i]));
                    i += 1;

                    print!("AC Input Current: {}, ", result[i]);
                    output.push_str(&format!("AC Input Current: {}\n", result[i]));
                    i += 1;

                    print!("Stop tx: {}, ", result[i]);
                    output.push_str(&format!("Stop tx: {}\n", result[i]));
                    i += 1;

                    print!("Alive_counter: {}, ", result[i]);
                    output.push_str(&format!("Alive_counter: {}\n", result[i]));
                    i += 1;

                    print!("Error 1 hardware: {}, ", result[i]);
                    output.push_str(&format!("Error 1 hardware: {}\n", result[i]));
                    i += 1;

                    print!("Error 2 temp: {}, ", result[i]);
                    output.push_str(&format!("Error 2 temp: {}\n", result[i]));
                    i += 1;

                    print!("Error 3 current: {}, ", result[i]);
                    output.push_str(&format!("Error 3 current: {}\n", result[i]));
                    i += 1;

                    print!("Error 4 volt in: {}, ", result[i]);
                    output.push_str(&format!("Error  4 volt in: {}\n", result[i]));
                    i += 1;

                    print!("Error 5 comn: {}, ", result[i]);
                    output.push_str(&format!("Error 5 comn: {}\n", result[i]));
                    i += 1;

                    println!("Last index is: {}", i);
                } else {
                    println!("Invalid data length for Bms3");
                    output.push_str("Invalid data length for Bms3");
                }
            }
            DataId::Dashboard => {
                println!("Dashboard result len: {}", result.len());
                if result.len() == 156 {
                    let mut bytes: [u8; 4] = [0; 4];

                    print!("Bike State: {}, ", result[0]);
                    output.push_str(&format!("Bike State: {}\n", result[0]));

                    println!("Bike Lock: {}", result[1]);
                    output.push_str(&format!("Bike Lock: {}\n", result[1]));

                    print!("Right Brake Switch: {}, ", result[2]);
                    output.push_str(&format!("Right Brake Switch: {}, ", result[2]));

                    print!("Left Brake Switch: {}, ", result[3]);
                    output.push_str(&format!("Left Brake Switch: {}, ", result[3]));

                    print!("Kill Switch: {}, ", result[4]);
                    output.push_str(&format!("Kill Switch: {}, ", result[4]));

                    print!("Power Switch: {}, ", result[5]);
                    output.push_str(&format!("Power Switch: {}, ", result[5]));

                    print!("Reverse Switch: {}, ", result[6]);
                    output.push_str(&format!("Reverse Switch: {}, ", result[6]));

                    //print!("Seat Switch: {}, ", result[7]);
                    //output.push_str(&format!("Seat Switch: {}, ", result[7]));

                    print!("Side Stand Switch: {}, ", result[8]);
                    output.push_str(&format!("Side Stand Switch: {}, ", result[8]));

                    //print!("Trip Switch: {}, ", result[9]);
                    //output.push_str(&format!("Trip Switch: {}, ", result[9]));

                    print!("Ride Mode Switch: {}, ", result[10]);
                    output.push_str(&format!("Ride Mode Switch: {}, ", result[10]));

                    print!("Hazard Switch: {}, ", result[11]);
                    output.push_str(&format!("Hazard Switch: {}, ", result[11]));

                    print!("Horn Switch: {}, ", result[12]);
                    output.push_str(&format!("Horn Switch: {}, ", result[12]));

                    print!("Right Indicator Switch: {}, ", result[13]);
                    output.push_str(&format!("Right Indicator Switch: {}, ", result[13]));

                    print!("Left Indicator Switch: {}, ", result[14]);
                    output.push_str(&format!("Left Indicator Switch: {}, ", result[14]));

                    //print!("Passing Switch: {}, ", result[15]);
                    //output.push_str(&format!("Passing Switch: {}, ", result[15]));

                    print!("High Beam Switch: {}, ", result[16]);
                    output.push_str(&format!("High Beam Switch: {}, ", result[16]));

                    print!("Start Switch: {}, ", result[17]);
                    output.push_str(&format!("Start Switch: {}, ", result[17]));

                    print!("Seat Switch: {}, ", result[18]);
                    output.push_str(&format!("Seat Switch: {}, ", result[18]));

                    print!("Trip Switch: {}, ", result[19]);
                    output.push_str(&format!("Trip Switch: {}, ", result[19]));

                    println!("Down Switch: {}", result[20]);
                    output.push_str(&format!("Down Switch: {}\n", result[20]));

                    print!("System Component: {}, ", result[21]);
                    output.push_str(&format!("System Component: {}, ", result[21]));

                    print!("System Fault Code: {}, ", result[22]);
                    output.push_str(&format!("System Fault Code: {}, ", result[22]));

                    print!("System Level: {}, ", result[23]);
                    output.push_str(&format!("System Level: {}, ", result[23]));

                    print!("BMS Component: {}, ", result[24]);
                    output.push_str(&format!("BMS Component: {}, ", result[24]));

                    print!("BMS Fault Code: {}, ", result[25]);
                    output.push_str(&format!("BMS Fault Code: {}, ", result[25]));

                    print!("BMS Level: {}, ", result[26]);
                    output.push_str(&format!("BMS Level: {}, ", result[26]));

                    print!("MC Component: {}, ", result[27]);
                    output.push_str(&format!("MC Component: {}, ", result[27]));

                    print!("MC Fault Code: {}, ", result[28]);
                    output.push_str(&format!("MC Fault Code: {}, ", result[28]));

                    print!("MC Level: {}, ", result[29]);
                    output.push_str(&format!("MC Level: {}, ", result[29]));

                    print!("OBC Component: {}, ", result[30]);
                    output.push_str(&format!("OBC Component: {}, ", result[30]));

                    print!("OBC Fault Code: {}, ", result[31]);
                    output.push_str(&format!("OBC Fault Code: {}, ", result[31]));

                    print!("OBC Level: {}, ", result[32]);
                    output.push_str(&format!("OBC Level: {}, ", result[32]));

                    print!("Output Component: {}, ", result[33]);
                    output.push_str(&format!("Output Component: {}, ", result[33]));

                    print!("Output Fault Code: {}, ", result[34]);
                    output.push_str(&format!("Output Fault Code: {}, ", result[34]));

                    println!("Output Level: {}", result[35]);
                    output.push_str(&format!("Output Level: {}", result[35]));

                    print!("Feature Component: {}, ", result[36]);
                    output.push_str(&format!("Feature Component: {}, ", result[36]));

                    print!("Feature Fault Code: {}, ", result[37]);
                    output.push_str(&format!("Feature Fault Code: {}, ", result[37]));

                    println!("Feature Level: {}", result[38]);
                    output.push_str(&format!("Feature Level: {}\n", result[38]));

                    let mut i = 39;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("ACC X: ", ", ", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("ACC Y: ", ", ", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("ACC Z: ", ", ", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("GYR X: ", ", ", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("GYR Y: ", ", ", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("GYR Z: ", ", ", bytes)));
                    i += 4;

                    print!("RKE: {}, ", result[i]);
                    output.push_str(&format!("RKE: {}, ", result[i]));
                    i += 1;

                    print!("PKE: {}, ", result[i]);
                    output.push_str(&format!("PKE: {}, ", result[i]));
                    i += 1;

                    println!("PKE Distance: {}", result[i]);
                    output.push_str(&format!("PKE Distance: {}\n", result[i]));
                    i += 1;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Volt 12V: ", ", ", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Volt 5V: ", ", ", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Volt 3V: ", ", ", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Throttle pct: ", ", ", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Throttle filt: ", ", ", bytes)));
                    i += 4;

                    println!("BMS status index: {}, ", i);

                    print!("BMS Status: {}, ", result[i]);
                    output.push_str(&format!("BMS Status: {}\n", result[i]));

                    i += 1;

                    print!("Pre-discharge relay: {}, ", result[i]);
                    output.push_str(&format!("Pre-discharge relay: {}\n", result[i]));

                    i += 1;

                    print!("Discharge relay: {}, ", result[i]);
                    output.push_str(&format!("Discharge relay: {}\n", result[i]));

                    i += 1;

                    print!("Charging relay: {}, ", result[i]);
                    output.push_str(&format!("Charging relay: {}\n", result[i]));

                    i += 1;

                    print!("DC-DC enable: {}, ", result[i]);
                    output.push_str(&format!("DC-DC enable: {}\n", result[i]));

                    i += 1;

                    print!("Charger: {}, ", result[i]);
                    output.push_str(&format!("Charger: {}\n", result[i]));

                    i += 1;

                    print!("SOC PCT: {}, ", result[i]);
                    output.push_str(&format!("SOC PCT: {}\n", result[i]));

                    i += 1;

                    print!("SOH PCT: {}, ", result[i]);
                    output.push_str(&format!("SOH PCT: {}\n", result[i]));

                    i += 1;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("BMS Voltage: ", "\n", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("BMS Current: ", "\n", bytes)));
                    i += 4;

                    print!("Alive counter: {}\n, ", result[i]);
                    output.push_str(&format!("Alive counter: {}\n", result[i]));
                    i += 1;

                    print!("DC-DC enable status: {}\n, ", result[i]);
                    output.push_str(&format!("DC-DC enable status: {}\n", result[i]));
                    i += 1;

                    let mut bytes_u16: [u8; 2] = [0; 2];
                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Max discharge current: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Max regen current: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Highest cell voltage: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Lowest cell voltage: ", "\n", bytes_u16)
                    ));
                    i += 2;

                    print!("Max temp: {}, ", result[i]);
                    output.push_str(&format!("Max temp: {}\n", result[i]));
                    //output.bms_max_temp = result[i];
                    i += 1;

                    print!("Max temp number: {}, ", result[i]);
                    output.push_str(&format!("Max temp number: {}\n", result[i]));
                    //output.bms_max_temp_number = result[i];
                    i += 1;

                    print!("Min temp: {}, ", result[i]);
                    output.push_str(&format!("Min temp: {}\n", result[i]));
                    //output.bms_min_temp = result[i];
                    i += 1;

                    print!("Min temp number: {}, ", result[i]);
                    output.push_str(&format!("Min temp number: {}\n", result[i]));
                    //output.bms_min_temp_number = result[i];
                    i += 1;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Charge discharge cycles: ", "\n", bytes_u16)
                    ));
                    //output.bms_charge_discharge_cycles = to_uint16(bytes_u16);
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!("{} ", print_uint16("fw tm major: ", "\n", bytes_u16)));
                    //output.fw_rt = to_uint16(bytes_u16);
                    println!("fw 1: {:?}", bytes_u16);
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!("{} ", print_uint16("fw tm minor: ", "\n", bytes_u16)));
                    //output.fw_tm = to_uint16(bytes_u16);
                    println!("fw 2: {:?}", bytes_u16);
                    i += 2;

                    println!("fw rt major: {}, ", result[i]);
                    output.push_str(&format!("fw rt major: {}\n", result[i]));
                    //output.fw_tm = to_uint16(bytes_u16);
                    i += 1;

                    println!("fw rt minor: {}, ", result[i]);
                    output.push_str(&format!("fw rt minor: {}\n", result[i]));
                    //output.fw_tm = to_uint16(bytes_u16);
                    i += 1;

                    println!("Persist: {}, ", result[i]);
                    output.push_str(&format!("Persist: {}\n", result[i]));
                    //output.vm_persist = result[i];
                    i += 1;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_uint32("Odometer: ", "\n", bytes)));
                    //output.vm_odometer = to_uint32(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_uint32("TripA: ", "\n", bytes)));
                    //output.vm_tripa = to_uint32(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_uint32("TripB: ", "\n", bytes)));
                    //output.vm_tripb = to_uint32(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_uint32("Last Charge: ", "\n", bytes)));
                    //output.vm_last_charge = to_uint32(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Efficiency: ", "\n", bytes)));
                    //output.vm_efficiency = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Power PCT: ", "\n", bytes)));
                    //output.vm_power_pct = to_float(bytes);
                    i += 4;

                    println!("Speed index: {}, ", i);

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Speed: ", "\n", bytes)));
                    //output.vm_speed = to_float(bytes);
                    i += 4;

                    println!("TripID: {}, ", result[i]);
                    println!("TripID index: {}, ", i);

                    output.push_str(&format!("TripID: {}\n", result[i]));
                    //output.vm_tripid = result[i];
                    i += 1;

                    print!("Trip Action: {}, ", result[i]);
                    output.push_str(&format!("Trip Action: {}\n", result[i]));
                    //output.vm_tripaction = result[i];
                    i += 1;

                    print!("Range: {}, ", result[i]);
                    output.push_str(&format!("Range: {}\n", result[i]));
                    //output.vm_tripaction = result[i];
                    i += 1;

                    /*print!("Target Charge SOC PCT: {}, ", result[i]);
                    output.push_str(&format!("Target Charge SOC PCT: {}\n", result[i]));
                    //output.cm_target_charge_soc_pct = result[i];
                    i += 1;

                    print!("Tartget Charge Hours Rem: {}, ", result[i]);
                    output.push_str(&format!("Tartget Charge Hours Rem: {}\n", result[i]));
                    //output.cm_target_charge_hours_rem = result[i];
                    i += 1;

                    print!("Tartget Charge Min Rem: {}, ", result[i]);
                    output.push_str(&format!("Tartget Charge Min Rem: {}\n", result[i]));
                    //output.cm_target_charge_min_rem = result[i];
                    i += 1;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Tartget Charge Range: ", "\n", bytes_u16)
                    ));
                    //output.cm_target_charge_range = to_uint16(bytes_u16);
                    i += 2;

                    println!("Charge Complete: {}, ", result[i]);
                    println!("Charge Complete index: {}, ", i);

                    output.push_str(&format!("Charge Complete: {}\n", result[i]));
                    //output.cm_charge_complete = result[i];
                    i += 1;

                    print!("SOC Limit: {}, ", result[i]);
                    output.push_str(&format!("SOC Limit: {}\n", result[i]));
                    //output.cm_soc_limit = result[i];
                    i += 1;

                    print!("SOC Limit Selection Page: {}, ", result[i]);
                    output.push_str(&format!("SOC Limit Selection Page: {}\n", result[i]));
                    //output.cm_soc_limit_selection_page = result[i];
                    i += 1;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!("{} ", print_uint16("VA Limit: ", "\n", bytes_u16)));
                    //output.cm_va_limit = to_uint16(bytes_u16);
                    i += 2;

                    print!("VA Limit Selection Page: {}, ", result[i]);
                    output.push_str(&format!("VA Limit Selection Page: {}\n", result[i]));
                    //output.cm_va_limit_selection_page = result[i];
                    i += 1;

                    println!("Store Cable Noti: {}", result[i]);
                    output.push_str(&format!("Store Cable Noti: {}\n", result[i]));
                    //output.cm_store_cable_noti = result[i];
                    i += 1;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Temp1: ", "\n", bytes)));
                    //output.temp1 = to_float(bytes);
                    println!("Temp 1: {}", print_float("Temp1: ", "\n", bytes));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Temp2: ", "\n", bytes)));
                    //output.temp2 = to_float(bytes);
                    println!("Temp 2: {}", print_float("Temp2: ", "\n", bytes));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Temp3: ", "\n", bytes)));
                    //output.temp3 = to_float(bytes);
                    println!("Temp 3: {}", print_float("Temp3: ", "\n", bytes));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Temp4: ", "\n", bytes)));
                    //output.temp4 = to_float(bytes);
                    println!("Temp 4: {}", print_float("Temp4: ", "\n", bytes));
                    i += 4;

                    print!("Activation Status: {}, ", result[i]);
                    output.push_str(&format!("Activation Status: {}\n", result[i]));
                    //output.obc_activation_status = result[i];
                    i += 1;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Output DC Volt: ", "\n", bytes_u16)
                    ));
                    //output.obc_output_dc_volt = to_uint16(bytes_u16);
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    output.push_str(&format!(
                        "{} ",
                        print_uint16("Output DC Current: ", "\n", bytes_u16)
                    ));
                    //output.obc_output_dc_current = to_uint16(bytes_u16);
                    i += 2;

                    print!("Max Temp: {}, ", result[i]);
                    output.push_str(&format!("Max Temp: {}\n", result[i]));
                    //output.obc_max_temp = result[i];
                    i += 1;

                    print!("AC Input Volt: {}, ", result[i]);
                    output.push_str(&format!("AC Input Volt: {}\n", result[i]));
                    //output.obc_input_volt = result[i];
                    i += 1;

                    print!("AC Input Current: {}, ", result[i]);
                    output.push_str(&format!("AC Input Current: {}\n", result[i]));
                    //output.obc_input_current = result[i];
                    i += 1;

                    print!("Stop tx: {}, ", result[i]);
                    output.push_str(&format!("Stop tx: {}\n", result[i]));
                    //output.obc_stop_tx = result[i];
                    i += 1;

                    print!("Alive_counter: {}, ", result[i]);
                    output.push_str(&format!("Alive_counter: {}\n", result[i]));
                    //output.obc_alive_counter = result[i];
                    i += 1;

                    print!("Error 1 hardware: {}, ", result[i]);
                    output.push_str(&format!("Error 1 hardware: {}\n", result[i]));
                    //output.obc_error1_hw = result[i];
                    i += 1;

                    print!("Error 2 temp: {}, ", result[i]);
                    output.push_str(&format!("Error 2 temp: {}\n", result[i]));
                    //output.obc_error2_temp = result[i];
                    i += 1;

                    print!("Error 3 current: {}, ", result[i]);
                    output.push_str(&format!("Error 3 current: {}\n", result[i]));
                    //output.obc_error3_voltln = result[i];
                    i += 1;

                    print!("Error 4 volt in: {}, ", result[i]);
                    output.push_str(&format!("Error  4 volt in: {}\n", result[i]));
                    //output.obc_error4_current = result[i];
                    i += 1;

                    print!("Error 5 comn: {}, ", result[i]);
                    output.push_str(&format!("Error 5 comn: {}\n", result[i]));
                    //output.obc_error5_comn = result[i];
                    i += 1;

                    print!("118 CPU Load: {}, ", result[i]);
                    output.push_str(&format!("118 CPU Load: {}\n", result[i]));
                    i += 1;

                    print!("148 CPU Load: {}, ", result[i]);
                    output.push_str(&format!("148 CPU Load: {}\n", result[i]));
                    i += 1; */

                    println!("Last index: {}", i);
                } else {
                    println!("Invalid data length for Dashboard");
                    output.push_str("Invalid data length for Dashboard");
                }
            }
            DataId::TempSensors => {
                if result.len() == 16 {
                    let mut bytes: [u8; 4] = [0; 4];
                    let mut i = 0;
                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Temp1: ", "\n", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Temp2: ", "\n", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Temp3: ", "\n", bytes)));
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    output.push_str(&format!("{} ", print_float("Temp4: ", "\n", bytes)));
                } else {
                    println!("Invalid data length for Temp Sensor");
                    output.push_str("Invalid data length for Temp Sensor");
                }
            }
            DataId::DiagState => {
                if result.len() == 2 {
                    output.push_str(&format!("Session State: {}\n", result[0]));
                    output.push_str(&format!("Security State: {}\n", result[1]));
                } else {
                    println!("Invalid data length for Temp Sensor");
                    output.push_str("Invalid data length for Temp Sensor");
                }
            }
            // Add other cases for different DataId variants...
            _ => {
                println!("Unknown DataId: {self:?}");
                output.push_str(&format!("Unknown DataId: {self:?}"));
            }
        }
        output
    }

    #[allow(unreachable_patterns)]
    fn parse_result_to_struct(&self, result: &[u8]) -> UdsMonitorViewResponseDetail {
        let mut output = UdsMonitorViewResponseDetail::default();

        #[allow(clippy::single_match)]
        match self {
            DataId::Dashboard => {
                println!("Dashboard result len: {}", result.len());
                if result.len() == 201 {
                    let mut bytes: [u8; 4] = [0; 4];

                    print!("Bike State: {}, ", result[0]);
                    output.bike_status = result[0];

                    println!("Bike Lock: {}", result[1]);
                    output.bike_lock = result[1];

                    print!("Right Brake Switch: {}, ", result[2]);
                    output.right_brake_sw = result[2];

                    print!("Left Brake Switch: {}, ", result[3]);
                    output.left_brake_sw = result[3];

                    print!("Kill Switch: {}, ", result[4]);
                    output.kill_sw = result[4];

                    print!("Power Switch: {}, ", result[5]);
                    output.power_sw = result[5];

                    print!("Reverse Switch: {}, ", result[6]);
                    output.reverse_sw = result[6];

                    //print!("Seat Switch: {}, ", result[7]);
                    //output.bike_status = result[7];

                    print!("Side Stand Switch: {}, ", result[8]);
                    output.side_stand_sw = result[8];

                    //print!("Trip Switch: {}, ", result[9]);
                    //output.bike_status = result[9];

                    print!("Ride Mode Switch: {}, ", result[10]);
                    output.ride_mode_sw = result[10];

                    print!("Hazard Switch: {}, ", result[11]);
                    output.hazard_sw = result[11];

                    print!("Horn Switch: {}, ", result[12]);
                    output.horn_sw = result[12];

                    print!("Right Indicator Switch: {}, ", result[13]);
                    output.right_indicator_sw = result[13];

                    print!("Left Indicator Switch: {}, ", result[14]);
                    output.left_indicator_sw = result[14];

                    //print!("Passing Switch: {}, ", result[15]);
                    //output.bike_status = result[15];

                    print!("High Beam Switch: {}, ", result[16]);
                    output.high_beam_sw = result[16];

                    print!("Start Switch: {}, ", result[17]);
                    output.start_sw = result[17];

                    print!("Seat Switch: {}, ", result[18]);
                    output.seat_sw = result[18];

                    print!("Trip Switch: {}, ", result[19]);
                    output.trip_sw = result[19];

                    println!("Down Switch: {}", result[20]);
                    output.down_sw = result[20];

                    print!("System Component: {}, ", result[21]);
                    //output.down_sw = result[20];

                    print!("System Fault Code: {}, ", result[22]);
                    output.dtc_syscode = result[22];

                    print!("System Level: {}, ", result[23]);
                    //output.down_sw = result[20];

                    print!("BMS Component: {}, ", result[24]);
                    //output.down_sw = result[20];

                    print!("BMS Fault Code: {}, ", result[25]);
                    output.dtc_bmscode = result[25];

                    print!("BMS Level: {}, ", result[26]);
                    //output.down_sw = result[20];

                    print!("MC Component: {}, ", result[27]);
                    //output.down_sw = result[20];

                    print!("MC Fault Code: {}, ", result[28]);
                    output.dtc_mccode = result[28];

                    print!("MC Level: {}, ", result[29]);
                    //output.down_sw = result[20];

                    print!("OBC Component: {}, ", result[30]);
                    //output.down_sw = result[20];

                    print!("OBC Fault Code: {}, ", result[31]);
                    output.dtc_obccode = result[31];

                    print!("OBC Level: {}, ", result[32]);
                    //output.down_sw = result[20];

                    print!("Output Component: {}, ", result[33]);
                    //output.down_sw = result[20];

                    print!("Output Fault Code: {}, ", result[34]);
                    output.dtc_outputcode = result[34];

                    println!("Output Level: {}", result[35]);
                    //output.down_sw = result[20];

                    print!("Feature Component: {}, ", result[36]);
                    //output.push_str(&format!("Feature Component: {}, ", result[15]));

                    print!("Feature Fault Code: {}, ", result[37]);
                    //output.push_str(&format!("Feature Fault Code: {}, ", result[16]));

                    println!("Feature Level: {}", result[38]);
                    //.push_str(&format!("Feature Level: {}", result[17]));

                    let mut i = 39;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("ACC X: ", ", ", bytes)));
                    output.acc_x = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("ACC Y: ", ", ", bytes)));
                    output.acc_y = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("ACC Z: ", ", ", bytes)));
                    output.acc_z = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("GYR X: ", ", ", bytes)));
                    output.gyr_x = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("GYR Y: ", ", ", bytes)));
                    output.gyr_y = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("GYR Z: ", ", ", bytes)));
                    output.gyr_z = to_float(bytes);
                    i += 4;

                    print!("RKE: {}, ", result[i]);
                    //utput.push_str(&format!("RKE: {}, ", result[i]));
                    output.rke = result[i];
                    i += 1;

                    print!("PKE: {}, ", result[i]);
                    //output.push_str(&format!("PKE: {}, ", result[i]));
                    output.pke = result[i];
                    i += 1;

                    println!("PKE Distance: {}", result[i]);
                    //output.push_str(&format!("PKE Distance: {}", result[i]));
                    output.pke_distance = result[i];
                    i += 1;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Volt 12V: ", ", ", bytes)));
                    output.adc_12v = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Volt 5V: ", ", ", bytes)));
                    output.adc_5v = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Volt 3V: ", ", ", bytes)));
                    output.adc_3v = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Throttle pct: ", ", ", bytes)));
                    output.throttle_pct = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //.push_str(&format!("{} ", print_float("Throttle filt: ", ", ", bytes)));
                    output.throttle_filt = to_float(bytes);
                    i += 4;

                    print!("BMS Status: {}, ", result[i]);
                    //output.push_str(&format!("BMS Status: {}\n", result[i]));
                    output.bms_status = result[i];

                    i += 1;

                    print!("Pre-discharge relay: {}, ", result[i]);
                    //output.push_str(&format!("Pre-discharge relay: {}\n", result[i]));
                    output.bms_predischarge_relay = result[i];

                    i += 1;

                    print!("Discharge relay: {}, ", result[i]);
                    //output.push_str(&format!("Discharge relay: {}\n", result[i]));
                    output.bms_discharge_relay = result[i];

                    i += 1;

                    print!("Charging relay: {}, ", result[i]);
                    //output.push_str(&format!("Charging relay: {}\n", result[i]));
                    output.bms_charging_relay = result[i];

                    i += 1;

                    print!("DC-DC enable: {}, ", result[i]);
                    //output.push_str(&format!("DC-DC enable: {}\n", result[i]));
                    output.bms_dcdc_enable = result[i];

                    i += 1;

                    print!("Charger: {}, ", result[i]);
                    //output.push_str(&format!("Charger: {}\n", result[i]));
                    output.bms_charger = result[i];

                    i += 1;

                    print!("SOC PCT: {}, ", result[i]);
                    //output.push_str(&format!("SOC PCT: {}\n", result[i]));
                    output.bms_soc_pct = result[i];

                    i += 1;

                    print!("SOH PCT: {}, ", result[i]);
                    //output.push_str(&format!("SOH PCT: {}\n", result[i]));
                    output.bms_soh_pct = result[i];

                    i += 1;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("BMS Voltage: ", "\n", bytes)));
                    output.bms_volt = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("BMS Current: ", "\n", bytes)));
                    output.bms_current = to_float(bytes);
                    i += 4;

                    print!("Alive counter: {}\n, ", result[i]);
                    //output.push_str(&format!("Alive counter: {}\n", result[i]));
                    output.bms_alive_counter = result[i];
                    i += 1;

                    print!("DC-DC enable status: {}\n, ", result[i]);
                    //output.push_str(&format!("DC-DC enable status: {}\n", result[i]));
                    output.bms_dcdc_enable_status = result[i];
                    i += 1;

                    let mut bytes_u16: [u8; 2] = [0; 2];
                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    // output.push_str(&format!(
                    //     "{} ",
                    //     print_uint16("Max discharge current: ", "\n", bytes_u16)
                    // ));
                    output.bms_max_discharge_current = to_uint16(bytes_u16);
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    // output.push_str(&format!(
                    //     "{} ",
                    //     print_uint16("Max regen current: ", "\n", bytes_u16)
                    // ));
                    output.bms_max_regen_current = to_uint16(bytes_u16);
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    // output.push_str(&format!(
                    //     "{} ",
                    //     print_uint16("Highest cell voltage: ", "\n", bytes_u16)
                    // ));
                    output.bms_highest_cell_volt = to_uint16(bytes_u16);
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    // output.push_str(&format!(
                    //     "{} ",
                    //     print_uint16("Lowest cell voltage: ", "\n", bytes_u16)
                    // ));
                    output.bms_lowest_cell_volt = to_uint16(bytes_u16);
                    i += 2;

                    print!("Max temp: {}, ", result[i]);
                    //output.push_str(&format!("Max temp: {}\n", result[i]));
                    output.bms_max_temp = result[i];
                    i += 1;

                    print!("Max temp number: {}, ", result[i]);
                    //output.push_str(&format!("Max temp number: {}\n", result[i]));
                    output.bms_max_temp_number = result[i];
                    i += 1;

                    print!("Min temp: {}, ", result[i]);
                    //output.push_str(&format!("Min temp: {}\n", result[i]));
                    output.bms_min_temp = result[i];
                    i += 1;

                    print!("Min temp number: {}, ", result[i]);
                    //output.push_str(&format!("Min temp number: {}\n", result[i]));
                    output.bms_min_temp_number = result[i];
                    i += 1;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    // output.push_str(&format!(
                    //     "{} ",
                    //     print_uint16("Charge discharge cycles: ", "\n", bytes_u16)
                    // ));
                    output.bms_charge_discharge_cycles = to_uint16(bytes_u16);
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);

                    output.fw_tm_major = to_uint16(bytes_u16);
                    println!("fw tm major: {:?}", bytes_u16);
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);

                    output.fw_tm_minor = to_uint16(bytes_u16);
                    println!("fw tm minor: {:?}", bytes_u16);
                    i += 2;

                    println!("fw rt major: {}, ", result[i]);
                    //output.push_str(&format!("fw rt major: {}\n", result[i]));
                    output.fw_rt_major = result[i];
                    i += 1;

                    println!("fw rt minor: {}, ", result[i]);
                    //output.push_str(&format!("fw rt major: {}\n", result[i]));
                    output.fw_rt_minor = result[i];
                    i += 1;

                    println!("Persist: {}, ", result[i]);
                    //output.push_str(&format!("Persist: {}\n", result[i]));
                    output.vm_persist = result[i];
                    i += 1;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_uint32("Odometer: ", "\n", bytes)));
                    output.vm_odometer = to_uint32(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_uint32("TripA: ", "\n", bytes)));
                    output.vm_tripa = to_uint32(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_uint32("TripB: ", "\n", bytes)));
                    output.vm_tripb = to_uint32(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_uint32("Last Charge: ", "\n", bytes)));
                    output.vm_last_charge = to_uint32(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Efficiency: ", "\n", bytes)));
                    output.vm_efficiency = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Power PCT: ", "\n", bytes)));
                    output.vm_power_pct = to_float(bytes);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Speed: ", "\n", bytes)));
                    output.vm_speed = to_float(bytes);
                    i += 4;

                    print!("TripID: {}, ", result[i]);
                    print!("TripID index: {}, ", i);

                    //output.push_str(&format!("TripID: {}\n", result[i]));
                    output.vm_tripid = result[i];
                    i += 1;

                    print!("Trip Action: {}, ", result[i]);
                    //output.push_str(&format!("Trip Action: {}\n", result[i]));
                    output.vm_tripaction = result[i];
                    i += 1;

                    print!("Range: {}, ", result[i]);
                    //output.push_str(&format!("Range: {}\n", result[i]));
                    output.vm_tripaction = result[i];
                    i += 1;

                    print!("Target Charge SOC PCT: {}, ", result[i]);
                    //output.push_str(&format!("Target Charge SOC PCT: {}\n", result[i]));
                    output.cm_target_charge_soc_pct = result[i];
                    i += 1;

                    print!("Tartget Charge Hours Rem: {}, ", result[i]);
                    //output.push_str(&format!("Tartget Charge Hours Rem: {}\n", result[i]));
                    output.cm_target_charge_hours_rem = result[i];
                    i += 1;

                    print!("Tartget Charge Min Rem: {}, ", result[i]);
                    //output.push_str(&format!("Tartget Charge Min Rem: {}\n", result[i]));
                    output.cm_target_charge_min_rem = result[i];
                    i += 1;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    // output.push_str(&format!(
                    //     "{} ",
                    //     print_uint16("Tartget Charge Range: ", "\n", bytes_u16)
                    // ));
                    output.cm_target_charge_range = to_uint16(bytes_u16);
                    i += 2;

                    print!("Charge Complete: {}, ", result[i]);
                    //output.push_str(&format!("Charge Complete: {}\n", result[i]));
                    output.cm_charge_complete = result[i];
                    i += 1;

                    print!("SOC Limit: {}, ", result[i]);
                    //output.push_str(&format!("SOC Limit: {}\n", result[i]));
                    output.cm_soc_limit = result[i];
                    i += 1;

                    print!("SOC Limit Selection Page: {}, ", result[i]);
                    //output.push_str(&format!("SOC Limit Selection Page: {}\n", result[i]));
                    output.cm_soc_limit_selection_page = result[i];
                    i += 1;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    //output.push_str(&format!("{} ", print_uint16("VA Limit: ", "\n", bytes_u16)));
                    output.cm_va_limit = to_uint16(bytes_u16);
                    i += 2;

                    print!("VA Limit Selection Page: {}, ", result[i]);
                    //output.push_str(&format!("VA Limit Selection Page: {}\n", result[i]));
                    output.cm_va_limit_selection_page = result[i];
                    i += 1;

                    println!("Store Cable Noti: {}", result[i]);
                    //output.push_str(&format!("Store Cable Noti: {}\n", result[i]));
                    output.cm_store_cable_noti = result[i];
                    i += 1;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Speed: ", "\n", bytes)));
                    output.temp1 = to_float(bytes);
                    println!("Temp 1: {}", output.temp1);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Speed: ", "\n", bytes)));
                    output.temp2 = to_float(bytes);
                    println!("Temp 2: {}", output.temp1);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Speed: ", "\n", bytes)));
                    output.temp3 = to_float(bytes);
                    println!("Temp 3: {}", output.temp1);
                    i += 4;

                    bytes.copy_from_slice(&result[i..i + 4]);
                    //output.push_str(&format!("{} ", print_float("Speed: ", "\n", bytes)));
                    output.temp4 = to_float(bytes);
                    println!("Temp 4: {}", output.temp1);
                    i += 4;

                    print!("Activation Status: {}, ", result[i]);
                    //output.push_str(&format!("Activation Status: {}\n", result[i]));
                    output.obc_activation_status = result[i];
                    i += 1;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    // output.push_str(&format!(
                    //     "{} ",
                    //     print_uint16("Output DC Volt: ", "\n", bytes_u16)
                    // ));
                    output.obc_output_dc_volt = to_uint16(bytes_u16);
                    i += 2;

                    bytes_u16.copy_from_slice(&result[i..i + 2]);
                    // output.push_str(&format!(
                    //     "{} ",
                    //     print_uint16("Output DC Current: ", "\n", bytes_u16)
                    // ));
                    output.obc_output_dc_current = to_uint16(bytes_u16);
                    i += 2;

                    print!("Max Temp: {}, ", result[i]);
                    //output.push_str(&format!("Max Temp: {}\n", result[i]));
                    output.obc_max_temp = result[i];
                    i += 1;

                    print!("AC Input Volt: {}, ", result[i]);
                    //output.push_str(&format!("AC Input Volt: {}\n", result[i]));
                    output.obc_input_volt = result[i];
                    i += 1;

                    print!("AC Input Current: {}, ", result[i]);
                    //output.push_str(&format!("AC Input Current: {}\n", result[i]));
                    output.obc_input_current = result[i];
                    i += 1;

                    print!("Stop tx: {}, ", result[i]);
                    //output.push_str(&format!("Stop tx: {}\n", result[i]));
                    output.obc_stop_tx = result[i];
                    i += 1;

                    print!("Alive_counter: {}, ", result[i]);
                    //output.push_str(&format!("Alive_counter: {}\n", result[i]));
                    output.obc_alive_counter = result[i];
                    i += 1;

                    print!("Error 1 hardware: {}, ", result[i]);
                    //output.push_str(&format!("Error 1 hardware: {}\n", result[i]));
                    output.obc_error1_hw = result[i];
                    i += 1;

                    print!("Error 2 temp: {}, ", result[i]);
                    //output.push_str(&format!("Error 2 temp: {}\n", result[i]));
                    output.obc_error2_temp = result[i];
                    i += 1;

                    print!("Error 3 current: {}, ", result[i]);
                    //output.push_str(&format!("Error 3 current: {}\n", result[i]));
                    output.obc_error3_voltln = result[i];
                    i += 1;

                    print!("Error 4 volt in: {}, ", result[i]);
                    //output.push_str(&format!("Error  4 volt in: {}\n", result[i]));
                    output.obc_error4_current = result[i];
                    i += 1;

                    print!("Error 5 comn: {}, ", result[i]);
                    //output.push_str(&format!("Error 5 comn: {}\n", result[i]));
                    output.obc_error5_comn = result[i];
                    i += 1;

                    print!("118 CPU Load: {}, ", result[i]);
                    output.cpu_118 = result[i];
                    i += 1;

                    print!("148 CPU Load: {}, ", result[i]);
                    output.cpu_148 = result[i];
                    i += 1;

                    println!("Last index: {}", i);
                } else {
                    println!("Invalid data length for Dashboard");
                    //output.push_str("Invalid data length for Dashboard");
                }
            }
            _ => {}
        }
        output
    }
}

fn print_float(template: &str, end: &str, bytes: [u8; 4]) -> String {
    let mut output = String::new();

    // Assuming little-endian byte order
    let bits = u32::from_le_bytes(bytes);
    let float_value = f32::from_bits(bits);
    // Format the float with two significant digits
    let formatted_float = format!("{:.2}", float_value);
    print!("{template}{formatted_float}{end}");
    output.push_str(&format!("{template}{formatted_float}{end}"));

    output
}

fn to_float(bytes: [u8; 4]) -> f32 {
    let bits = u32::from_le_bytes(bytes);
    f32::from_bits(bits)
}

fn print_uint32(template: &str, end: &str, bytes: [u8; 4]) -> String {
    let mut output = String::new();

    // Assuming little-endian byte order
    let bits = u32::from_le_bytes(bytes);
    print!("{template}{bits}{end}");
    output.push_str(&format!("{template}{bits}{end}"));

    output
}

fn to_uint32(bytes: [u8; 4]) -> u32 {
    u32::from_le_bytes(bytes)
}

fn to_uint16(bytes: [u8; 2]) -> u16 {
    u16::from_le_bytes(bytes)
}

fn print_uint16(template: &str, end: &str, bytes: [u8; 2]) -> String {
    let mut output = String::new();

    // Assuming little-endian byte order
    let bits = u16::from_le_bytes(bytes);
    print!("{template}{bits}{end}");
    output.push_str(&format!("{template}{bits}{end}"));

    output
}
