extern crate clap;
extern crate dbus;
extern crate gio;

use gio::SettingsExt;

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

    let conn = dbus::blocking::Connection::new_session().unwrap();
    let proxy = dbus::blocking::Proxy::new(
        "org.gnome.Shell",
        "/org/gnome/Shell",
        std::time::Duration::from_millis(5000),
        &conn,
    );
    let _: () = proxy
        .method_call(
            "org.gnome.Shell.Extensions",
            "DisableExtension",
            ("dash-to-panel@jderose9.github.com",),
        )
        .unwrap();
    let _: () = proxy
        .method_call(
            "org.gnome.Shell.Extensions",
            "DisableExtension",
            ("dash-to-dock@micxgx.gmail.com",),
        )
        .unwrap();
}

fn set_mode_panel() {
    reset();
    let settings = gio::Settings::new("org.gnome.desktop.wm.preferences");
    let _ = settings.set_string("button-layout", "menu:minimize,maximize,close");
    gio::Settings::sync();
    let conn = dbus::blocking::Connection::new_session().unwrap();
    let proxy = dbus::blocking::Proxy::new(
        "org.gnome.Shell",
        "/org/gnome/Shell",
        std::time::Duration::from_millis(5000),
        &conn,
    );
    let _: () = proxy
        .method_call(
            "org.gnome.Shell.Extensions",
            "EnableExtension",
            ("dash-to-panel@jderose9.github.com",),
        )
        .unwrap();
}

fn set_mode_dock() {
    reset();
    let settings = gio::Settings::new("org.gnome.desktop.wm.preferences");
    let _ = settings.set_string("button-layout", "menu:minimize,maximize,close");
    gio::Settings::sync();
    let conn = dbus::blocking::Connection::new_session().unwrap();
    let proxy = dbus::blocking::Proxy::new(
        "org.gnome.Shell",
        "/org/gnome/Shell",
        std::time::Duration::from_millis(5000),
        &conn,
    );
    let _: () = proxy
        .method_call(
            "org.gnome.Shell.Extensions",
            "EnableExtension",
            ("dash-to-dock@micxgx.gmail.com",),
        )
        .unwrap();
}
