use libloading::Library;
use std::ffi::{c_void, CStr};
use std::fmt;
use std::sync::Arc;

use super::pcan_types::{DWORD, LPSTR, WORD};
use crate::core::channel::{CanFrame, ChannelError, ChannelResult, Packet};
use crate::hardware::pcan_usb::pcan_types::PCANParameter;
use crate::hardware::{HardwareError, HardwareResult};

use super::pcan_types::{
    MsgType, PCANBaud, PCANError, PCanErrorTy, PCanResult, PcanUSB, TPCANBaudrate, TPCANBitrateFD,
    TPCANHandle, TPCANMode, TPCANParameter, TPCANStatus, TPCANType, TpCanMsg, TpCanMsgFD,
    TpCanTimestamp,
};

type GetErrorTextFn =
    unsafe extern "stdcall" fn(error: TPCANStatus, languge: WORD, buffer: LPSTR) -> TPCANStatus;

type GetStatusFn = unsafe extern "stdcall" fn(channel: TPCANHandle) -> TPCANStatus;

type InitializeFn = unsafe extern "stdcall" fn(
    channel: TPCANHandle,
    btr0btr1: TPCANBaudrate,
    hwtype: TPCANType,
    ioport: DWORD,
    interrupt: WORD,
) -> TPCANStatus;

type InitializeFdFn =
    unsafe extern "stdcall" fn(channel: TPCANHandle, bitrate: TPCANBitrateFD) -> TPCANStatus;

type LookUpChannelFn =
    unsafe extern "stdcall" fn(paramters: LPSTR, found_channel: *mut TPCANHandle) -> TPCANStatus;

type ReadFn = unsafe extern "stdcall" fn(
    channel: TPCANHandle,
    buffer: *mut TpCanMsg,
    timestamp: *mut TpCanTimestamp,
) -> TPCANStatus;

type ReadFdFn = unsafe extern "stdcall" fn(
    channel: TPCANHandle,
    buffer: *mut TpCanMsgFD,
    timestamp: *mut TpCanTimestamp,
) -> TPCANStatus;

type ResetFn = unsafe extern "stdcall" fn(channel: TPCANHandle) -> TPCANStatus;

type FilterMessagesFn = unsafe extern "stdcall" fn(
    channel: TPCANHandle,
    from_id: DWORD,
    to_id: DWORD,
    mode: TPCANMode,
) -> TPCANStatus;

type GetValueFn = unsafe extern "stdcall" fn(
    channel: TPCANHandle,
    parameter: TPCANParameter,
    buffer: *mut c_void,
    buffer_len: DWORD,
) -> TPCANStatus;

type SetValueFn = unsafe extern "stdcall" fn(
    channel: TPCANHandle,
    parameter: TPCANParameter,
    buffer: *mut c_void,
    buffer_len: DWORD,
) -> TPCANStatus;

type UninitalizeFn = unsafe extern "stdcall" fn(channel: TPCANHandle) -> TPCANStatus;

type WriteFn =
    unsafe extern "stdcall" fn(channel: TPCANHandle, buffer: *mut TpCanMsg) -> TPCANStatus;

type WriteFdFn =
    unsafe extern "stdcall" fn(channel: TPCANHandle, buffer: *mut TpCanMsgFD) -> TPCANStatus;

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Clone)]
pub struct PCanDrv {
    /// Loaded library to interface with the device
    lib: Arc<Library>,
    /// Is the device currently connected.unwrap()
    is_connected: bool,
    get_error_text_fn: GetErrorTextFn,
    get_status_fn: GetStatusFn,
    initialize_fn: InitializeFn,
    initialize_fd_fn: InitializeFdFn,
    lookup_channel_fn: LookUpChannelFn,
    read_fn: ReadFn,
    read_fd_fn: ReadFdFn,
    reset_fn: ResetFn,
    filter_messages_fn: FilterMessagesFn,
    get_value_fn: GetValueFn,
    set_value_fn: SetValueFn,
    uninitialize_fn: UninitalizeFn,
    write_fn: WriteFn,
    write_fd_fn: WriteFdFn,
}

impl fmt::Debug for PCanDrv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PassthruDrv")
            .field("is_connected", &self.is_connected)
            .field("library", &self.lib)
            .finish()
    }
}

#[allow(dead_code)]
impl PCanDrv {
    pub fn load_lib() -> HardwareResult<PCanDrv> {
        let lib = unsafe { Library::new("PCANBasic.dll").unwrap() };
        let res = unsafe {
            Self {
                get_error_text_fn: *lib
                    .get::<GetErrorTextFn>(b"CAN_GetErrorText\0")
                    .unwrap()
                    .into_raw(),
                get_status_fn: *lib
                    .get::<GetStatusFn>(b"CAN_GetStatus\0")
                    .unwrap()
                    .into_raw(),
                initialize_fn: *lib
                    .get::<InitializeFn>(b"CAN_Initialize\0")
                    .unwrap()
                    .into_raw(),
                initialize_fd_fn: *lib
                    .get::<InitializeFdFn>(b"CAN_InitializeFD\0")
                    .unwrap()
                    .into_raw(),
                lookup_channel_fn: *lib
                    .get::<LookUpChannelFn>(b"CAN_LookUpChannel\0")
                    .unwrap()
                    .into_raw(),
                read_fn: *lib.get::<ReadFn>(b"CAN_Read\0").unwrap().into_raw(),
                read_fd_fn: *lib.get::<ReadFdFn>(b"CAN_ReadFD\0").unwrap().into_raw(),
                reset_fn: *lib.get::<ResetFn>(b"CAN_Reset\0").unwrap().into_raw(),
                filter_messages_fn: *lib
                    .get::<FilterMessagesFn>(b"CAN_FilterMessages\0")
                    .unwrap()
                    .into_raw(),
                get_value_fn: *lib.get::<GetValueFn>(b"CAN_GetValue\0").unwrap().into_raw(),
                set_value_fn: *lib.get::<SetValueFn>(b"CAN_SetValue\0").unwrap().into_raw(),
                uninitialize_fn: *lib
                    .get::<UninitalizeFn>(b"CAN_Uninitialize\0")
                    .unwrap()
                    .into_raw(),
                write_fn: *lib.get::<WriteFn>(b"CAN_Write\0").unwrap().into_raw(),
                write_fd_fn: *lib.get::<WriteFdFn>(b"CAN_WriteFD\0").unwrap().into_raw(),
                lib: Arc::new(lib),
                is_connected: false,
            }
        };
        res.reset_driver().unwrap();
        Ok(res)
    }

    fn reset_driver(&self) -> HardwareResult<()> {
        log::debug!("reset_driver called");
        let res = unsafe { (self.uninitialize_fn)(0x00) };
        check_pcan_func_result((), res).map_err(|e| e.into())
    }

    pub(crate) fn reset_handle(&self, handle: PcanUSB) -> HardwareResult<()> {
        log::debug!("reset_handle called: handle: 0x{:04X}", handle.repr());
        let res = unsafe { (self.uninitialize_fn)(handle.repr()) };
        check_pcan_func_result((), res).map_err(|e| e.into())
    }

    pub(crate) fn get_device_info(&self, handle: &PcanUSB) -> HardwareResult<(String, String)> {
        log::debug!("get_device_info called: handle: 0x{:04X}", *handle as i16);
        let mut n: [u8; 33] = [0; 33];
        let mut v: [u8; 256] = [0; 256];

        check_pcan_func_result((), unsafe {
            (self.get_value_fn)(
                handle.repr(),
                PCANParameter::HardwareName.repr(),
                &mut n as *mut _ as *mut c_void,
                256,
            )
        })
        .map_err(HardwareError::from)
        .unwrap();

        check_pcan_func_result((), unsafe {
            (self.get_value_fn)(
                handle.repr(),
                PCANParameter::APIVersion.repr(),
                &mut v as *mut _ as *mut c_void,
                33,
            )
        })
        .map_err(HardwareError::from)
        .unwrap();

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

    pub(crate) fn get_path(&self) -> &'static str {
        "PCANBasic.dll"
    }

    pub(crate) fn initialize_can(&mut self, handle: PcanUSB, baud: PCANBaud) -> HardwareResult<()> {
        log::debug!(
            "initialize_can called: handle: 0x{:04X}, baud: 0x{:08X}",
            handle as i16,
            baud as i32
        );
        // Reset handle
        //let _ = self.reset_handle(handle as u16);
        log::debug!("Init CAN");
        check_pcan_func_result((), unsafe {
            (self.initialize_fn)(handle.repr(), baud.repr(), 0, 0, 0)
        })
        .map_err(HardwareError::from)
        .unwrap();

        // Configure Open filter
        let mut param: [TPCANParameter; 1] = [0x01];
        let mut p_type = PCANParameter::MessageFilter.repr();
        log::debug!("Config filter");
        check_pcan_func_result((), unsafe {
            (self.set_value_fn)(
                handle.repr(),
                p_type,
                param.as_mut_ptr() as *mut c_void,
                std::mem::size_of_val(&param) as DWORD,
            )
        })
        .map_err(HardwareError::from)
        .unwrap();

        // Configure BusOffAutoReset
        p_type = PCANParameter::BusOffAutoReset.repr();
        log::debug!("Configure BusOffAutoReset");
        check_pcan_func_result((), unsafe {
            (self.set_value_fn)(
                handle.repr(),
                p_type,
                param.as_mut_ptr() as *mut c_void,
                std::mem::size_of_val(&param) as DWORD,
            )
        })
        .map_err(HardwareError::from)
    }

    pub(crate) fn read(&mut self, handle: PcanUSB) -> ChannelResult<CanFrame> {
        log::debug!("read called: handle: 0x{:04X}", handle as i16);
        let mut can_msg = TpCanMsg {
            id: 0,
            msgtype: MsgType::Standard,
            len: 0,
            data: [0; 8],
        };
        let res = unsafe { (self.read_fn)(handle.repr(), &mut can_msg, std::ptr::null_mut()) };
        check_pcan_func_result((), res)
            .map_err(ChannelError::from)
            .unwrap();
        // Read OK!
        Ok(CanFrame::new(
            can_msg.id,
            &can_msg.data[0..can_msg.len as usize],
            can_msg.msgtype == MsgType::Extended,
        ))
    }

    pub(crate) fn write(&mut self, handle: PcanUSB, packet: CanFrame) -> ChannelResult<()> {
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
        check_pcan_func_result((), unsafe { (self.write_fn)(handle as WORD, &mut can_msg) })
            .map_err(ChannelError::from)
    }
}
