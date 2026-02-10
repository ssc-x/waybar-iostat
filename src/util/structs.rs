use std::ops::{Add, Sub};

/// 1 UNIX sector = 512 bytes.
/// https://www.kernel.org/doc/html/latest/block/stat.html
const UNIX_SECTOR_SIZE: u64 = 512;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct SectorCount(pub(crate) u64);

pub(crate) struct IOStats {
    pub(super) read: SectorCount,
    pub(super) written: SectorCount,
}

#[derive(Debug)]
pub(crate) enum AlertClass {
    NORMAL,
    WARNING,
    CRITICAL,
}

#[derive(Debug)]
pub(crate) struct FormattedIOStats {
    pub(crate) text: String,
    pub(crate) class: AlertClass,
}

impl SectorCount {
    pub(crate) fn as_bytes(self) -> u64 {
        self.0 * UNIX_SECTOR_SIZE
    }

    pub(crate) fn from_mib(value: u64) -> Self {
        SectorCount((value << 20) / UNIX_SECTOR_SIZE)
    }
}

impl Add for SectorCount {
    type Output = Self;

    fn add(self, rhs: SectorCount) -> Self::Output {
        SectorCount(self.0 + rhs.0)
    }
}

impl Sub for &IOStats {
    type Output = IOStats;

    fn sub(self, rhs: &IOStats) -> Self::Output {
        IOStats {
            read: SectorCount(self.read.0 - rhs.read.0),
            written: SectorCount(self.written.0 - rhs.written.0),
        }
    }
}
