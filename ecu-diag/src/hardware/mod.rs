//! The hardware module contains simplified API's and abstraction layers
//! for interacting with common hardware that can be used for either Bench setups or OBD2 adapters
//! in order to communicate with vehicle ECUs

pub mod isotp;
pub mod pcan_usb;
pub mod software_isotp;
pub mod tcp;

use crate::core::channel::{CanChannel, IsoTPChannel};

/// Hardware API result
pub type HardwareResult<T> = Result<T, HardwareError>;

/// The hardware trait defines functions supported by all adapter types,
/// as well as functions that can create abstracted communication channels
/// that can be used in diagnostic servers
pub trait Hardware: Clone {
    /// Creates an ISO-TP channel on the devices.
    /// This channel will live for as long as the hardware trait. Upon being dropped,
    /// the channel will automatically be closed, if it has been opened.
    ///
    /// `force_native` - Force the use the protocol ISO-TP channel if the hardware supports it.
    ///                  If the channel does not support native ISOTP, but this is set to true,
    ///                  then UnsupportedChannel error will be raised.
    fn create_iso_tp_channel(
        &mut self,
        force_native: bool,
    ) -> HardwareResult<Box<dyn IsoTPChannel>>;

    fn create_native_iso_tp_channel(&mut self) -> HardwareResult<dyn CanChannel>>;

    /// Creates a CAN Channel on the devices.
    /// This channel will live for as long as the hardware trait. Upon being dropped,
    /// the channel will automatically be closed, if it has been opened.
    fn create_can_channel(&mut self) -> HardwareResult<Box<dyn CanChannel>>;

    /// Returns true if the ISO-TP channel is current open and in use
    fn is_iso_tp_channel_open(&self) -> bool;

    /// Returns true if the CAN channel is currently open and in use
    fn is_can_channel_open(&self) -> bool;

    /// Tries to read battery voltage from Pin 16 of an OBD port (+12V).
    /// This is mainly used by diagnostic adapters, and is purely optional
    /// Should the adapter not support this feature, [std::option::Option::None] is returned
    fn read_battery_voltage(&mut self) -> Option<f32>;

    /// Tries to read battery voltage from the igntion pin on the OBD2 port. A reading
    /// would indicate ignition is on in the vehicle.
    /// This is mainly used by diagnostic adapters, and is purely optional
    /// Should the adapter not support this feature, [std::option::Option::None] is returned
    fn read_ignition_voltage(&mut self) -> Option<f32>;

    /// Returns the information of the hardware
    fn get_info(&self) -> &HardwareInfo;

    /// Returns if the hardware is currently connected
    fn is_connected(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Device hardware info used by [HardwareScanner]
#[derive(Default)]
pub struct HardwareInfo {
    /// Name of the hardware
    pub name: String,
    /// Optional vendor of the hardware
    pub vendor: Option<String>,
    /// Optional version of the firmware running on the adapter / device
    pub device_fw_version: Option<String>,
    /// Optional API standard the device conforms to
    pub api_version: Option<String>,
    /// Optional library (Dll/So/Dynlib) version used
    pub library_version: Option<String>,
    /// Optional file location of the library used
    pub library_location: Option<String>,
    /// Listed capabilities of the hardware
    pub capabilities: HardwareCapabilities,
}

/// Trait for scanning hardware on a system which can be used
/// to diagnose ECUs
pub trait HardwareScanner<T: Hardware> {
    /// Lists all scanned devices. This does not necessarily
    /// mean that the hardware can be used, just that the system
    /// known it exists.
    fn list_devices(&self) -> Vec<HardwareInfo>;
    /// Tries to open a device by a specific index from the [HardwareScanner::list_devices] function.
    fn open_device_by_index(&self, idx: usize) -> HardwareResult<T>;
    /// Tries to open a device given the devices name
    fn open_device_by_name(&self, name: &str) -> HardwareResult<T>;
}

#[derive(Clone, Debug, thiserror::Error)]
/// Represents error that can be returned by Hardware API
pub enum HardwareError {
    /// Low level device driver error
    #[error("Device library API error. Code {code}, Description: '{desc}'")]
    APIError {
        /// API Error code
        code: u32,
        /// API Error description
        desc: String,
    },
    /// Indicates that a conflicting channel type was opened on a device which does not
    /// support multiple channels of the same underlying network to be open at once.
    #[error("Channel type conflicts with an already open channel")]
    ConflictingChannel,
    /// Indicates a channel type is not supported by the API
    #[error("Channel type not supported on this hardware")]
    ChannelNotSupported,
    /// Hardware not found
    #[error("Device not found")]
    DeviceNotFound,
    /// Function called on device that has not yet been opened
    #[error("Device was not opened")]
    DeviceNotOpen,

    /// Lib loading error
    #[cfg(feature = "passthru")]
    #[error("Device API library load error")]
    LibLoadError(
        #[from]
        #[source]
        Arc<libloading::Error>,
    ),
}

#[cfg(feature = "passthru")]
impl From<libloading::Error> for HardwareError {
    fn from(err: libloading::Error) -> Self {
        Self::LibLoadError(Arc::new(err))
    }
}

#[cfg(all(feature = "pcan-usb", windows))]
impl From<PCanErrorTy> for HardwareError {
    fn from(value: PCanErrorTy) -> Self {
        match value {
            PCanErrorTy::StandardError(ty) => match ty {
                _ => Self::APIError {
                    code: ty as u32,
                    desc: ty.to_string(),
                }
                .into(),
            },
            PCanErrorTy::Unknown(e) => Self::APIError {
                code: e,
                desc: value.to_string(),
            }
            .into(),
        }
    }
}

/// ISOTP layer type
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IsoTpChannelType {
    /// ISO-TP is NOT supported at all by the hardware
    None,
    /// ISO-TP is emulated via a CAN channel in the ECU-diagnostics crate
    Emulated,
    /// ISO-TP is supported by the hardware and protocol (Natively)
    Protocol,
}

/// Contains details about what communication protocols
/// are supported by the physical hardware
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HardwareCapabilities {
    /// Supports ISO-TP
    pub iso_tp: IsoTpChannelType,
    /// Supports CANBUS
    pub can: bool,
    /// Supports standard Kline OBD (ISO9141)
    pub kline: bool,
    /// Supports KWP2000 over Kline (ISO14230)
    pub kline_kwp: bool,
    /// Supports J1850 VPW and J180 PWM
    pub sae_j1850: bool,
    /// Supports Chryslers serial communication interface
    pub sci: bool,
    /// Supports IP protocols (Diagnostic Over IP)
    pub ip: bool,
}

impl Default for HardwareCapabilities {
    fn default() -> Self {
        Self {
            iso_tp: IsoTpChannelType::None,
            can: false,
            kline: false,
            kline_kwp: false,
            sae_j1850: false,
            sci: false,
            ip: false,
        }
    }
}

/// Return is (Mask, filter)
#[allow(dead_code)]
pub(crate) fn ids_to_filter_mask(ids: &[u32], use_ext_can: bool) -> (u32, u32) {
    if ids.is_empty() {
        return (0, 0); // Allow anything filter
    }
    let mut m: u32 = ids[0]; // Mask
    let mut f: u32 = ids[0]; // Filter

    for id in ids {
        f &= id;
        m |= id;
    }
    m ^= f;

    if use_ext_can {
        m ^= 0x1FFFFFFF;
    } else {
        m ^= 0x7FF;
    }
    (m, f)
}

#[cfg(test)]
pub mod hardware_tests {
    use super::ids_to_filter_mask;

    #[test]
    pub fn test_filter_mask_gen() {
        let ids = [0x1E0, 0x1E1, 0x1E9, 0x7E0];
        let (m, f) = ids_to_filter_mask(&ids, false);
        for id in ids {
            assert_eq!(m & id, f)
        }
    }

    use std::{
        sync::{mpsc, Arc},
        time::Instant,
    };

    use crate::{
        core::channel::{
            CanChannel, CanFrame, ChannelError, IsoTPChannel, IsoTPSettings, PacketChannel,
            PayloadChannel,
        },
        hardware::software_isotp::SoftwareIsoTpChannel,
    };

    pub struct EmuCanChannel {
        name: &'static str,
        in_queue: Arc<mpsc::Receiver<CanFrame>>,
        out_queue: mpsc::Sender<CanFrame>,
    }

    unsafe impl Send for EmuCanChannel {}
    unsafe impl Sync for EmuCanChannel {}

    impl EmuCanChannel {
        pub fn new(
            sender: mpsc::Sender<CanFrame>,
            receiver: mpsc::Receiver<CanFrame>,
            name: &'static str,
        ) -> Self {
            Self {
                name,
                in_queue: Arc::new(receiver),
                out_queue: sender,
            }
        }
    }

    impl CanChannel for EmuCanChannel {
        fn set_can_cfg(
            &mut self,
            _baud: u32,
            _use_extended: bool,
        ) -> crate::core::channel::ChannelResult<()> {
            Ok(())
        }
    }

    impl PacketChannel<CanFrame> for EmuCanChannel {
        fn open(&mut self) -> crate::core::channel::ChannelResult<()> {
            Ok(())
        }

        fn close(&mut self) -> crate::core::channel::ChannelResult<()> {
            Ok(())
        }

        fn write_packets(
            &mut self,
            packets: Vec<CanFrame>,
            _timeout_ms: u32,
        ) -> crate::core::channel::ChannelResult<()> {
            for p in packets {
                log::debug!("{} Out -> {p:02X?}", self.name);
                self.out_queue.send(p).unwrap();
            }
            Ok(())
        }

        fn read_packets(
            &mut self,
            max: usize,
            timeout_ms: u32,
        ) -> crate::core::channel::ChannelResult<Vec<CanFrame>> {
            let mut read_packets = vec![];
            let start = Instant::now();
            loop {
                let res = self
                    .in_queue
                    .try_recv()
                    .map_err(|_| ChannelError::BufferEmpty);
                match res {
                    Ok(f) => {
                        log::debug!("{} In  -> {f:02X?}", self.name);
                        read_packets.push(f);
                    }
                    Err(ChannelError::BufferEmpty) => return Ok(read_packets),
                    Err(e) => return Err(e),
                }
                if read_packets.len() == max {
                    return Ok(read_packets);
                }
                if timeout_ms != 0 && start.elapsed().as_millis() > timeout_ms as u128 {
                    return if read_packets.is_empty() {
                        Err(ChannelError::BufferEmpty)
                    } else {
                        Ok(read_packets)
                    };
                }
            }
        }

        fn clear_rx_buffer(&mut self) -> crate::core::channel::ChannelResult<()> {
            while self.in_queue.try_recv().is_ok() {}
            Ok(())
        }

        fn clear_tx_buffer(&mut self) -> crate::core::channel::ChannelResult<()> {
            Ok(())
        }
    }

    fn setup(
        bs: u8,
        stmin: u8,
        padding: bool,
        ext_address: Option<(u8, u8)>,
        ecu1_addr: u32,
        ecu2_addr: u32,
    ) -> (SoftwareIsoTpChannel, SoftwareIsoTpChannel) {
        let (ecu1tx, ecu1rx) = mpsc::channel::<CanFrame>();
        let (ecu2tx, ecu2rx) = mpsc::channel::<CanFrame>();
        let ecu1 = Box::new(EmuCanChannel::new(ecu2tx, ecu1rx, "Tester"));
        let ecu2 = Box::new(EmuCanChannel::new(ecu1tx, ecu2rx, "ECU"));

        let mut iso_tp1 = SoftwareIsoTpChannel::new(ecu1);
        let mut iso_tp2 = SoftwareIsoTpChannel::new(ecu2);

        iso_tp1.set_iso_tp_cfg(IsoTPSettings {
            block_size: bs,
            st_min: stmin,
            extended_addresses: ext_address,
            pad_frame: padding,
            can_speed: 500_000,
            can_use_ext_addr: false,
        });

        iso_tp2.set_iso_tp_cfg(IsoTPSettings {
            block_size: bs,
            st_min: stmin,
            extended_addresses: ext_address,
            pad_frame: padding,
            can_speed: 500_000,
            can_use_ext_addr: false,
        });

        iso_tp1.set_ids(ecu1_addr, ecu2_addr);
        iso_tp2.set_ids(ecu2_addr, ecu1_addr);

        PayloadChannel::open(&mut iso_tp1);
        PayloadChannel::open(&mut iso_tp2);

        (iso_tp1, iso_tp2)
    }

    #[test]
    fn test_single_frame() {
        env_logger::try_init();
        let TX_BYTES = &[0x01, 0x02, 0x03, 0x04, 0x05];

        let (mut ch1, mut ch2) = setup(8, 20, true, None, 0x07E1, 0x07E9);

        ch1.write_bytes(0x07E1, None, TX_BYTES, 0)
            .expect("Write failed!");

        let r = ch2.read_bytes(1000);
        assert!(r.is_ok());
        assert_eq!(TX_BYTES.to_vec(), r.unwrap());
    }

    #[test]
    fn test_multi_frame() {
        env_logger::try_init();
        let TX_BYTES = (0..0xFF).collect::<Vec<u8>>();

        let (mut ch1, mut ch2) = setup(8, 20, true, None, 0x07E1, 0x07E9);

        ch1.write_bytes(0x07E1, None, &TX_BYTES, 5000)
            .expect("Write failed!");

        let mut r = ch2.read_bytes(5000);

        assert!(r.is_ok());
        assert_eq!(TX_BYTES.to_vec(), r.unwrap());

        let TX2_BYTES = (0x00..0xFF).rev().collect::<Vec<u8>>();

        ch1.write_bytes(0x07E1, None, &TX2_BYTES, 5000)
            .expect("Write failed!");

        r = ch2.read_bytes(5000);

        assert!(r.is_ok());
        assert_eq!(TX2_BYTES.to_vec(), r.unwrap());
    }
}
