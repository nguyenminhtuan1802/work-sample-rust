use super::pcan_types::{
    MsgType, PCANBaud, PCANError, PCanErrorTy, PCanResult, PcanUSB, TPCANBaudrate, TPCANHandle,
    TPCANMode, TPCANParameter, TPCANStatus, TPCANType, TpCanMsg, TpCanTimestamp,
};
use super::pcan_types::{DWORD, LPSTR, WORD};
use crate::core::channel::{CanFrame, ChannelError, ChannelResult, Packet};
use crate::hardware::pcan_usb::pcan_types::PCANParameter;
use crate::hardware::{HardwareError, HardwareResult};
use std::ffi::{c_void, CStr};
use std::fmt;

#[cfg(windows)]
#[allow(dead_code)]
#[link(name = "PCANBasic")]
extern "C" {
    fn CAN_Initialize(
        channel: TPCANHandle,
        baudrate: TPCANBaudrate,
        hwType: TPCANType,
        ioPort: u32,
        interrupt: u16,
    ) -> TPCANStatus;
    fn CAN_Uninitialize(channel: TPCANHandle) -> TPCANStatus;
    fn CAN_Reset(channel: TPCANHandle) -> TPCANStatus;
    fn CAN_GetStatus(channel: TPCANHandle) -> TPCANStatus;

    fn CAN_Read(
        channel: TPCANHandle,
        msgBuffer: *mut TpCanMsg,
        timeStmpBuffer: *mut TpCanTimestamp,
    ) -> TPCANStatus;
    fn CAN_Write(channel: TPCANHandle, msgBuffer: *mut TpCanMsg) -> TPCANStatus;

    fn CAN_FilterMessages(
        channel: TPCANHandle,
        from_id: DWORD,
        to_id: DWORD,
        mode: TPCANMode,
    ) -> TPCANStatus;
    fn CAN_SetValue(
        channel: TPCANHandle,
        parameter: TPCANParameter,
        buffer: *mut c_void,
        buffer_len: DWORD,
    ) -> TPCANStatus;
    fn CAN_GetValue(
        channel: TPCANHandle,
        parameter: TPCANParameter,
        buffer: *mut c_void,
        buffer_len: DWORD,
    ) -> TPCANStatus;
    fn CAN_LookUpChannel(paramters: LPSTR, found_channel: *mut TPCANHandle) -> TPCANStatus;
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
#[link(name = "pcanbasic")]
extern "C" {
    fn CAN_Initialize(
        channel: TPCANHandle,
        baudrate: TPCANBaudrate,
        hwType: TPCANType,
        ioPort: u32,
        interrupt: u16,
    ) -> TPCANStatus;
    fn CAN_Uninitialize(channel: TPCANHandle) -> TPCANStatus;
    fn CAN_Reset(channel: TPCANHandle) -> TPCANStatus;
    fn CAN_GetStatus(channel: TPCANHandle) -> TPCANStatus;

    fn CAN_Read(
        channel: TPCANHandle,
        msgBuffer: *mut TpCanMsg,
        timeStmpBuffer: *mut TpCanTimestamp,
    ) -> TPCANStatus;
    fn CAN_Write(channel: TPCANHandle, msgBuffer: *mut TpCanMsg) -> TPCANStatus;

    fn CAN_FilterMessages(
        channel: TPCANHandle,
        from_id: DWORD,
        to_id: DWORD,
        mode: TPCANMode,
    ) -> TPCANStatus;
    fn CAN_SetValue(
        channel: TPCANHandle,
        parameter: TPCANParameter,
        buffer: *mut c_void,
        buffer_len: DWORD,
    ) -> TPCANStatus;
    fn CAN_GetValue(
        channel: TPCANHandle,
        parameter: TPCANParameter,
        buffer: *mut c_void,
        buffer_len: DWORD,
    ) -> TPCANStatus;
    fn CAN_LookUpChannel(paramters: LPSTR, found_channel: *mut TPCANHandle) -> TPCANStatus;
}

fn check_pcan_func_result<T>(ret: T, status: TPCANStatus) -> PCanResult<T> {
    match status {
        0 => PCanResult::Ok(ret),
        x => {
            if let Some(r) = PCANError::from_repr(x) {
                PCanResult::Err(PCanErrorTy::StandardError(r))
            } else {
                PCanResult::Err(PCanErrorTy::Unknown(x))
            }
        }
    }
}

#[derive(Clone)]
pub struct PCanDrvNew {
    /// Is the device currently connected?
    pub is_connected: bool,
}

impl fmt::Debug for PCanDrvNew {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PCan_Drv")
            .field("is_connected", &self.is_connected)
            .finish()
    }
}

impl PCanDrvNew {
    pub fn reset_driver(&self) -> HardwareResult<()> {
        // log::debug!("reset_driver called");
        // let res = unsafe { CAN_Uninitialize(0x00) };
        // check_pcan_func_result((), res).map_err(|e| e.into())
        Ok(())
    }

    pub fn reset_handle(&self, handle: PcanUSB) -> HardwareResult<()> {
        log::debug!("reset_handle called: handle: 0x{:04X}", handle.repr());
        let res = unsafe { CAN_Uninitialize(handle.repr()) };
        check_pcan_func_result((), res).map_err(|e| e.into())
    }

    pub fn get_device_info(&self, handle: &PcanUSB) -> HardwareResult<(String, String)> {
        log::debug!("get_device_info called: handle: 0x{:04X}", *handle as i16);
        let mut n: [u8; 33] = [0; 33];
        let mut v: [u8; 256] = [0; 256];

        check_pcan_func_result((), unsafe {
            CAN_GetValue(
                handle.repr(),
                PCANParameter::HardwareName.repr(),
                &mut n as *mut _ as *mut c_void,
                256,
            )
        })
        .map_err(HardwareError::from)?;

        check_pcan_func_result((), unsafe {
            CAN_GetValue(
                handle.repr(),
                PCANParameter::APIVersion.repr(),
                &mut v as *mut _ as *mut c_void,
                33,
            )
        })
        .map_err(HardwareError::from)?;

        let name = CStr::from_bytes_until_nul(&n)
            .unwrap()
            .to_string_lossy()
            .to_string();
        let version = CStr::from_bytes_until_nul(&v)
            .unwrap()
            .to_string_lossy()
            .to_string();
        Ok((name, version))
    }

    pub fn initialize_can(&mut self, handle: PcanUSB, baud: PCANBaud) -> HardwareResult<()> {
        log::debug!(
            "initialize_can called: handle: 0x{:04X}, baud: 0x{:08X}",
            handle as i16,
            baud as i32
        );
        // Reset handle
        //let _ = self.reset_handle(handle as u16);
        log::debug!("Init CAN");
        check_pcan_func_result((), unsafe {
            CAN_Initialize(handle.repr(), baud.repr(), 0, 0, 0)
        })
        .map_err(HardwareError::from)?;

        // Configure Open filter
        let mut param: [TPCANParameter; 1] = [0x01];
        let mut p_type = PCANParameter::MessageFilter.repr();
        log::debug!("Config filter");
        check_pcan_func_result((), unsafe {
            CAN_SetValue(
                handle.repr(),
                p_type,
                param.as_mut_ptr() as *mut c_void,
                std::mem::size_of_val(&param) as DWORD,
            )
        })
        .map_err(HardwareError::from)?;

        // Configure BusOffAutoReset
        p_type = PCANParameter::BusOffAutoReset.repr();
        log::debug!("Configure BusOffAutoReset");
        check_pcan_func_result((), unsafe {
            CAN_SetValue(
                handle.repr(),
                p_type,
                param.as_mut_ptr() as *mut c_void,
                std::mem::size_of_val(&param) as DWORD,
            )
        })
        .map_err(HardwareError::from)
    }

    pub fn read(&mut self, handle: PcanUSB) -> ChannelResult<CanFrame> {
        //log::debug!("read called: handle: 0x{:04X}", handle as i16);
        let mut can_msg = TpCanMsg {
            id: 0,
            msgtype: MsgType::Standard,
            len: 0,
            data: [0; 8],
        };
        let res = unsafe { CAN_Read(handle.repr(), &mut can_msg, std::ptr::null_mut()) };
        check_pcan_func_result((), res).map_err(ChannelError::from)?;
        // Read OK!
        Ok(CanFrame::new(
            can_msg.id,
            &can_msg.data[0..can_msg.len as usize],
            can_msg.msgtype == MsgType::Extended,
        ))
    }

    pub fn write(&mut self, handle: PcanUSB, packet: CanFrame) -> ChannelResult<()> {
        log::debug!(
            "write called: handle: 0x{:04X}, addr: 0x{:08X}, content: {:02X?}",
            handle as i16,
            packet.get_address(),
            packet.get_data()
        );
        let mut can_msg = TpCanMsg {
            id: packet.get_address(),
            msgtype: if packet.is_extended() {
                MsgType::Extended
            } else {
                MsgType::Standard
            },
            len: packet.get_data().len() as u8,
            data: [0; 8],
        };
        let l = packet.get_data().len();
        can_msg.data[0..l].copy_from_slice(packet.get_data());
        let status = unsafe { CAN_Write(handle as WORD, &mut can_msg) };
        check_pcan_func_result((), status).map_err(ChannelError::from)
    }

    pub fn get_path(&self) -> &'static str {
        "PCANBasic.dll"
    }

    pub fn load_lib() -> HardwareResult<PCanDrvNew> {
        let res = PCanDrvNew {
            is_connected: false,
        };
        res.reset_driver().unwrap();
        Ok(res)
    }
}

#[cfg(test)]
pub mod test {
    use super::super::pcan_types::{PCANBaud, PcanUSB};
    use super::PCanDrvNew;
    use crate::core::channel::CanFrame;

    #[test]
    pub fn test_pcan_api() {
        let mut canApi = PCanDrvNew { is_connected: true };
        let handle = PcanUSB::USB1;
        let baud = PCANBaud::Can500Kbps;
        canApi.initialize_can(handle, baud);

        let data: [u8; 8] = [0x01, 0x11, 0, 0, 0, 0, 0, 0];
        let msg = CanFrame::new(0x784, &data, false);
        canApi.write(handle, msg);
        canApi.reset_handle(handle);
    }

    #[test]
    pub fn test_pcan_api_extended() {
        let mut canApi = PCanDrvNew { is_connected: true };
        let handle = PcanUSB::USB1;
        let baud = PCANBaud::Can500Kbps;
        canApi.initialize_can(handle, baud);

        let data: [u8; 8] = [0x02, 0x3E, 0, 0, 0, 0, 0, 0];
        let msg = CanFrame::new(0x784, &data, true);
        canApi.write(handle, msg);
        canApi.reset_handle(handle);
    }
}
