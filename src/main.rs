#![windows_subsystem = "windows"]

use chrono::{Local, Timelike};
use std::thread;
use std::time::Duration;
use tray_item::{IconSource, TrayItem};

fn main() {
    let mut tray = TrayItem::new("Day Progress", IconSource::Resource("app-icon")).unwrap();

    tray.add_label("").unwrap();
    tray.add_menu_item("Exit", || {
        std::process::exit(0);
    })
    .unwrap();

    let wake = (7, 0);
    let sleep = (23, 0);

    loop {
        let remain = calculate_remaining_time_percentage(wake, sleep);
        let tag = Box::leak(format!("num-{}", remain).into_boxed_str());
        let label = format!("{}%", remain);

        tray.inner_mut()
            .set_icon(IconSource::Resource(tag))
            .unwrap();
        tray.inner_mut().set_label(&label, 0).unwrap();
        tray.inner_mut().set_tooltip(&label).unwrap();

        thread::sleep(Duration::from_secs(60));
    }
}

fn calculate_remaining_time_percentage(wake: (u32, u32), sleep: (u32, u32)) -> u32 {
    let now = Local::now();
    let wake_up_time = now
        .with_hour(wake.0)
        .unwrap()
        .with_minute(wake.1)
        .unwrap()
        .with_second(0)
        .unwrap();
    let sleep_time = now
        .with_hour(sleep.0)
        .unwrap()
        .with_minute(sleep.1)
        .unwrap()
        .with_second(0)
        .unwrap();

    let total = (sleep_time - wake_up_time).num_seconds() as f64;
    let elapsed = (now - wake_up_time).num_seconds() as f64;

    if elapsed < 0.0 {
        100
    } else if elapsed >= total {
        0
    } else {
        ((total - elapsed) / total * 100.0).ceil() as u32
    }
}
