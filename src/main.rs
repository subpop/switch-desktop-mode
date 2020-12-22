extern crate clap;
extern crate dbus;
extern crate gio;

use gio::SettingsExt;

const DASHTOPANEL: &str = "dash-to-panel@jderose9.github.com";
const DASHTODOCK: &str = "dash-to-dock@micxgx.gmail.com";

fn main() {
    let app = clap::App::new("switch-desktop-mode").arg(
        clap::Arg::with_name("MODE")
            .required(true)
            .index(1)
            .possible_values(&["default", "panel", "dock"]),
    );

    let matches = app.get_matches();

    match matches.value_of("MODE").unwrap() {
        "panel" => set_mode_panel(),
        "dock" => set_mode_dock(),
        "default" | _ => reset(),
    };
}

fn reset() {
    let settings = gio::Settings::new("org.gnome.desktop.wm.preferences");
    settings.reset("button-layout");
    gio::Settings::sync();

    disable_shell_extension(DASHTOPANEL);
    disable_shell_extension(DASHTODOCK);
}

fn set_mode_panel() {
    reset();
    let settings = gio::Settings::new("org.gnome.desktop.wm.preferences");
    let _ = settings.set_string("button-layout", ":minimize,maximize,close");
    gio::Settings::sync();

    enable_shell_extension(DASHTOPANEL);
}

fn set_mode_dock() {
    reset();
    let settings = gio::Settings::new("org.gnome.desktop.wm.preferences");
    let _ = settings.set_string("button-layout", ":minimize,maximize,close");
    gio::Settings::sync();

    enable_shell_extension(DASHTODOCK);
}

fn disable_shell_extension(uuid: &str) {
    let conn = dbus::blocking::Connection::new_session().unwrap();
    let proxy = dbus::blocking::Proxy::new(
        "org.gnome.Shell",
        "/org/gnome/Shell",
        std::time::Duration::from_millis(5000),
        &conn,
    );
    let _: () = proxy
        .method_call("org.gnome.Shell.Extensions", "DisableExtension", (uuid,))
        .unwrap();
}

fn enable_shell_extension(uuid: &str) {
    let conn = dbus::blocking::Connection::new_session().unwrap();
    let proxy = dbus::blocking::Proxy::new(
        "org.gnome.Shell",
        "/org/gnome/Shell",
        std::time::Duration::from_millis(5000),
        &conn,
    );
    let _: () = proxy
        .method_call("org.gnome.Shell.Extensions", "EnableExtension", (uuid,))
        .unwrap();
}
