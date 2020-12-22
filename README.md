switch-desktop-mode is a simple little program to set some GSettings and GNOME
Shell extensions.

## Installation

```
cargo install
```

## Usage

```
switch-desktop-mode MODE
```

## Modes

### `default`

* Resets the titlebar buttons to the default value.
* Disables dash-to-dock
* Disables dash-to-panel

### `panel`

* Sets titlebar buttons to include minimize and maximize
* Enables dash-to-panel

### `dock`

* Sets titlebar buttons to include minimize and maximize
* Enables dash-to-dock
