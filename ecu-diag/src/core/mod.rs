use crate::core::channel::ChannelError;
use crate::hardware::HardwareError;
use std::sync::Arc;

pub(crate) mod channel;
pub(crate) mod dtc;
pub(crate) mod dynamic_diag;

/// Diagnostic server result
pub type DiagServerResult<T> = Result<T, DiagError>;

#[allow(dead_code)]
#[derive(Clone, Debug, thiserror::Error)]
/// Diagnostic server error
pub enum DiagError {
    /// The Diagnostic server does not support the request
    #[error("Diagnostic server does not support the request")]
    NotSupported,
    /// Diagnostic error code from the ECU itself
    #[error("ECU Negative response. Error 0x{:02X?}, definition: {:?}", code, def)]
    ECUError {
        /// Raw Negative response code from ECU
        code: u8,
        /// Negative response code definition according to protocol
        def: Option<String>,
    },
    /// Response empty
    #[error("ECU did not respond to the request")]
    EmptyResponse,
    /// ECU Responded but send a message that wasn't a reply for the sent message
    #[error("ECU response is out of order")]
    WrongMessage,
    /// Diagnostic server terminated!?
    #[error("Diagnostic server was terminated before the request")]
    ServerNotRunning,
    /// ECU Responded with a message, but the length was incorrect
    #[error("ECU response size was not the correct length")]
    InvalidResponseLength,
    /// A parameter given to the function is invalid. Check the function's documentation
    /// for more information
    #[error("Diagnostic function parameter invalid")]
    ParameterInvalid,
    /// Error with underlying communication channel
    #[error("Diagnostic server hardware channel error")]
    ChannelError(
        #[from]
        #[source]
        ChannelError,
    ),
    /// Device hardware error
    #[error("Diagnostic server hardware error")]
    HardwareError(
        #[from]
        #[source]
        Arc<HardwareError>,
    ),
    /// Feauture is not iumplemented yet
    #[error("Diagnostic server feature is unimplemented: '{0}'")]
    NotImplemented(String),
    /// Mismatched PID response ID
    #[error(
        "Requested Ident 0x{:04X?}, but received ident 0x{:04X?}",
        want,
        received
    )]
    MismatchedIdentResponse {
        /// Requested PID
        want: u16,
        /// Received PID from ECU
        received: u16,
    },
}

#[allow(dead_code)]
/// Converts a single byte into a BCD string
pub fn bcd_decode(input: u8) -> String {
    format!("{}{}", (input & 0xF0) >> 4, input & 0x0F)
}

#[allow(dead_code)]
/// Converts a slice to a BCD string
pub fn bcd_decode_slice(input: &[u8], sep: Option<&str>) -> String {
    let mut res = String::new();
    for (pos, x) in input.iter().enumerate() {
        res.push_str(bcd_decode(*x).as_str());
        if let Some(separator) = sep {
            if pos != input.len() - 1 {
                res.push_str(separator)
            }
        }
    }
    res
}
