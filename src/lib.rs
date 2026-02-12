mod util;

use crate::util::{AlertClass, IOStats, format_io_stats, read_io_stats};

use async_io::Timer;
use serde::Deserialize;
use std::time::Duration;
use waybar_cffi::{
    InitInfo, Module,
    gtk::{
        CssProvider, Label,
        glib::MainContext,
        prelude::ContainerExt,
        traits::{CssProviderExt, LabelExt, StyleContextExt, WidgetExt},
    },
    waybar_module,
};

struct IOStat;

impl Module for IOStat {
    type Config = Config;

    fn init(info: &InitInfo, config: Config) -> Self {
        let interval = Duration::from_secs_f32(config.interval.unwrap_or(1.0));

        let container = info.get_root_widget();
        container.set_margin_start(4);
        container.set_margin_end(4);

        let style_context = container.style_context();
        let css = CssProvider::new();
        style_context.add_provider(&css, 0);
        css.load_from_data(
            b".cffi-iostat {
  background-color: rgba(150, 75, 0, 0.35);
}

.cffi-iostat-warning {
  background-color: rgba(175, 100, 0, 0.75);
}

.cffi-iostat-critical {
  background-color: rgba(150, 0, 0, 1);
}",
        )
        .expect("parsed CSS");
        style_context.add_class("cffi-iostat");

        let label = Label::new(Some("     0.000 MiB/s read      0.000 MiB/s write"));
        label.set_margin_start(10);
        label.set_margin_end(10);
        container.add(&label);

        MainContext::default().spawn_local(async move {
            let mut previous_stats: Option<IOStats> = None;

            loop {
                let stats = read_io_stats().expect("read stats");

                if let Some(previous_stats) = previous_stats.as_ref() {
                    let current_stats = format_io_stats(&stats - previous_stats);

                    match current_stats.class {
                        AlertClass::NORMAL => {
                            style_context.remove_class("cffi-iostat-warning");
                            style_context.remove_class("cffi-iostat-critical");
                        }
                        AlertClass::WARNING => {
                            if !style_context.has_class("cffi-iostat-warning") {
                                style_context.add_class("cffi-iostat-warning");
                            }

                            style_context.remove_class("cffi-iostat-critical");
                        }
                        AlertClass::CRITICAL => {
                            if !style_context.has_class("cffi-iostat-critical") {
                                style_context.add_class("cffi-iostat-critical");
                            }

                            style_context.remove_class("cffi-iostat-warning");
                        }
                    }

                    label.set_text(format!("{}", current_stats.text).as_str());
                }

                previous_stats = Some(stats);

                Timer::after(interval).await;
            }
        });

        IOStat
    }
}

waybar_module!(IOStat);

#[derive(Deserialize)]
struct Config {
    interval: Option<f32>,
}
