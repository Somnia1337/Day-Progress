#![windows_subsystem = "windows"]

use chrono::{Local, Timelike};
use std::fs;
use std::str::FromStr;
use std::thread;
use tray_item::{IconSource, TrayItem};

type HM = (u32, u32);

const WAKE: HM = (8, 0);
const SLEEP: HM = (0, 0);

fn main() {
    let (wake_time, mut sleep_time) = read_configs().unwrap_or((WAKE, SLEEP));
    if sleep_time.0 * 60 + sleep_time.1 < wake_time.0 * 60 + wake_time.1 {
        sleep_time.0 += 24;
    }

    let mut tray = TrayItem::new("Day Progress", IconSource::Resource("app-icon")).unwrap();

    tray.add_label("").unwrap();
    tray.add_menu_item("Exit", || {
        std::process::exit(0);
    })
    .unwrap();

    loop {
        if !update(&mut tray, wake_time, sleep_time) {
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

fn update(tray: &mut TrayItem, wake_time: HM, sleep_time: HM) -> bool {
    let remain = calc_remain(wake_time, sleep_time);
    let tag = Box::leak(format!("num-{}", remain.0).into_boxed_str());
    let label = format!(
        "{}h {}m until {:02}:{:02}",
        remain.1, remain.2, sleep_time.0, sleep_time.1
    );

    tray.inner_mut()
        .set_icon(IconSource::Resource(tag))
        .unwrap();
    tray.inner_mut().set_label(&label, 0).unwrap();
    tray.inner_mut().set_tooltip(&label).unwrap();

    remain.0 > 0
}

fn calc_remain(wake_time: HM, sleep_time: HM) -> (u32, i64, i64) {
    let now = Local::now().with_second(0).unwrap();
    let wake = now
        .with_hour(wake_time.0)
        .unwrap()
        .with_minute(wake_time.1)
        .unwrap()
        .with_second(0)
        .unwrap();
    let mut sleep = now
        .with_hour(sleep_time.0 % 24)
        .unwrap()
        .with_minute(sleep_time.1)
        .unwrap()
        .with_second(0)
        .unwrap();

    if sleep_time.0 >= 24 {
        sleep = sleep + chrono::Duration::days(1);
    }

    let total = (sleep - wake).num_seconds() as f64;
    let elapsed = (now - wake).num_seconds() as f64;

    let percentage = if elapsed < 0.0 {
        100
    } else if elapsed >= total {
        0
    } else {
        ((total - elapsed) / total * 100.0).ceil() as u32
    };
    let hour = (sleep - now).num_hours();
    let minute = (sleep - now).num_minutes() - hour * 60;

    (percentage, hour, minute)
}

fn read_configs() -> Result<(HM, HM), Box<dyn std::error::Error>> {
    let config_path = "configs.txt";
    let content = fs::read_to_string(config_path)?;

    let mut lines = content.lines();
    let wake = lines.next().ok_or("Missing wake time in config")?;
    let sleep = lines.next().ok_or("Missing sleep time in config")?;

    let wake_hm = parse_time(wake)?;
    let sleep_hm = parse_time(sleep)?;

    Ok((wake_hm, sleep_hm))
}

fn parse_time(s: &str) -> Result<HM, Box<dyn std::error::Error>> {
    let hm: Vec<&str> = s.split(':').collect();
    if hm.len() != 2 {
        return Err("Invalid time format".into());
    }

    let h = u32::from_str(hm[0])?;
    let m = u32::from_str(hm[1])?;

    Ok((h % 24, m % 24))
}
