use super::{
    pcan_usb::{pcan_api::PCanDrvNew, pcan_types::PcanUSB},
    Hardware, HardwareInfo,
};
use crate::core::channel::{
    CanChannel, CanFrame, ChannelResult, IsoTPSettings, Packet, PacketChannel,
};
use crate::hardware::pcan_usb::PcanUsbDevice;
use crate::uds::errors::*;
use std::time::Instant;

use super::software_isotp::{IsoTpRxAction, IsoTpRxMemory};

#[allow(dead_code)]
pub struct IsoTpProtocol {
    pub connection_status: bool,
    device: PcanUsbDevice,
    channel: Box<dyn CanChannel>,
    cfg: IsoTPSettings,
}

impl Drop for IsoTpProtocol {
    fn drop(&mut self) {
        self.connection_status = false;
    }
}

impl Default for IsoTpProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl IsoTpProtocol {
    /// Creates a new Native ISOTP channel
    pub fn new() -> Self {
        let mut device = PcanUsbDevice::new(
            PcanUSB::USB1,
            HardwareInfo::default(),
            PCanDrvNew {
                is_connected: false,
            },
        )
        .unwrap();
        let channel = device.create_native_iso_tp_channel().unwrap();
        Self {
            connection_status: false,
            device,
            channel,
            cfg: IsoTPSettings::default(),
        }
    }

    pub fn set_iso_tp_cfg(&mut self, cfg: IsoTPSettings) -> ChannelResult<()> {
        self.cfg = cfg;
        Ok(())
    }

    pub fn init(&mut self) {
        match self.channel.set_can_cfg(500_000, false) {
            Ok(()) => {
                log::debug!("Success: Set Can config")
            }
            Err(e) => {
                log::error!("Error: Set CAN config {e:?}");
                //println!("Error: Set CAN config {e:?}");
            }
        }

        match self.channel.open() {
            Ok(()) => {
                log::debug!("Success: open CAN channel");
                self.connection_status = true;
            }
            Err(e) => {
                log::error!("Error: Open CAN device {e:?}");
                //println!("Error: Open can device {e:?}");
                self.connection_status = false;
            }
        }

        // default iso-tp config
        // let cfg = IsoTPSettings { block_size: 8, st_min: 0x14, extended_addresses: None, pad_frame: false, can_speed: 500_000, can_use_ext_addr: false};
        // match self.set_iso_tp_cfg(cfg) {
        //     Ok(()) => {}
        //     Err(e) => {println!("Error: Set ISO-TP config {:?}", e);}
        // }
    }

    pub fn send(&mut self, frame: CanFrame) {
        let timeout_ms = 5000;
        match self.channel.write_packets(vec![frame], timeout_ms) {
            Ok(()) => {}
            Err(e) => {
                log::error!("Error: write can {e:?}");
                //println!("Error: write can {e:?}");
            }
        }
    }

    #[allow(unused_assignments)]
    pub fn send_receive(&mut self, frame: CanFrame) -> Vec<u8> {
        let timeout_ms = 5000;
        match self.channel.write_packets(vec![frame], timeout_ms) {
            Ok(()) => {}
            Err(e) => {
                log::error!("Error: write can {e:?}");
                //println!("Error: write can {e:?}");
                self.connection_status = false;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        let mut rx_memory = IsoTpRxMemory::default();

        let default_tx_addr = frame.get_address();

        let mut empty_can_frame_count = 0;

        loop {
            match self.channel.read_packets(1, timeout_ms) {
                Ok(frames) => {
                    if frames.is_empty() {
                        empty_can_frame_count += 1;
                        if empty_can_frame_count == 800000 {
                            log::debug!("Count is 800000 exceed timeout");
                            println!("Count is 800000 exceed timeout");
                            empty_can_frame_count = 0;
                            self.connection_status = false;

                            break;
                        }
                        continue;
                    }
                    empty_can_frame_count = 0;

                    // Only use 1st frame
                    let frame = frames[0];

                    //TODO filter by receiving can id
                    let data = frame.get_data();
                    let pci_byte_idx = 0; // TODO for EXT ID Rx
                    match data.get(pci_byte_idx) {
                        Some(pci) => {
                            match pci & 0xF0 {
                                0x00 => {
                                    log::debug!("ISOTP One frame {data:02X?}");
                                    //println!("ISOTP One frame {data:02X?}");
                                    rx_memory.add_single_frame(data);
                                    break;
                                }
                                0x10 => {
                                    // Start of multi frame
                                    log::debug!("ISOTP Start frame {data:02X?}");
                                    //println!("ISOTP First frame {data:02X?}");
                                    let mut data_tx: Vec<u8> = vec![];
                                    if rx_memory.receiving {
                                        data_tx.push(0x32);
                                    } else {
                                        data_tx.push(0x30);
                                        data_tx.push(self.cfg.block_size);
                                        data_tx.push(self.cfg.st_min);
                                        rx_memory.bs = self.cfg.block_size;
                                        rx_memory.add_start_frame(data);
                                    }
                                    if self.cfg.pad_frame {
                                        data_tx.resize(8, 0xCC);
                                    }
                                    // Send flow control
                                    let f = CanFrame::new(
                                        default_tx_addr,
                                        &data_tx,
                                        self.cfg.can_use_ext_addr,
                                    );
                                    match self.channel.write_packets(vec![f], timeout_ms) {
                                        Ok(()) => {}
                                        Err(e) => {
                                            log::error!("Error: write can {e:?}");
                                            //println!("Error: write can {e:?}");
                                        }
                                    }
                                }
                                0x20 => {
                                    // Continuation of multi frame

                                    // Check separation time
                                    let real_time = (Instant::now() - rx_memory.last_rx_time)
                                        .as_millis()
                                        as u64;
                                    let st = u64::from(self.cfg.st_min);
                                    if real_time < u64::from(self.cfg.st_min) {
                                        log::error!("Separation time vilation! Clock time {real_time} is not less than min separation time {st}");
                                        println!("Separation time vilation! This could be a CAN misread error.
                                                    Try running the command again");
                                        panic!();
                                    }

                                    log::debug!("ISOTP continue frame {data:02X?}");
                                    //println!("ISOTP continue frame {data:02X?}");
                                    let status = rx_memory.add_continuous_frame(data);
                                    if status == IsoTpRxAction::SendFC {
                                        let mut data_tx: Vec<u8> = vec![];
                                        data_tx.push(0x30);
                                        data_tx.push(self.cfg.block_size);
                                        data_tx.push(self.cfg.st_min);
                                        rx_memory.bs = self.cfg.block_size;
                                        if self.cfg.pad_frame {
                                            data_tx.resize(8, 0xCC);
                                        }

                                        rx_memory.frames_received = 0; // Reset the counter

                                        let f = CanFrame::new(
                                            default_tx_addr,
                                            &data_tx,
                                            self.cfg.can_use_ext_addr,
                                        );
                                        match self.channel.write_packets(vec![f], timeout_ms) {
                                            Ok(()) => {}
                                            Err(e) => {
                                                log::error!("Error: write can {e:?}");
                                                //println!("Error: write can {e:?}");
                                            }
                                        }
                                    } else if status == IsoTpRxAction::Completed {
                                        break;
                                    }
                                }
                                0x30 => {
                                    // Flow control
                                    log::debug!("ISOTP Flow control {data:02X?}. Note: this should not happen");
                                    //println!("Received FC frame {data:02X?}. Note: this should not happen");
                                }
                                _ => {
                                    log::error!("Invalid ISOTP CAN frame! {frame:?}");
                                    //println!("Invalid ISOTP CAN frame! {frame:?}");
                                }
                            }
                        }
                        None => {
                            log::error!("ISOTP CAN frame too short! {frame:?}");
                        }
                    }
                }
                Err(e) => {
                    log::error!("Error: read can {e:?}");
                    //println!("Error: read can {e:?}");
                    self.connection_status = false;
                    break;
                }
            }
        }

        if rx_memory.completed {
            log::debug!("Received data: {}", rx_memory.format_data());
            //println!("Received data: {}", rx_memory.format_data());
            process_ecu_response(frame.clone().get_sid(), &rx_memory.data);
            return rx_memory.data;
        }

        vec![]
    }
}

#[cfg(test)]
pub mod test {
    use super::IsoTpProtocol;
    use crate::core::channel::CanFrame;

    #[test]
    pub fn test_can_isotp() {
        let mut client = IsoTpProtocol::new();
        client.init();
        let data: [u8; 8] = [0x01, 0x11, 0, 0, 0, 0, 0, 0];
        let msg = CanFrame::new(0x784, &data, false);

        client.send_receive(msg);
    }
}
