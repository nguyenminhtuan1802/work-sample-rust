use crate::uds::diagnostic_session_control::UdsSessionType;
use crate::uds::ecu_reset::ResetType;
use crate::uds::read_data_by_id::DataId;
use crate::uds::routine_control::ServiceRequest;
use crate::uds::routine_control::ServiceResponse;
use crate::uds::routine_control::{RoutineControlSubfcn, RoutineId};
use crate::uds::UDSClientSession;

use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use std::default::Default;

// Public types

pub enum UdsServiceResponse {
    Success(UdsSericeResponseDetail),
    Fail(UdsSericeResponseDetail),
}

pub struct UdsSericeResponseDetail {
    pub console_output: String,
}

pub enum UdsMonitorViewResponse {
    Success(UdsMonitorViewResponseDetail),
    Fail,
}

pub struct UdsMonitorViewResponseDetail {
    pub right_brake_sw: u8,
    pub left_brake_sw: u8,
    pub kill_sw: u8,
    pub power_sw: u8,
    pub reverse_sw: u8,
    pub side_stand_sw: u8,
    pub ride_mode_sw: u8,
    pub hazard_sw: u8,
    pub horn_sw: u8,
    pub right_indicator_sw: u8,
    pub left_indicator_sw: u8,
    pub high_beam_sw: u8,
    pub start_sw: u8,
    pub seat_sw: u8,
    pub trip_sw: u8,
    pub down_sw: u8,

    pub temp1: f32,
    pub temp2: f32,
    pub temp3: f32,
    pub temp4: f32,

    pub acc_x: f32,
    pub acc_y: f32,
    pub acc_z: f32,
    pub gyr_x: f32,
    pub gyr_y: f32,
    pub gyr_z: f32,

    pub rke_rssi: f32,
    pub pke_rssi: f32,
    pub throttle_pct: f32,
    pub throttle_filt: f32,

    pub cpu_118: u8,
    pub cpu_148: u8,

    pub fw_rt_major: u8,
    pub fw_rt_minor: u8,
    pub fw_tm_major: u16,
    pub fw_tm_minor: u16,

    pub dtc_syscode: u8,
    pub dtc_bmscode: u8,
    pub dtc_mccode: u8,
    pub dtc_obccode: u8,
    pub dtc_outputcode: u8,

    pub adc_12v: f32,
    pub adc_5v: f32,
    pub adc_3v: f32,

    pub bike_status: u8,
    pub bike_lock: u8,

    pub bms_status: u8,
    pub bms_predischarge_relay: u8,
    pub bms_discharge_relay: u8,
    pub bms_charging_relay: u8,
    pub bms_dcdc_enable: u8,
    pub bms_charger: u8,
    pub bms_soc_pct: u8,
    pub bms_soh_pct: u8,
    pub bms_volt: f32,
    pub bms_current: f32,
    pub bms_alive_counter: u8,
    pub bms_dcdc_enable_status: u8,
    pub bms_max_discharge_current: u16,
    pub bms_max_regen_current: u16,
    pub bms_highest_cell_volt: u16,
    pub bms_lowest_cell_volt: u16,
    pub bms_max_temp: u8,
    pub bms_max_temp_number: u8,
    pub bms_min_temp: u8,
    pub bms_min_temp_number: u8,
    pub bms_charge_discharge_cycles: u16,

    pub obc_activation_status: u8,
    pub obc_output_dc_volt: u16,
    pub obc_output_dc_current: u16,
    pub obc_max_temp: u8,
    pub obc_input_volt: u8,
    pub obc_input_current: u8,
    pub obc_stop_tx: u8,
    pub obc_alive_counter: u8,
    pub obc_error1_hw: u8,
    pub obc_error2_temp: u8,
    pub obc_error3_voltln: u8,
    pub obc_error4_current: u8,
    pub obc_error5_comn: u8,

    pub vm_persist: u8,
    pub vm_odometer: u32,
    pub vm_tripa: u32,
    pub vm_tripb: u32,
    pub vm_last_charge: u32,
    pub vm_efficiency: f32,
    pub vm_power_pct: f32,
    pub vm_speed: f32,
    pub vm_tripid: u8,
    pub vm_tripaction: u8,
    pub vm_range: u8,

    pub cm_target_charge_soc_pct: u8,
    pub cm_target_charge_hours_rem: u8,
    pub cm_target_charge_min_rem: u8,
    pub cm_target_charge_range: u16,
    pub cm_charge_complete: u8,
    pub cm_soc_limit: u8,
    pub cm_soc_limit_selection_page: u8,
    pub cm_va_limit: u16,
    pub cm_va_limit_selection_page: u8,
    pub cm_store_cable_noti: u8,

    pub rke: u8,
    pub pke: u8,
    pub pke_distance: u8,
}

impl Default for UdsMonitorViewResponseDetail {
    fn default() -> Self {
        Self {
            right_brake_sw: 0,
            left_brake_sw: 0,
            kill_sw: 0,
            power_sw: 0,

            reverse_sw: 0,
            side_stand_sw: 0,
            ride_mode_sw: 0,
            hazard_sw: 0,
            horn_sw: 0,
            right_indicator_sw: 0,
            left_indicator_sw: 0,
            high_beam_sw: 0,
            start_sw: 0,
            seat_sw: 0,
            trip_sw: 0,
            down_sw: 0,

            temp1: 0.0,
            temp2: 0.0,
            temp3: 0.0,
            temp4: 0.0,

            acc_x: 0.0,
            acc_y: 0.0,
            acc_z: 0.0,
            gyr_x: 0.0,
            gyr_y: 0.0,
            gyr_z: 0.0,

            rke_rssi: 0.0,
            pke_rssi: 0.0,
            throttle_pct: 0.0,
            throttle_filt: 0.0,

            cpu_118: 0,
            cpu_148: 0,

            fw_rt_major: 0,
            fw_rt_minor: 0,
            fw_tm_major: 0,
            fw_tm_minor: 0,

            dtc_syscode: 0,
            dtc_bmscode: 0,
            dtc_mccode: 0,
            dtc_obccode: 0,
            dtc_outputcode: 0,

            adc_12v: 0.0,
            adc_5v: 0.0,
            adc_3v: 0.0,

            bike_status: 0,
            bike_lock: 0,

            bms_status: 0,
            bms_predischarge_relay: 0,
            bms_discharge_relay: 0,
            bms_charging_relay: 0,
            bms_dcdc_enable: 0,
            bms_charger: 0,
            bms_soc_pct: 0,
            bms_soh_pct: 0,
            bms_volt: 0.0,
            bms_current: 0.0,
            bms_alive_counter: 0,
            bms_dcdc_enable_status: 0,
            bms_max_discharge_current: 0,
            bms_max_regen_current: 0,
            bms_highest_cell_volt: 0,
            bms_lowest_cell_volt: 0,
            bms_max_temp: 0,
            bms_max_temp_number: 0,
            bms_min_temp: 0,
            bms_min_temp_number: 0,
            bms_charge_discharge_cycles: 0,

            obc_activation_status: 0,
            obc_output_dc_volt: 0,
            obc_output_dc_current: 0,
            obc_max_temp: 0,
            obc_input_volt: 0,
            obc_input_current: 0,
            obc_stop_tx: 0,
            obc_alive_counter: 0,
            obc_error1_hw: 0,
            obc_error2_temp: 0,
            obc_error3_voltln: 0,
            obc_error4_current: 0,
            obc_error5_comn: 0,

            vm_persist: 0,
            vm_odometer: 0,
            vm_tripa: 0,
            vm_tripb: 0,
            vm_last_charge: 0,
            vm_efficiency: 0.0,
            vm_power_pct: 0.0,
            vm_speed: 0.0,
            vm_tripid: 0,
            vm_tripaction: 0,
            vm_range: 0,

            cm_target_charge_soc_pct: 0,
            cm_target_charge_hours_rem: 0,
            cm_target_charge_min_rem: 0,
            cm_target_charge_range: 0,
            cm_charge_complete: 0,
            cm_soc_limit: 0,
            cm_soc_limit_selection_page: 0,
            cm_va_limit: 0,
            cm_va_limit_selection_page: 0,
            cm_store_cable_noti: 0,

            rke: 0,
            pke: 0,
            pke_distance: 0,
        }
    }
}

// UDS public API
#[allow(async_fn_in_trait)]
pub trait UdsServiceProvider {
    async fn new_uds_client(
        tx: UnboundedSender<ServiceRequest>,
        rx: UnboundedReceiver<ServiceResponse>,
    ) -> UDSClientSession;
    fn invoke_read_data_by_id_service(&mut self, data_id: DataId) -> UdsServiceResponse;
    fn invoke_reset_ecu_service(&mut self, reset_mode: ResetType) -> UdsServiceResponse;
    async fn invoke_routine_control_service(
        &mut self,
        routine_subfcn: RoutineControlSubfcn,
        routine_id: RoutineId,
        routine_control_option: &[u8],
    ) -> UdsServiceResponse;
    async fn invoke_routine_control_service_get_result(
        &mut self,
        routine_subfcn: RoutineControlSubfcn,
        routine_id: RoutineId,
        routine_control_option: &[u8],
    ) -> UdsServiceResponse;
    fn invoke_set_session_mode(&mut self, session_mode: UdsSessionType) -> UdsServiceResponse;

    fn invoke_read_data_by_id_service_return_struct(
        &mut self,
        data_id: DataId,
    ) -> UdsMonitorViewResponse;
}
