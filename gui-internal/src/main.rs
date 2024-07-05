//slint::slint!(import { AppUi } from "src/app.slint";);

mod uds_client;

mod generated_code {
    slint::include_modules!();
}
use ecu_diag::api::{UdsMonitorViewResponse, UdsServiceResponse};
pub use generated_code::*;

use crate::uds_client::{UdsMessage, UdsWorker};
pub use ecu_diag::uds::diagnostic_session_control::UdsSessionType;
use slint::{SharedString, Timer, TimerMode};

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use ecu_diag::hardware::tcp::TcpProtocol;
use ecu_diag::uds::routine_control::{ServiceRequest, ServiceResponse};

use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let app = AppUi::new().unwrap();
    app.on_open_url(|url| {
        open::that(url.as_str()).ok();
    });
    app.set_cargo_ui_version(env!("CARGO_PKG_VERSION").into());

    let (stop_tx_uds_worker, stop_rx_uds_worker) = oneshot::channel::<()>();
    let (stop_tx_monitor, stop_rx_monitor) = oneshot::channel::<()>();
    let (tx_res, rx_res) = tokio::sync::mpsc::unbounded_channel::<ServiceResponse>();
    let (tx_req, rx_req) = tokio::sync::mpsc::unbounded_channel::<ServiceRequest>();
    let worker = UdsWorker::new(&app, rx_res, tx_req, stop_rx_uds_worker, stop_rx_monitor).await;

    let tcp_connection = TcpProtocol::new().await;

    let ui_handle_tcp_listener = app.as_weak();
    let ui_handle_service_listener = app.as_weak();

    let (stop_tx_tcp, stop_rx_tcp) = oneshot::channel::<()>();
    let (stop_tx_service, stop_rx_service) = oneshot::channel::<()>();

    match tcp_connection {
        Some(tcp) => {
            app.set_tcp_connection_state(1);
            app.set_tcp_connection_state1(1);
            let _tcp_listener = tokio::spawn({
                let reader_clone = Arc::clone(&tcp.reader);
                async move {
                    uds_client::start_tcp_listener(
                        reader_clone,
                        tx_res.clone(),
                        ui_handle_tcp_listener,
                        stop_rx_tcp,
                    )
                    .await;
                }
            });

            let _service_listener = tokio::spawn({
                let writer_clone = Arc::clone(&tcp.writer);
                async move {
                    uds_client::start_service_listener(
                        writer_clone,
                        rx_req,
                        ui_handle_service_listener,
                        stop_rx_service,
                    )
                    .await;
                }
            });
        }
        None => {
            app.set_tcp_connection_state(2);
            app.set_tcp_connection_state1(2);
        }
    }

    let ui_handle_action_cancel = app.as_weak();
    app.on_action_cancel({
        move || {
            ui_handle_action_cancel.unwrap().set_is_streaming(false);
        }
    });

    let timer = Timer::default();
    let ui_handle_timer = app.as_weak();
    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        {
            let send_channel = worker.channel.clone();
            move || {
                if ui_handle_timer.unwrap().get_is_streaming() {
                    let action = ui_handle_timer.unwrap().get_streaming_action();
                    send_channel.send(UdsMessage::Action { action }).unwrap();
                }
            }
        },
    );

    let ui_handle_on_action_stream = app.as_weak();
    app.on_action_stream({
        //ui_handle.unwrap().set_service_output(String::from("Running Service...").into());
        move |action| {
            if ui_handle_on_action_stream.unwrap().get_is_streaming() {
                ui_handle_on_action_stream.unwrap().set_is_streaming(false);
            } else {
                ui_handle_on_action_stream.unwrap().set_is_streaming(true);
                ui_handle_on_action_stream
                    .unwrap()
                    .set_streaming_action(action);
            }
        }
    });

    let ui_handle_on_action_submit = app.as_weak();
    app.on_action_submit({
        let send_channel = worker.channel.clone();
        //ui_handle.unwrap().set_service_output(String::from("Running Service...").into());
        move |action| {
            if ui_handle_on_action_submit.unwrap().get_is_streaming() {
                ui_handle_on_action_submit.unwrap().set_is_streaming(false);
            }

            let action_clone = action.clone();
            if action_clone.service == "Connectivity" {
                ui_handle_on_action_submit
                    .unwrap()
                    .set_current_running_service(action_clone.option1);
            }

            thread::sleep(Duration::from_millis(100));
            send_channel.send(UdsMessage::Action { action }).unwrap();
        }
    });

    app.on_action_get_result({
        let send_channel = worker.channel.clone();
        move |service| {
            let action = Action {
                service: SharedString::from("Connectivity"),
                option1: SharedString::from(format!("{} Get Result", service)),
                option2: SharedString::from(""),
                option3: SharedString::from(""),
            };

            thread::sleep(Duration::from_millis(100));
            send_channel.send(UdsMessage::Action { action }).unwrap();
        }
    });

    let ui_handle1 = app.as_weak();
    let timer1 = Timer::default();

    #[allow(unused_assignments)]
    timer1.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        move || {
            let uds_client_clone = Arc::clone(&worker.uds_client);
            let ui_handle1_clone = ui_handle1.clone();
            tokio::spawn(async move {
                let mut mode = 0;

                //println!("Lock uds client from timer");
                let mut uds_client = uds_client_clone.lock().await;
                //println!("Unlock uds client from timer");

                match uds_client.current_diag_mode.mode {
                    UdsSessionType::Default => {
                        mode = 1;
                    }
                    UdsSessionType::Programming => {
                        mode = 2;
                    }
                    UdsSessionType::Extended => {
                        mode = 2;
                    }
                    UdsSessionType::SafetySystem => {
                        mode = 2;
                    }
                    UdsSessionType::StreamMode => {
                        mode = 3;
                    }
                    UdsSessionType::Invalid => {
                        mode = 4;
                    }
                }

                //let uds_client_clone_clone = Arc::clone(&uds_client_clone);
                let can_status = uds_client.check_can_connection_status();

                // if can_status {
                //     println!("can connected");
                // } else {
                //     println!("can disconnected");
                // }

                // println!("diag mode is : {}", mode);

                let _ = slint::invoke_from_event_loop(move || {
                    ui_handle1_clone
                        .unwrap()
                        .set_diagnostics_session_state(mode);

                    if can_status {
                        ui_handle1_clone.unwrap().set_can_connection_state(1);
                        ui_handle1_clone.unwrap().set_can_connection_state1(1);
                    } else {
                        ui_handle1_clone.unwrap().set_can_connection_state(2);
                        ui_handle1_clone.unwrap().set_can_connection_state1(2);
                    }
                });
            });
        },
    );

    let ui_handle2 = app.as_weak();
    let timer2 = Timer::default();
    timer2.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        {
            let mut receive_channel = worker.receive_channel;
            move || {
                if let Ok(res) = receive_channel.try_recv() {
                    match res {
                        UdsServiceResponse::Success(success) => {
                            // log::debug!("Sending OK, awaiting response from ECU");
                            println!("Received data: {:?}", success.console_output);
                            ui_handle2
                                .unwrap()
                                .set_service_output(success.console_output.into());
                        }
                        UdsServiceResponse::Fail(fail) => {
                            // log::debug!("Sending OK, awaiting response from ECU");
                            println!("Received data: {:?}", fail.console_output);
                            ui_handle2
                                .unwrap()
                                .set_service_output(fail.console_output.into());
                        }
                    }
                }
            }
        },
    );

    let ui_handle_timer_monitor_view_get_result = app.as_weak();
    let monitor_view_timer_get_result = Timer::default();
    monitor_view_timer_get_result.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        {
            let mut receive_channel = worker.receive_channel_monitor_view;
            move || {
                if let Ok(res) = receive_channel.try_recv() {
                    match res {
                        UdsMonitorViewResponse::Success(success) => {
                            // log::debug!("Sending OK, awaiting response from ECU");
                            //println!("Received data: {:?}", success);
                            let app = ui_handle_timer_monitor_view_get_result.unwrap();

                            app.set_monitor_view_temp1(success.temp1.to_string().into());
                            app.set_monitor_view_temp2(success.temp2.to_string().into());
                            app.set_monitor_view_temp3(success.temp3.to_string().into());
                            app.set_monitor_view_temp4(success.temp4.to_string().into());

                            app.set_monitor_view_acc_x(success.acc_x.to_string().into());
                            app.set_monitor_view_acc_y(success.acc_y.to_string().into());
                            app.set_monitor_view_acc_z(success.acc_z.to_string().into());
                            app.set_monitor_view_gyr_x(success.gyr_x.to_string().into());
                            app.set_monitor_view_gyr_y(success.gyr_y.to_string().into());
                            app.set_monitor_view_gyr_z(success.gyr_z.to_string().into());

                            app.set_monitor_view_rke_rssi(success.rke_rssi.to_string().into());
                            app.set_monitor_view_pke_rssi(success.pke_rssi.to_string().into());
                            app.set_monitor_view_throttle_pct(
                                success.throttle_pct.to_string().into(),
                            );
                            app.set_monitor_view_throttle_filt(
                                success.throttle_filt.to_string().into(),
                            );

                            app.set_monitor_view_fw_tm_major(success.fw_tm_major.to_string().into());
                            app.set_monitor_view_fw_tm_minor(success.fw_tm_minor.to_string().into());
                            app.set_monitor_view_fw_rt_major(success.fw_rt_major.to_string().into());
                            app.set_monitor_view_fw_rt_minor(success.fw_rt_minor.to_string().into());

                            app.set_monitor_view_dtc_syscode(
                                success.dtc_syscode.to_string().into(),
                            );
                            app.set_monitor_view_dtc_bmscode(
                                success.dtc_bmscode.to_string().into(),
                            );
                            app.set_monitor_view_dtc_mccode(success.dtc_mccode.to_string().into());
                            app.set_monitor_view_dtc_obccode(
                                success.dtc_obccode.to_string().into(),
                            );
                            app.set_monitor_view_dtc_outputcode(
                                success.dtc_outputcode.to_string().into(),
                            );

                            app.set_monitor_view_adc_12v(success.adc_12v.to_string().into());
                            app.set_monitor_view_adc_5v(success.adc_5v.to_string().into());
                            app.set_monitor_view_adc_3v(success.adc_3v.to_string().into());

                            app.set_monitor_view_bike_status(
                                success.bike_status.to_string().into(),
                            );
                            app.set_monitor_view_bike_lock(success.bike_lock.to_string().into());

                            app.set_monitor_view_bms_status(success.bms_status.to_string().into());
                            app.set_monitor_view_bms_predischarge_relay(
                                success.bms_predischarge_relay.to_string().into(),
                            );
                            app.set_monitor_view_bms_discharge_relay(
                                success.bms_discharge_relay.to_string().into(),
                            );
                            app.set_monitor_view_bms_charging_relay(
                                success.bms_charging_relay.to_string().into(),
                            );
                            app.set_monitor_view_bms_dcdc_enable(
                                success.bms_dcdc_enable.to_string().into(),
                            );
                            app.set_monitor_view_bms_charger(
                                success.bms_charger.to_string().into(),
                            );
                            app.set_monitor_view_bms_soc_pct(
                                success.bms_soc_pct.to_string().into(),
                            );
                            app.set_monitor_view_bms_soh_pct(
                                success.bms_soh_pct.to_string().into(),
                            );
                            app.set_monitor_view_bms_volt(success.bms_volt.to_string().into());
                            app.set_monitor_view_bms_current(
                                success.bms_current.to_string().into(),
                            );
                            app.set_monitor_view_bms_alive_counter(
                                success.bms_alive_counter.to_string().into(),
                            );
                            app.set_monitor_view_bms_dcdc_enable_status(
                                success.bms_dcdc_enable_status.to_string().into(),
                            );
                            app.set_monitor_view_bms_max_discharge_current(
                                success.bms_max_discharge_current.to_string().into(),
                            );
                            app.set_monitor_view_bms_max_regen_current(
                                success.bms_max_regen_current.to_string().into(),
                            );
                            app.set_monitor_view_bms_highest_cell_volt(
                                success.bms_highest_cell_volt.to_string().into(),
                            );
                            app.set_monitor_view_bms_lowest_cell_volt(
                                success.bms_lowest_cell_volt.to_string().into(),
                            );
                            app.set_monitor_view_bms_max_temp(
                                success.bms_max_temp.to_string().into(),
                            );
                            app.set_monitor_view_bms_max_temp_number(
                                success.bms_max_temp_number.to_string().into(),
                            );
                            app.set_monitor_view_bms_min_temp(
                                success.bms_min_temp.to_string().into(),
                            );
                            app.set_monitor_view_bms_min_temp_number(
                                success.bms_min_temp_number.to_string().into(),
                            );
                            app.set_monitor_view_bms_charge_discharge_cycles(
                                success.bms_charge_discharge_cycles.to_string().into(),
                            );

                            app.set_monitor_view_obc_activation_status(
                                success.obc_activation_status.to_string().into(),
                            );
                            app.set_monitor_view_obc_output_dc_volt(
                                success.obc_output_dc_volt.to_string().into(),
                            );
                            app.set_monitor_view_obc_output_dc_current(
                                success.obc_output_dc_current.to_string().into(),
                            );
                            app.set_monitor_view_obc_max_temp(
                                success.obc_max_temp.to_string().into(),
                            );
                            app.set_monitor_view_obc_input_volt(
                                success.obc_input_volt.to_string().into(),
                            );
                            app.set_monitor_view_obc_input_current(
                                success.obc_input_current.to_string().into(),
                            );
                            app.set_monitor_view_obc_stop_tx(
                                success.obc_stop_tx.to_string().into(),
                            );
                            app.set_monitor_view_obc_alive_counter(
                                success.obc_alive_counter.to_string().into(),
                            );
                            app.set_monitor_view_obc_error1_hw(
                                success.obc_error1_hw.to_string().into(),
                            );
                            app.set_monitor_view_obc_error2_temp(
                                success.obc_error2_temp.to_string().into(),
                            );
                            app.set_monitor_view_obc_error3_voltln(
                                success.obc_error3_voltln.to_string().into(),
                            );
                            app.set_monitor_view_obc_error4_current(
                                success.obc_error4_current.to_string().into(),
                            );
                            app.set_monitor_view_obc_error5_comn(
                                success.obc_error5_comn.to_string().into(),
                            );

                            app.set_monitor_view_vm_persist(success.vm_persist.to_string().into());
                            app.set_monitor_view_vm_odometer(
                                success.vm_odometer.to_string().into(),
                            );
                            app.set_monitor_view_vm_tripa(success.vm_tripa.to_string().into());
                            app.set_monitor_view_vm_tripb(success.vm_tripb.to_string().into());
                            app.set_monitor_view_vm_last_charge(
                                success.vm_last_charge.to_string().into(),
                            );
                            app.set_monitor_view_vm_efficiency(
                                success.vm_efficiency.to_string().into(),
                            );
                            app.set_monitor_view_vm_power_pct(
                                success.vm_power_pct.to_string().into(),
                            );
                            app.set_monitor_view_vm_speed(success.vm_speed.to_string().into());
                            app.set_monitor_view_vm_tripid(success.vm_tripid.to_string().into());
                            app.set_monitor_view_vm_tripaction(
                                success.vm_tripaction.to_string().into(),
                            );
                            app.set_monitor_view_vm_range(success.vm_range.to_string().into());

                            app.set_monitor_view_cm_target_charge_soc_pct(
                                success.cm_target_charge_soc_pct.to_string().into(),
                            );
                            app.set_monitor_view_cm_target_charge_hours_rem(
                                success.cm_target_charge_hours_rem.to_string().into(),
                            );
                            app.set_monitor_view_cm_cm_target_charge_min_rem(
                                success.cm_target_charge_min_rem.to_string().into(),
                            );
                            app.set_monitor_view_cm_cm_target_charge_range(
                                success.cm_target_charge_range.to_string().into(),
                            );
                            app.set_monitor_view_cm_charge_complete(
                                success.cm_charge_complete.to_string().into(),
                            );
                            app.set_monitor_view_cm_soc_limit(
                                success.cm_soc_limit.to_string().into(),
                            );
                            app.set_monitor_view_cm_soc_limit_selection_page(
                                success.cm_soc_limit_selection_page.to_string().into(),
                            );
                            app.set_monitor_view_cm_va_limit(
                                success.cm_va_limit.to_string().into(),
                            );
                            app.set_monitor_view_cm_va_limit_selection_page(
                                success.cm_va_limit_selection_page.to_string().into(),
                            );
                            app.set_monitor_view_cm_store_cable_noti(
                                success.cm_store_cable_noti.to_string().into(),
                            );

                            app.set_monitor_view_rke(success.rke.to_string().into());
                            app.set_monitor_view_pke(success.pke.to_string().into());
                            app.set_monitor_view_pke_distance(
                                success.pke_distance.to_string().into(),
                            );

                            app.set_monitor_view_cpu_118(success.cpu_118.to_string().into());
                            app.set_monitor_view_cpu_148(success.cpu_148.to_string().into());

                            app.set_right_brake(success.right_brake_sw as i32);
                            app.set_left_brake(success.left_brake_sw as i32);
                            app.set_kill_sw(success.kill_sw as i32);
                            app.set_power_sw(success.power_sw as i32);
                            app.set_reverse_sw(success.reverse_sw as i32);
                            app.set_side_stand_sw(success.side_stand_sw as i32);
                            app.set_ride_mode_sw(success.ride_mode_sw as i32);
                            app.set_hazard_sw(success.hazard_sw as i32);
                            app.set_horn_sw(success.horn_sw as i32);
                            app.set_right_indicator_sw(success.right_indicator_sw as i32);
                            app.set_left_indicator_sw(success.left_indicator_sw as i32);
                            app.set_high_beam_sw(success.high_beam_sw as i32);
                            app.set_start_sw(success.start_sw as i32);
                            app.set_seat_sw(success.seat_sw as i32);
                            app.set_trip_sw(success.trip_sw as i32);
                            app.set_down_sw(success.down_sw as i32);
                        }
                        UdsMonitorViewResponse::Fail => {}
                    }
                }
            }
        },
    );

    let monitor_view_timer = Timer::default();
    let ui_handle_timer_monitor_view = app.as_weak();
    monitor_view_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(2000),
        {
            let send_channel = worker.channel_monitor_view.clone();
            move || {
                if ui_handle_timer_monitor_view
                    .unwrap()
                    .get_monitor_view_is_streaming()
                {
                    let action = Action {
                        service: SharedString::from("Read Monitor View"),
                        option1: SharedString::from(""),
                        option2: SharedString::from(""),
                        option3: SharedString::from(""),
                    };

                    thread::sleep(Duration::from_millis(100));
                    send_channel.send(UdsMessage::Action { action }).unwrap();
                }
            }
        },
    );

    let ui_handle_on_monitor_view_action_start = app.as_weak();
    app.on_monitor_view_action_start({
        move || {
            if ui_handle_on_monitor_view_action_start
                .unwrap()
                .get_monitor_view_is_streaming()
            {
                ui_handle_on_monitor_view_action_start
                    .unwrap()
                    .set_monitor_view_is_streaming(false);
            } else {
                ui_handle_on_monitor_view_action_start
                    .unwrap()
                    .set_monitor_view_is_streaming(true);
            }
        }
    });

    app.run().unwrap();

    let _ = stop_tx_tcp.send(());
    let _ = stop_tx_service.send(());
    let _ = stop_tx_uds_worker.send(());
    let _ = stop_tx_monitor.send(());
}
