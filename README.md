# waybar-iostat

`iostat` widget for [Waybar](https://github.com/Alexays/Waybar).

<img src="https://raw.githubusercontent.com/ssc-x/waybar-iostat/master/assets/preview.gif" width="384" height="64" alt="preview" />

Developed and tested on [Hyprland](https://github.com/hyprwm/Hyprland).

# Installation

1. Run `cargo build --release`
2. Copy `target/release/libwaybar_iostat.so` wherever you want. Even the Waybar config dir is fine.
3. Add the module to your `config.jsonc`, e.g.

```jsonc
{
  // ...
  "modules-right": [
    // ...
    "cffi/iostat",
    // ...
  ],
  // ...
  "cffi/iostat": {
    "module_path": "/wherever/you/put/libwaybar_iostat.so",
    "interval": 1.5, // refresh interval in seconds - optional (default 1.0)
  },
  // ...
}
```

4. Run `killall -USR2 waybar` to restart Waybar.
5. Enjoy!

# Configuration

This widget can be styled using Waybar's `style.css`.

The base class is `.cffi-iostat`. When the disk throughput exceeds certain hardcoded thresholds, one of `.cffi-iostat-warning` or `.cffi-iostat-critical` is added as well.
