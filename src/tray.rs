use tray_item::TrayItem;

#[cfg(target_os = "linux")]
use gtk;

#[cfg(target_os = "linux")]
pub fn weather_tray(weather: String) {
    gtk::init().unwrap();

    let mut tray = TrayItem::new(&weather, "").unwrap();
    tray.add_menu_item("Exit", || {
        gtk::main_quit();
    })
    .unwrap();

    gtk::main();
}

#[cfg(target_os = "macos")]
pub fn weather_tray(weather: String) {
    let mut tray = TrayItem::new(&weather, "").unwrap();
    let inner = tray.inner_mut();
    inner.add_quit_item("Exit");
    inner.display();
}

#[cfg(target_os = "windows")]
pub fn weather_tray(weather: String) {
    let mut tray = TrayItem::new(&weather, "").unwrap();
    tray.add_label(&weather).unwrap();
    tray.add_menu_item("Exit", || {
        std::process::exit(0);
    })
    .unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
