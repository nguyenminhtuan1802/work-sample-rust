//!  Provides methods to manipulate the ECUs diagnostic session mode

use crate::uds::UDSClientSession;
use crate::uds::UdsSericeResponseDetail;
use crate::uds::UdsServiceResponse;

use automotive_diag::uds::UdsCommand;

use super::security_access::SecurityLevelAccess;
use crate::uds::diagnostic_session_control::UdsSessionType;
use chrono::NaiveDate;
use std::fmt;

impl UDSClientSession {
    /// Requests the ECU to start a service
    pub async fn uds_routine_control(
        &mut self,
        sub_fcn: RoutineControlSubfcn,
        routine_id: &[u8],
        routine_control_option: &[u8],
    ) -> UdsServiceResponse {
        let mut args: Vec<u8> = Vec::new();
        args.push(sub_fcn as u8);
        args.extend_from_slice(routine_id);
        args.extend_from_slice(routine_control_option);

        let value = u16::from_be_bytes([routine_id[0], routine_id[1]]);
        let mut request = ServiceRequest {
            domain: Domain::InvalidDomain,
            command: 0x00,
        };

        if Self::is_connectivity_service(value, &mut request) {
            if self.tx.send(request).is_err() {
                println!("receiver dropped");
                return UdsServiceResponse::Fail(UdsSericeResponseDetail {
                    console_output: String::from("FAIL"),
                });
            }
            // Success
            UdsServiceResponse::Success(UdsSericeResponseDetail {
                console_output: String::from("SUCCESS"),
            })
        } else {
            let resp = self.send_command_with_response(UdsCommand::RoutineControl, &args);

            if !resp.is_empty() {
                if resp[0] == (UdsCommand::RoutineControl as u8 + 0x40) {
                    // Success
                    return UdsServiceResponse::Success(UdsSericeResponseDetail {
                        console_output: String::from("SUCCESS"),
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
    }

    pub fn uds_routine_control_setup(&mut self) {
        if self.current_diag_mode.mode != UdsSessionType::Programming {
            self.uds_set_session_mode(UdsSessionType::Programming);
        }
        if self.current_diag_mode.sec_level == SecurityLevelAccess::None {
            let key = self.uds_security_access_request_seed(SecurityLevelAccess::Level1RequestSeed);
            self.uds_security_access_send_key(SecurityLevelAccess::Level1SendKey, &key);
        }
    }

    /// Requests the ECU to get service results
    pub async fn uds_routine_control_get_result(
        &mut self,
        sub_fcn: RoutineControlSubfcn,
        routine_id: &[u8],
        routine_control_option: &[u8],
    ) -> UdsServiceResponse {
        let mut args: Vec<u8> = Vec::new();
        args.push(sub_fcn as u8);
        args.extend_from_slice(routine_id);
        args.extend_from_slice(routine_control_option);

        let value = u16::from_be_bytes([routine_id[0], routine_id[1]]);
        let mut unused = ServiceRequest {
            domain: Domain::InvalidDomain,
            command: 0x00,
        };

        if Self::is_connectivity_service(value, &mut unused) {
            #[allow(unused_assignments)]
            let mut returned_string = "".to_string();

            let res = self.rx.try_recv();

            match res {
                Ok(message) => {
                    // Message received successfully
                    returned_string = message.response;
                }
                Err(error) => match error {
                    tokio::sync::mpsc::error::TryRecvError::Empty => {
                        // Channel is empty
                        log::debug!("Channel is empty");
                        println!("Channel is empty");
                        returned_string =  "NO RESULT TO SHOW\nNO SERVICE IS RUNNING OR CURRENT SERVICE IS STILL RUNNING".to_string();
                    }
                    tokio::sync::mpsc::error::TryRecvError::Disconnected => {
                        // Channel is empty
                        log::debug!("Channel is disconnected");
                        println!("Channel is disconnected");
                        returned_string =
                            "NO RESULT: RECEIVING CHANNEL IS DISCONNECTED. TRY RESTART THE APP"
                                .to_string();
                    }
                },
            }

            // Success
            UdsServiceResponse::Success(UdsSericeResponseDetail {
                console_output: format!("SUCCESS\n{}", returned_string),
            })
        } else {
            let resp = self.send_command_with_response(UdsCommand::RoutineControl, &args);

            log::debug!("RESPONSE FROM SERVICE: {:02X?}", resp);
            println!("RESPONSE FROM SERVICE: {:02X?}", resp);

            if resp[0] == (UdsCommand::RoutineControl as u8 + 0x40) {
                let routine_id: i16 = ((resp[2] as i16) << 8) | (resp[3] as i16);

                if routine_id == 0x020D {
                    let num_scan: u8 = resp[4];
                    let signal_strength: i16 = ((resp[5] as i16) << 8) | (resp[6] as i16);

                    // Success
                    // Format the output string
                    if resp[4] == 0xFF && resp[5] == 0xFF && resp[6] == 0xFF {
                        let console_output = "SUCCESS\nNO WIFI SCAN RESULT".to_string();

                        UdsServiceResponse::Success(UdsSericeResponseDetail { console_output })
                    } else {
                        let console_output = format!(
                            "SUCCESS\nNumber of wifi Scan: {}\nBest Signal Strength: {}",
                            num_scan, signal_strength
                        );
                        UdsServiceResponse::Success(UdsSericeResponseDetail { console_output })
                    }
                } else if routine_id == 0x020E {
                    if resp.len() == 9
                        && resp[4] == 0x00
                        && resp[5] == 0x00
                        && resp[6] == 0x00
                        && resp[7] == 0x00
                        && resp[8] == 0xFF
                    {
                        let console_output = "SUCCESS\nNO WIFI IP".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else {
                        let console_output = format!(
                            "SUCCESS\nWIFI IP: {}.{}.{}.{}",
                            resp[4], resp[5], resp[6], resp[7]
                        );
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    }
                } else if routine_id == 0x020F {
                    /*WIFI RESTART APP IMX SERVICE*/
                    if resp[4] == 0x01 {
                        let console_output = "SUCCESS\nWIFI INIT SUCCESS".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else if resp[4] == 0x02 {
                        let console_output = "SUCCESS\nWIFI INIT FAIL".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else if resp[4] == 0x03 {
                        let console_output = "SUCCESS\nWIFI INIT NO RESULT".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else {
                        let console_output = "SUCCESS\nWIFI INIT UNKNOWN RESULT".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    }
                } else if routine_id == 0x0210 {
                    /*GPS CHECK LOG*/
                    if resp[4] == 0xFF && resp[5] == 0xFF && resp[6] == 0xFF && resp[7] == 0xFF {
                        let console_output = "SUCCESS\nNO GPS LOG".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else {
                        let year_bytes = &[resp[4], resp[5]]; // Extract 2 bytes for year (big endian)
                        let month = resp[6]; // Extract 1 byte for month
                        let day = resp[7]; // Extract 1 byte for day
                        let hour = resp[8]; // Extract 1 byte for hour
                        let minute = resp[9]; // Extract 1 byte for minute
                        let second = resp[10]; // Extract 1 byte for second

                        // Convert bytes to u16 for year
                        let year = u16::from_be_bytes([year_bytes[0], year_bytes[1]]);

                        // Create a DateTime object
                        let datetime =
                            NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
                                .unwrap()
                                .and_hms_opt(hour as u32, minute as u32, second as u32)
                                .unwrap();

                        // Format the datetime as RFC3339 string
                        let datetime_str = datetime.to_string();
                        let console_output =
                            format!("SUCCESS\nLAST VALID GPS LOG TIMESTAMP: {}", datetime_str);
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    }
                } else if routine_id == 0x0211 {
                    /* LTE CHECK IP */
                    if resp[4] == 0x00
                        && resp[5] == 0x00
                        && resp[6] == 0x00
                        && resp[7] == 0x00
                        && resp[8] == 0xFF
                    {
                        let console_output = "SUCCESS\nNO LTE IP".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else {
                        let console_output = format!(
                            "SUCCESS\nLTE IP: {}.{}.{}.{}",
                            resp[4], resp[5], resp[6], resp[7]
                        );
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    }
                } else if routine_id == 0x0212 {
                    /* LTE CHECK PING */
                    if resp[4] == 0x01 {
                        let console_output = "SUCCESS\nPING 8.8.8.8 OK".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else if resp[4] == 0x00 {
                        let console_output = "SUCCESS\nPING 8.8.8.8 NOT OK".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else {
                        let console_output = "SUCCESS\nPING 8.8.8.8 INVALID RESULT".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    }
                } else if routine_id == 0x0213 {
                    /*LTE CHECK ENABLE SIGNAL*/
                    if resp[4] == 0x01 {
                        let console_output = "SUCCESS\nLTE ENABLED SIGNAL ON".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else if resp[4] == 0x02 {
                        let console_output = "SUCCESS\nLTE ENABLED SIGNAL OFF".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else if resp[4] == 0x03 {
                        let console_output = "SUCCESS\nLTE ENABLED SIGNAL CAN'T READ".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else {
                        let console_output =
                            "SUCCESS\nLTE ENABLED SIGNAL INVALID RESULT".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    }
                } else if routine_id == 0x0214 {
                    /*LTE GET MODEM INFO*/
                    if resp[4] == 0xFF && resp[5] == 0xFF && resp[6] == 0xFF {
                        let console_output = "SUCCESS\nNO LTE MODEM INFO".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else {
                        let state_byte = resp[4];
                        let signal_quality_bytes = [resp[5], resp[6]];
                        let has_operator = resp[7] == 0x01;

                        let state = match state_byte {
                            0x00 => "FAILED",
                            0x01 => "UNKNOWN",
                            0x02 => "INITIALIZING",
                            0x03 => "LOCKED",
                            0x04 => "DISABLED",
                            0x05 => "DISABLING",
                            0x06 => "ENABLING",
                            0x07 => "ENABLED",
                            0x08 => "SEARCHING",
                            0x09 => "REGISTERED",
                            0x0A => "DISCONNECTING",
                            0x0B => "CONNECTING",
                            0x0C => "CONNECTED",
                            // Add other variants as needed
                            _ => "INVALID STATE", // Invalid state byte
                        };

                        let signal_quality = u16::from_be_bytes(signal_quality_bytes);
                        let console_output = format!(
                            "SUCCESS\nMODEM STATE: {}\nSIGNAL QUALITY: {}\nHAS OPERATOR: {}",
                            state, signal_quality, has_operator
                        );
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    }
                } else if routine_id == 0x0215 {
                    /*BLE RESTART APP SERVICE*/
                    if resp[4] == 0x01 {
                        let console_output = "SUCCESS\nBLE ADVERTISING SUCCESS".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else if resp[4] == 0x02 {
                        let console_output = "SUCCESS\nBLE ADVERTISING FAIL".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else if resp[4] == 0x00 {
                        let console_output = "SUCCESS\nBLE ADVERTISING NO RESULT".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else {
                        let console_output = "SUCCESS\nBLE ADVERTISING INVALID RESULT".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    }
                } else if routine_id == 0x0216 {
                    /*BLE CHECK PAIR STATUS*/
                    if resp[4] == 0x01 {
                        let console_output = "SUCCESS\nBLE PAIR SUCCESS".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else if resp[4] == 0x02 {
                        let console_output = "SUCCESS\nBLE PAIR FAIL".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else if resp[4] == 0x00 {
                        let console_output = "SUCCESS\nBLE PAIR NO RESULT".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    } else {
                        let console_output = "SUCCESS\nBLE PAIR INVALID RESULT".to_string();
                        return UdsServiceResponse::Success(UdsSericeResponseDetail {
                            console_output,
                        });
                    }
                } else if routine_id == 0x0217 {
                    /* IMX CHECK SERVICE STATUS */
                    let console_output = format!(
                        "SUCCESS\nMQTT: {}\nICOM: {}\nSERVICE MANAGER: {}\nLOGGING: {}\nOTA: {}\n",
                        resp[4] == 0x01,
                        resp[5] == 0x01,
                        resp[6] == 0x01,
                        resp[7] == 0x01,
                        resp[8] == 0x01
                    );
                    return UdsServiceResponse::Success(UdsSericeResponseDetail { console_output });
                } else {
                    // Success: Need to handle other services
                    // Format the output string
                    let console_output = "SUCCESS".to_string();

                    UdsServiceResponse::Success(UdsSericeResponseDetail { console_output })
                }
            } else {
                // Fail
                // #define RoutineInProgress (0xA0)
                // #define IncorrectServiceID (0xA1)
                // #define InvalidRoutineResult (0xA2)
                if resp[2] == 0xA0 {
                    UdsServiceResponse::Fail(UdsSericeResponseDetail {
                        console_output: String::from("FAIL due to Service In Progress"),
                    })
                } else if resp[2] == 0xA1 {
                    UdsServiceResponse::Fail(UdsSericeResponseDetail {
                        console_output: String::from("FAIL due to Incorrect Service ID"),
                    })
                } else if resp[2] == 0xA2 {
                    UdsServiceResponse::Fail(UdsSericeResponseDetail {
                        console_output: String::from("FAIL due to Invalid Routine Result"),
                    })
                } else {
                    UdsServiceResponse::Fail(UdsSericeResponseDetail {
                        console_output: String::from("FAIL due to unknown error"),
                    })
                }
            }
        }
    }

    pub fn uds_routine_control_setup_get_result(&mut self) {
        if self.current_diag_mode.mode != UdsSessionType::Programming {
            self.uds_set_session_mode(UdsSessionType::Programming);
        }
        if self.current_diag_mode.sec_level == SecurityLevelAccess::None {
            let key = self.uds_security_access_request_seed(SecurityLevelAccess::Level1RequestSeed);
            self.uds_security_access_send_key(SecurityLevelAccess::Level1SendKey, &key);
        }
    }

    fn is_connectivity_service<T>(value: T, req: &mut ServiceRequest) -> bool
    where
        T: PartialEq + Into<u16>,
    {
        let value_u16: u16 = value.into();
        if value_u16 == RoutineId::WifiScan as u16 {
            req.command = 0x01;
            req.domain = Domain::Wifi;
            true
        } else if value_u16 == RoutineId::WifiCheckIp as u16 {
            req.command = 0x02;
            req.domain = Domain::Wifi;
            true
        } else if value_u16 == RoutineId::WifiRestartApp as u16 {
            req.command = 0x03;
            req.domain = Domain::Wifi;
            true
        } else if value_u16 == RoutineId::GpsCheckLog as u16 {
            req.command = 0x01;
            req.domain = Domain::Gps;
            true
        } else if value_u16 == RoutineId::LteCheckIp as u16 {
            req.command = 0x01;
            req.domain = Domain::Lte;
            true
        } else if value_u16 == RoutineId::LteCheckPing as u16 {
            req.command = 0x02;
            req.domain = Domain::Lte;
            true
        } else if value_u16 == RoutineId::LteCheckEnableSignal as u16 {
            req.command = 0x03;
            req.domain = Domain::Lte;
            true
        } else if value_u16 == RoutineId::LteGetModemInfo as u16 {
            req.command = 0x04;
            req.domain = Domain::Lte;
            true
        } else if value_u16 == RoutineId::LteGetSignalStrength as u16 {
            req.command = 0x05;
            req.domain = Domain::Lte;
            true
        } else if value_u16 == RoutineId::BleRestartApp as u16 {
            req.command = 0x01;
            req.domain = Domain::Ble;
            true
        } else if value_u16 == RoutineId::BleCheckPair as u16 {
            req.command = 0x02;
            req.domain = Domain::Ble;
            true
        } else if value_u16 == RoutineId::ImxCheckServiceStatus as u16 {
            req.command = 0x01;
            req.domain = Domain::Imx;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RoutineControlSubfcn {
    StartRoutine = 0x01,
    StopRoutine = 0x02,
    RequestRoutineResults = 0x03,
}

#[derive(Debug, Clone, Copy)]
pub enum RoutineId {
    EnableImxLte = 0x0200,
    DisableImxLte = 0x0201,
    EnableImxHmi = 0x0202,
    DisableImxHmi = 0x0203,
    SimulateInput = 0x0204,
    SwitchUsbOtgUsbHost = 0x0205,
    TriggerOutput = 0x0206,
    OpenDebugScreen = 0x0207,
    CloseDebugScreen = 0x0208,
    ToggleOffBMSVoltage = 0x0209,
    ToggleOnBMSVoltage = 0x020A,
    BikeForceUnlock = 0x020B,
    BikeForceLock = 0x020C,
    WifiScan = 0x020D,
    WifiCheckIp = 0x020E,
    WifiRestartApp = 0x020F,
    GpsCheckLog = 0x0210,
    LteCheckIp = 0x0211,
    LteCheckPing = 0x0212,
    LteCheckEnableSignal = 0x0213,
    LteGetModemInfo = 0x0214,
    BleRestartApp = 0x0215,
    BleCheckPair = 0x0216,
    ImxCheckServiceStatus = 0x0217,
    LteGetSignalStrength = 0x0218,
}

impl RoutineId {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            RoutineId::EnableImxLte
            | RoutineId::DisableImxLte
            | RoutineId::EnableImxHmi
            | RoutineId::DisableImxHmi
            | RoutineId::SimulateInput
            | RoutineId::SwitchUsbOtgUsbHost
            | RoutineId::TriggerOutput
            | RoutineId::OpenDebugScreen
            | RoutineId::CloseDebugScreen
            | RoutineId::ToggleOffBMSVoltage
            | RoutineId::ToggleOnBMSVoltage
            | RoutineId::BikeForceUnlock
            | RoutineId::BikeForceLock
            | RoutineId::WifiScan
            | RoutineId::WifiCheckIp
            | RoutineId::WifiRestartApp
            | RoutineId::GpsCheckLog
            | RoutineId::LteCheckIp
            | RoutineId::LteCheckPing
            | RoutineId::LteCheckEnableSignal
            | RoutineId::LteGetModemInfo
            | RoutineId::LteGetSignalStrength
            | RoutineId::BleRestartApp
            | RoutineId::BleCheckPair
            | RoutineId::ImxCheckServiceStatus => (*self as u16).to_be_bytes().to_vec(),
        }
    }
}

use serde::{Deserialize, Serialize};
use std::str::FromStr;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WifiScanInfo {
    pub names: Vec<String>,
    pub signal_strength: Vec<i16>,
}

impl fmt::Display for WifiScanInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Iterate over pairs of names and signal strengths
        for (name, strength) in self.names.iter().zip(self.signal_strength.iter()) {
            // Format and write each pair to the formatter
            writeln!(f, "Name: {}, Signal Strength: {}", name, strength)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Domain {
    InvalidDomain = 0x00,
    Wifi = 0x01,
    Gps = 0x02,
    Lte = 0x03,
    Ble = 0x04,
    Imx = 0x05,
}

impl FromStr for Domain {
    type Err = ();

    fn from_str(input: &str) -> Result<Domain, Self::Err> {
        match input {
            "InvalidDomain" => Ok(Domain::InvalidDomain),
            "Wifi" => Ok(Domain::Wifi),
            "Gps" => Ok(Domain::Gps),
            "Lte" => Ok(Domain::Lte),
            "Ble" => Ok(Domain::Ble),
            "Imx" => Ok(Domain::Imx),

            // Handle other cases, map to unknown instead of error out
            _ => Ok(Domain::InvalidDomain),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceResponse {
    pub domain: Domain,
    pub command: u8,
    pub response: String,
}

impl ServiceResponse {
    pub fn parse_response(&mut self) {
        match self.domain {
            Domain::Wifi => {
                if self.command == 0x01 {
                    #[allow(unused_assignments)]
                    let mut wifi_scan_info = WifiScanInfo::default();
                    if self.response != "NO SCAN RESULT" {
                        wifi_scan_info = serde_json::from_str(&self.response)
                            .expect("Failed to parse JSON into WifiScanInfo");
                        self.response = wifi_scan_info.to_string();
                    }
                    //println!("Response: {:?}", wifi_scan_info);
                }
            }
            Domain::Lte => {
                if self.command == 0x05 {
                    self.response.push_str(
                        "\n Signal Strength Map:\n rsrp >= -80.0 => Excellent\n
rsrp >= -90.0 => Good\n
rsrp >= -100.0 => Fair to poor\n
rsrp >= -120.0 => very poor signal",
                    );
                }
            }
            _ => { /* No further parsing needed */ }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceRequest {
    pub domain: Domain,
    pub command: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum SimulateInputOption {
    /// Right brake switch
    RightBrakeSwitch = 0x00,
    /// Left brake switch
    LeftBrakeSwitch = 0x01,
    /// Kill switch
    KillSwitch = 0x02,
    /// Power switch
    PowerSwitch = 0x03,
    /// Reverse switch
    ReverseSwitch = 0x04,
    /// Seat switch
    SeatSwitch = 0x05,
    /// Side stand switch
    SideStandSwitch = 0x06,
    /// Trip switch
    TripSwitch = 0x07,
    /// Ride mode switch
    RideModeSwitch = 0x08,
    /// Hazard switch
    HazardSwitch = 0x09,
    /// Horn switch
    HornSwitch = 0x0A,
    /// Right indicator switch
    RightIndicatorSwitch = 0x0B,
    /// Left indicator switch
    LeftIndicatorSwitch = 0x0C,
    /// Passing switch
    PassingSwitch = 0x0D,
    /// High beam switch
    HighBeamSwitch = 0x0E,
    /// Start switch
    StartSwitch = 0x0F,
    /// Back switch
    BackSwitch = 0x10,
    /// Select switch
    SelectSwitch = 0x11,
    /// Down switch
    DownSwitch = 0x12,
    /// Keyfob short press
    KeyfobShortPress = 0x13,
    /// Keyfob long press
    KeyfobLongPress = 0x14,
}

#[derive(Debug, Clone, Copy)]
pub enum TriggerOutputOption {
    /// HSS Rear Right Indicator
    HssRearRightIndicator = 0x00,
    /// HSS Rear Left Indicator
    HssRearLeftIndicator = 0x01,
    /// HSS Brake Light
    HssBrakeLight = 0x02,
    /// HSS Horn
    HssHorn = 0x03,
    /// HSS High Beam
    HssHighBeam = 0x04,
    /// HSS Low Beam
    HssLowBeam = 0x05,
    /// HSS License Plate
    HssLicensePlate = 0x06,
    /// HSS Front Left Indicator
    HssFrontLeftIndicator = 0x07,
    /// HSS Front Right Indicator
    HssFrontRightIndicator = 0x08,
    /// HSS Tail Light
    HssTailLight = 0x09,
    /// HSS Seat Lock
    HssSeatLock = 0x0A,
    /// HSS BMS Enable
    HssBmsEnable = 0x0B,
    /// HSS Motor Enable
    HssMotorEnable = 0x0C,
    /// HSS Steer Lock
    HssSteerLock = 0x0D,
    /// HSS DRL
    HssDrl = 0x0E,
    /// HSS Tire Pressure Monitoring System (TPMS)
    HssTpms = 0x0F,
    /// HSS Side Stand Power
    HssSideStandPower = 0x10,
}
