# Screen Capture Utility

This Project was developed by:
- [Cappello Alessio](https://github.com/AlessioCappello2)
- [Carena Simone](https://github.com/SimoneCarena)
- [Cavallotti Edoardo](https://github.com/edocava)

## Creating a Screenshot Utility using the Rust Programming Language

The aim of this project is to create a screen-grabbing utility capable of acquiring what is currently shown in a display, post-process it and make it available in one or more formats.
The following features are included:

1. **Platform Support**: The utility is compatible with multiple desktop operating systems, including Windows, macOS, and Linux.
2. **Selection Options**: The utility allows the user to restrict the grabbed image to a custom area selected with a click and drag motion. The selected area may be further adjusted with subsequent interactions.
3. **Hotkey Support**: The utility supports customizable hotkeys for quick screen grabbing. Users are able to set up their preferred shortcut keys.
4. **Output Format**: The utility supports multiple output formats including .png, .jpg, .gif. It also support copying the screen grab to the clipboard.
5. **Annotation Tools**: The utility has built-in annotation tools like shapes, arrows, text, and a color picker for highlighting or redacting parts of the screen grab.
6. **Delay Timer**: The utility supports a delay timer function, allowing users to set up a screen grab after a specified delay.
7. **Save Options**: The utility allows users to specify the default save location for screen grabs. It also supports automatic saving with predefined naming conventions.
8. **Multi-monitor Support**: The utility is able to recognize and handle
multiple monitors independently, allowing users to grab screens from any of the connected displays.

## Crates Depenedencies

- screenshot crate: *screenshots*
    - [crates.io](https://crates.io/crates/screenshots)
    - [github](https://github.com/nashaofu/screenshots-rs)
- gui crates: *egui*
    - [crates.io](https://crates.io/crates/egui)
    - [github](https://github.com/emilk/egui)
- hotkeys crate: *global-hotkey*
    - [crates.io](https://crates.io/crates/global-hotkey)
    - [github](https://github.com/tauri-apps/global-hotkey)
- image manipulation crates:
    - *image*
        - [crates.io](https://crates.io/crates/image)
        - [github](https://github.com/image-rs/image)
    - *imageproc*
        - [crates.io](https://crates.io/crates/imageproc)
        - [github](https://github.com/image-rs/imageproc)
- text manipulation crate: *rusttype*
    - [crates.io](https://crates.io/crates/rusttype)
    - [gitlab](https://gitlab.redox-os.org/redox-os/rusttype)
- error definition crate: *thiserror*
    - [crates.io](https://crates.io/crates/thiserror)
    - [github](https://github.com/dtolnay/thiserror)
- home directory utility crate: *home*
    - [crates.io](https://crates.io/crates/home)
- clipboard crate: *arboard*
    - [crates.io](https://crates.io/crates/arboard)
    - [github](https://github.com/1Password/arboard)

```{toml}
arboard = "3.2.0"
egui = "0.22.0"
global-hotkey = "0.2.3"
home = "0.5.5"
image = "0.24.6"
imageproc = "0.23.0"
rusttype = "0.9.3"
screenshots = "0.7.0"
thiserror = "1.0.44"
```
