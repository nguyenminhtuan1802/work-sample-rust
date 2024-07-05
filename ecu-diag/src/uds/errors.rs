#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, thiserror::Error)]
pub enum UdsError {
    /// ECU rejected the request (No specific error)
    #[error("ECU rejected the request (No specific error)")]
    GeneralReject = 0x10,
    /// Service is not supported by the ECU
    #[error("Service is not supported by the ECU")]
    ServiceNotSupported = 0x11,
    /// Sub function is not supported by the ECU
    #[error("Sub function is not supported by the ECU")]
    SubFunctionNotSupported = 0x12,
    /// Request message was an invalid length, or the format of the
    /// request was incorrect
    #[error("Request message was an invalid length, or the format of the request was incorrect")]
    IncorrectMessageLengthOrInvalidFormat = 0x13,
    /// The response message is too long for the transport protocol
    #[error("The response message is too long for the transport protocol")]
    ResponseTooLong = 0x14,
    /// The ECU is too busy to perform this request. Therefore, the request
    /// Should be sent again if this error occurs
    #[error("The ECU is too busy to perform this request")]
    BusyRepeatRequest = 0x21,
    /// The requested action could not be preformed due to the prerequisite conditions
    /// not being correct
    #[error("The requested action could not be preformed due to the prerequisite conditions not being correct")]
    ConditionsNotCorrect = 0x22,
    /// The ECU cannot perform the request as the request has been sent in the incorrect order.
    /// For example, if [`SecurityOperation::SendKey`] is used before [`SecurityOperation::RequestSeed`],
    /// then the ECU will respond with this error.
    #[error(
        "The ECU cannot perform the request as the request has been sent in the incorrect order"
    )]
    RequestSequenceError = 0x24,
    /// The ECU cannot perform the request as it has timed out trying to communicate with another
    /// component within the vehicle.
    #[error("The ECU cannot perform the request as it has timed out trying to communicate with another component within the vehicle")]
    NoResponseFromSubnetComponent = 0x25,
    /// The ECU cannot perform the requested action as there is currently a DTC
    /// or failure of a component that is preventing the execution of the request.
    #[error("The ECU cannot perform the requested action as there is currently a DTC or failure of a component that is preventing the execution of the request")]
    FailurePreventsExecutionOfRequestedAction = 0x26,
    /// The request message contains data outside of a valid range
    #[error("The request message contains data outside of a valid range")]
    RequestOutOfRange = 0x31,
    /// The request could not be completed due to security access being denied.
    #[error("The request could not be completed due to security access being denied")]
    SecurityAccessDenied = 0x33,
    /// The key sent from [`SecurityOperation::SendKey`] was invalid
    #[error("The key sent from [`SecurityOperation::SendKey`] was invalid")]
    InvalidKey = 0x35,
    /// The client has tried to obtain security access to the ECU too many times with
    /// incorrect keys
    #[error("The client has tried to obtain security access to the ECU too many times with incorrect keys")]
    ExceedNumberOfAttempts = 0x36,
    /// The client has tried to request seed_key's too quickly, before the ECU timeout's period
    /// has expired
    #[error("The client has tried to request seed_key's too quickly")]
    RequiredTimeDelayNotExpired = 0x37,
    /// The ECU cannot accept the requested upload/download request due to a fault condition
    #[error(
        "The ECU cannot accept the requested upload/download request due to a fault condition"
    )]
    UploadDownloadNotAccepted = 0x70,
    /// The ECU has halted data transfer due to a fault condition
    #[error("The ECU has halted data transfer due to a fault condition")]
    TransferDataSuspended = 0x71,
    /// The ECU has encountered an error during reprogramming (erasing / flashing)
    #[error("The ECU has encountered an error during reprogramming (erasing / flashing)")]
    GeneralProgrammingFailure = 0x72,
    /// The ECU has detected the reprogramming error as the blockSequenceCounter is incorrect.
    #[error(
        "The ECU has detected the reprogramming error as the blockSequenceCounter is incorrect."
    )]
    WrongBlockSequenceCounter = 0x73,
    /// The ECU has accepted the request, but cannot reply right now. If this error occurs,
    /// the [`SecurityOperation`] will automatically stop sending tester present messages and
    /// will wait for the ECUs response. If after 2000ms, the ECU did not respond, then this error
    /// will get returned back to the function call.
    #[error("The ECU has accepted the request, but cannot reply right now")]
    RequestCorrectlyReceivedResponsePending = 0x78,
    /// The sub function is not supported in the current diagnostic session mode
    #[error("The sub function is not supported in the current diagnostic session mode")]
    SubFunctionNotSupportedInActiveSession = 0x7E,
    /// The service is not supported in the current diagnostic session mode
    #[error("The service is not supported in the current diagnostic session mode")]
    ServiceNotSupportedInActiveSession = 0x7F,
    /// Engine RPM is too high
    #[error("Engine RPM is too high")]
    RpmTooHigh = 0x81,
    /// Engine RPM is too low
    #[error("Engine RPM is too low")]
    RpmTooLow = 0x82,
    /// Engine is running
    #[error("Engine is running")]
    EngineIsRunning = 0x83,
    /// Engine is not running
    #[error("Engine is not running")]
    EngineIsNotRunning = 0x84,
    /// Engine has not been running for long enough
    #[error("Engine has not been running for long enough")]
    EngineRunTimeTooLow = 0x85,
    /// Engine temperature (coolant) is too high
    #[error("Engine temperature (coolant) is too high")]
    TemperatureTooHigh = 0x86,
    /// Engine temperature (coolant) is too low
    #[error("Engine temperature (coolant) is too low")]
    TemperatureTooLow = 0x87,
    /// Vehicle speed is too high
    #[error("Vehicle speed is too high")]
    VehicleSpeedTooHigh = 0x88,
    /// Vehicle speed is too low
    #[error("Vehicle speed is too low")]
    VehicleSpeedTooLow = 0x89,
    /// Throttle or pedal value is too high
    #[error("Throttle or pedal value is too high")]
    ThrottleTooHigh = 0x8A,
    /// Throttle or pedal value is too low
    #[error("Throttle or pedal value is too low")]
    ThrottleTooLow = 0x8B,
    /// Transmission is not in neutral
    #[error("Transmission is not in neutral")]
    TransmissionRangeNotInNeutral = 0x8C,
    /// Transmission is not in gear
    #[error("Transmission is not in gear")]
    TransmissionRangeNotInGear = 0x8D,
    /// Brake is not applied
    #[error("Brake is not applied")]
    BrakeSwitchNotClosed = 0x8F,
    /// Shifter lever is not in park
    #[error("Shifter lever is not in park")]
    ShifterLeverNotInPark = 0x90,
    /// Automatic/CVT transmission torque convert is locked
    #[error("Automatic/CVT transmission torque convert is locked")]
    TorqueConverterClutchLocked = 0x91,
    /// Voltage is too high
    #[error("Voltage is too high")]
    VoltageTooHigh = 0x92,
    /// Voltage is too low
    #[error("Voltage is too low")]
    VoltageTooLow = 0x93,
}

// Function to convert u8 to UdsError
impl UdsError {
    fn from_u8(value: u8) -> Option<UdsError> {
        match value {
            0x10..=0x93 => Some(unsafe { std::mem::transmute(value) }),
            _ => None,
        }
    }
}

pub fn process_ecu_response(sid: &[u8], r: &[u8]) {
    let positive_code = sid[0] + 0x40;
    //println!("Expected positive code: {:02X}", positive_code);
    if r[0] == positive_code {
        log::debug!("Positive code response received");
        //println!("Positive code response received");
    } else {
        match UdsError::from_u8(r[0]) {
            Some(val) => log::debug!("{}. Rejected SID: 0x{:02X}", val, sid[0]),
            None => log::debug!("Can't parse response code"),
        }
    }
}
