#![windows_subsystem = "windows"]

use chrono::{Local, Timelike};
use std::thread;
use tray_item::{IconSource, TrayItem};

const WAKE: (u32, u32) = (7, 0);
const SLEEP: (u32, u32) = (23, 0);

fn main() {
    let mut tray = TrayItem::new("Day Progress", IconSource::Resource("app-icon")).unwrap();

    tray.add_label("").unwrap();
    tray.add_menu_item("Exit", || {
        std::process::exit(0);
    })
    .unwrap();

    loop {
        if !update(&mut tray) {
            break;
        }

        let now = Local::now();
        let next_min = now.with_second(0).unwrap() + chrono::Duration::minutes(1);
        let thread_sleep = (next_min - now).to_std().unwrap();
        thread::sleep(thread_sleep);
    }

    let overtime = "Go to bed NOW";
    tray.inner_mut()
        .set_icon(IconSource::Resource("app-icon"))
        .unwrap();
    tray.inner_mut().set_label(overtime, 0).unwrap();
    tray.inner_mut().set_tooltip(overtime).unwrap();

    thread::park();
}

fn update(tray: &mut TrayItem) -> bool {
    let remain = calc_remain();
    let tag = Box::leak(format!("num-{}", remain.0).into_boxed_str());
    let label = format!(
        "{}h {}m until {:02}:{:02}",
        remain.1, remain.2, SLEEP.0, SLEEP.1
    );

    tray.inner_mut()
        .set_icon(IconSource::Resource(tag))
        .unwrap();
    tray.inner_mut().set_label(&label, 0).unwrap();
    tray.inner_mut().set_tooltip(&label).unwrap();

    remain.0 > 0
}

fn calc_remain() -> (u32, i64, i64) {
    let now = Local::now().with_second(0).unwrap();
    let wake_up_time = now
        .with_hour(WAKE.0)
        .unwrap()
        .with_minute(WAKE.1)
        .unwrap()
        .with_second(0)
        .unwrap();
    let sleep_time = now
        .with_hour(SLEEP.0)
        .unwrap()
        .with_minute(SLEEP.1)
        .unwrap()
        .with_second(0)
        .unwrap();

    let total = (sleep_time - wake_up_time).num_seconds() as f64;
    let elapsed = (now - wake_up_time).num_seconds() as f64;

    let percentage = if elapsed < 0.0 {
        100
    } else if elapsed >= total {
        0
    } else {
        ((total - elapsed) / total * 100.0).ceil() as u32
    };
    let hour = (sleep_time - now).num_hours();
    let minute = (sleep_time - now).num_minutes() - hour * 60;

    (percentage, hour, minute)
}
