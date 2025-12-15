#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{
    clock::CpuClock,
    gpio::{AnyPin, Input, InputConfig, Pin},
    peripherals::LPWR,
    rtc_cntl::{
        Rtc,
        sleep::{
            Ext0WakeupSource, //
            TimerWakeupSource,
            WakeupLevel,
        },
    },
    system::wakeup_cause,
    timer::timg::TimerGroup,
};

use iot_no_std::operation_mods;

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
    let mut p = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(p.TIMG0);
    esp_rtos::start(timg0.timer0);

    let wakeup_cause = wakeup_cause();

    match wakeup_cause {
        esp_hal::system::SleepSource::Ext0 => {
            log::info!("Woke up from button!");

            if check_long_press(Pin::degrade(p.GPIO0.reborrow())).await {
                log::info!(">>> LONG PRESS DETECTED: Entering Config Mode <<<");
                run_config_mode().await;
            } else {
                log::info!(">>> SHORT PRESS DETECTED: Show Display Info <<<");
                run_display_mode().await;
            }
        }
        esp_hal::system::SleepSource::Timer => {
            log::info!("Woke up from Timer: Measuring water level...");
            run_measurement_mode().await;
        }
        _ => {
            log::info!("Power On / Reset: Measuring water level...");
            run_measurement_mode().await;
        }
    }

    // let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    // let (mut _wifi_controller, _interfaces) =
    //     esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
    //         .expect("Failed to initialize Wi-Fi controller");

    // TODO: Spawn some tasks
    let _ = spawner;

    Timer::after(Duration::from_secs(5)).await;

    duty_cycle(
        Duration::from_secs(5),
        Pin::degrade(p.GPIO0.reborrow()),
        p.LPWR,
    );
}

fn duty_cycle(duration: Duration, pin: AnyPin, rtc_cntl: LPWR) -> () {
    let timer_source = TimerWakeupSource::new(duration.into());
    let ext_source = Ext0WakeupSource::new(pin, WakeupLevel::Low);

    Rtc::new(rtc_cntl).sleep_deep(&[&timer_source, &ext_source]);
}

async fn check_long_press<'a>(pin: AnyPin<'a>) -> bool {
    let button = Input::new(pin, InputConfig::default());
    let check_interval = Duration::from_millis(100);
    let long_press_duration = 5; // 5 * 100ms

    for _ in 0..long_press_duration {
        if button.is_high() {
            return false;
        }
        Timer::after(check_interval).await;
    }

    true
}

async fn run_config_mode() {
    Timer::after(Duration::from_millis(500)).await;
}

async fn run_display_mode() {
    Timer::after(Duration::from_millis(500)).await;
}

async fn run_measurement_mode() {
    Timer::after(Duration::from_millis(500)).await;
}
