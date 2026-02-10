mod structs;

pub(crate) use crate::util::structs::*;

use std::error::Error;
use std::{fs, path::Path};

/// Ignore mapped and loop devices so as not to overcount. We are interested in physical devices
/// only. lsblk also does something similar to determine the device types to show in its output.
const IGNORED_DEVICE_PREFIXES: [&str; 2] = ["dm-", "loop"];

/// Reads the total number of sectors read and written across all disks.
pub(crate) fn read_io_stats() -> Result<IOStats, Box<dyn Error>> {
    Ok(fs::read_dir(Path::new("/sys/block"))?
        .map(|entry| -> Result<Option<IOStats>, Box<dyn Error>> {
            let entry = entry?;

            if let Ok(device_name) = entry.file_name().into_string()
                && IGNORED_DEVICE_PREFIXES
                    .iter()
                    .any(|p| device_name.starts_with(p))
            {
                return Ok(None);
            }

            let stat_file = Path::join(&entry.path(), Path::new("stat"));
            let raw_stats = fs::read_to_string(stat_file)?;
            let stats: Vec<&str> = raw_stats.trim().split_whitespace().collect();

            let read = SectorCount(
                stats
                    .get(2)
                    .ok_or_else(|| "Couldn't get number of sectors read")?
                    .parse::<u64>()?,
            );

            let written = SectorCount(
                stats
                    .get(6)
                    .ok_or_else(|| "Couldn't get number of sectors written")?
                    .parse::<u64>()?,
            );

            Ok(Some(IOStats { read, written }))
        })
        .collect::<Result<Vec<Option<IOStats>>, Box<dyn Error>>>()?
        .into_iter()
        .filter_map(|v| v)
        .fold(
            IOStats {
                read: SectorCount(0),
                written: SectorCount(0),
            },
            |p, c| IOStats {
                read: p.read + c.read,
                written: p.written + c.written,
            },
        ))
}

pub(crate) fn format_io_stats(stats: IOStats) -> FormattedIOStats {
    let text = format!(
        "{} read {} write",
        format_value(stats.read),
        format_value(stats.written)
    );

    let max_value = stats.read.max(stats.written);

    let class = match max_value {
        m if m < SectorCount::from_mib(128) => AlertClass::NORMAL,
        m if m < SectorCount::from_mib(1_024) => AlertClass::WARNING,
        _ => AlertClass::CRITICAL,
    };

    return FormattedIOStats { text, class };
}

fn format_value(value: SectorCount) -> String {
    format!(
        "{:>10} MiB/s",
        match format!("{:04.0}", 1_000 * value.as_bytes() / 1_024 / 1_024) // 1_000 * MiB
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
        {
            Ok(value) => value
                .join(",")
                .rsplitn(2, ",")
                .collect::<Vec<&str>>()
                .into_iter()
                .rev()
                .collect::<Vec<&str>>()
                .join("."),
            Err(_) => String::from("(??)"),
        }
    )
}
