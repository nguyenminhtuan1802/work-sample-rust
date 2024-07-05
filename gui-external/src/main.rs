#[allow(clippy::all)]
mod generated_code {
    slint::include_modules!();
}
pub use generated_code::*;

slint::include_modules!();

use chrono::prelude::*;
use ecu_diag::api::UdsServiceProvider;
use ecu_diag::api::UdsServiceResponse;
use ecu_diag::uds::read_data_by_id::DataId;
use ecu_diag::uds::UDSClientSession;
use slint::{Timer, TimerMode};

fn main() {
    let ui = MainWindow::new().unwrap();

    let timer = Timer::default();
    let ui_handle = ui.as_weak().unwrap();
    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        move || {
            // Get current time
            let current_time = Local::now();

            // Format time as string
            let formatted_time = current_time.format("%H:%M:%S").to_string();
            ui_handle.invoke_tick(formatted_time.into());
        },
    );

    let mut client = UDSClientSession::new_uds_client();
    let timer1 = Timer::default();
    let ui_handle1 = ui.as_weak().unwrap();
    timer1.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(500),
        move || {
            // Get current time
            let res = client.invoke_read_data_by_id_service(DataId::Dashboard);

            match res {
                UdsServiceResponse::Success(mes) => {
                    let speed_str = mes.console_output.split(", ").find_map(|s| {
                        let mut parts = s.split(": ");
                        if let Some(key) = parts.next() {
                            if key.trim() == "Vehicle Speed" {
                                return parts.next();
                            }
                        }
                        None
                    });

                    if let Some(speed) = speed_str {
                        match speed.parse::<f32>() {
                            Ok(num) => {
                                ui_handle1.invoke_update_speed(num);
                            }
                            Err(_err) => {}
                        }
                    } else {
                        println!("Speed value not found");
                    }

                    let cpu_load_str = mes.console_output.split(", ").find_map(|s| {
                        let mut parts = s.split(": ");
                        if let Some(key) = parts.next() {
                            if key.trim() == "CPU Load" {
                                return parts.next();
                            }
                        }
                        None
                    });

                    if let Some(cpu_load) = cpu_load_str {
                        match cpu_load.parse::<i32>() {
                            Ok(num) => {
                                ui_handle1.invoke_update_cpu_load(num);
                            }
                            Err(_err) => {}
                        }
                    } else {
                        println!("CPU Load value not found");
                    }

                    let throttle_str = mes.console_output.split(", ").find_map(|s| {
                        let mut parts = s.split(": ");
                        if let Some(key) = parts.next() {
                            if key.trim() == "Throttle" {
                                return parts.next();
                            }
                        }
                        None
                    });

                    if let Some(throttle) = throttle_str {
                        match throttle.parse::<i32>() {
                            Ok(num) => {
                                ui_handle1.invoke_update_throttle(num);
                            }
                            Err(_err) => {}
                        }
                    } else {
                        println!("Throttle value not found");
                    }

                    let battery_str = mes.console_output.split(", ").find_map(|s| {
                        let mut parts = s.split(": ");
                        if let Some(key) = parts.next() {
                            if key.trim() == "Battery" {
                                return parts.next();
                            }
                        }
                        None
                    });

                    if let Some(battery) = battery_str {
                        match battery.parse::<i32>() {
                            Ok(num) => {
                                ui_handle1.invoke_update_battery(num);
                            }
                            Err(_err) => {}
                        }
                    } else {
                        println!("Battery value not found");
                    }
                }
                UdsServiceResponse::Fail(_mes) => {}
            }
        },
    );

    ui.run()
}
