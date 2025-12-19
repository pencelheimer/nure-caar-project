#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::{
    Duration, //
    Timer,
};
use esp_hal::{
    clock::CpuClock, //
    system::wakeup_cause,
    timer::timg::TimerGroup,
};

use iot_no_std::{
    hardware::SystemHardware, //
    operation_mods::info::InfoMode,
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let p = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(p.TIMG0);
    esp_rtos::start(timg0.timer0);

    let mut hw = SystemHardware::new(
        p.I2C0.into(),
        p.GPIO21.into(),
        p.GPIO22.into(),
        p.GPIO0.into(),
        p.GPIO19.into(),
        p.GPIO18.into(),
        p.LPWR,
    );

    let wakeup_cause = wakeup_cause();

    match wakeup_cause {
        esp_hal::system::SleepSource::Ext0 => {
            log::info!("Woke up from button");

            let mut button = hw.button().unwrap();

            if button.is_long_press().await {
                log::info!("Entering Setup Mode");
                Timer::after(Duration::from_millis(500)).await;
            } else {
                log::info!("Entering Info Mode");
                if let Some(mut mode) = InfoMode::new(&mut hw) {
                    mode.run().await;
                }
            }
        }
        esp_hal::system::SleepSource::Timer | _ => {
            log::info!("Entering Meassure Mode");
            if let Some(mut sensor) = hw.sensor() {
                let mut measurements = [0.0; 5];
                for i in 0..5 {
                    if let Some(dist) = sensor.measure().await {
                        measurements[i] = dist;
                        log::info!("Sample {}: {:.2} cm", i + 1, dist);
                    }
                    embassy_time::Timer::after_millis(200).await;
                }
            }
        }
    }

    // let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    // let (mut _wifi_controller, _interfaces) =
    //     esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
    //         .expect("Failed to initialize Wi-Fi controller");

    // TODO: Spawn some tasks
    let _ = spawner;

    hw.deep_sleep(Duration::from_secs(5)).await;
}
