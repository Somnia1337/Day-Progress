fn main() {
    extern crate winres;

    let mut res = winres::WindowsResource::new();

    res.set_icon_with_id("assets/app.ico", "app-icon");

    for i in 0..=100 {
        let p = format!("assets/icons/num-{}.ico", i);
        res.set_icon_with_id(&p, &format!("num-{}", i));
    }

    res.compile().expect("Failed to compile resources");
}
