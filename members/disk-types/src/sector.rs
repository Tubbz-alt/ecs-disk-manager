use crate::DeviceExt;
use std::str::FromStr;

/// Trait for getting and sectors from a device.
pub trait SectorExt: DeviceExt {
    /// Calculates the requested sector from a given `Sector` variant.
    fn get_sector(&self, sector: Sector) -> u64 {
        const MIB2: u64 = 2 * 1024 * 1024;

        let end = || self.sectors() - (MIB2 / self.logical_sector_size());
        let megabyte = |size| (size * 1_000_000) / self.logical_sector_size();

        match sector {
            Sector::Start => MIB2 / self.logical_sector_size(),
            Sector::End => end(),
            Sector::Megabyte(size) => megabyte(size),
            Sector::MegabyteFromEnd(size) => end() - megabyte(size),
            Sector::Unit(size) => size,
            Sector::UnitFromEnd(size) => end() - size,
            Sector::Percent(value) => {
                if value == ::std::u16::MAX {
                    self.sectors()
                } else {
                    ((self.sectors() * self.logical_sector_size()) / ::std::u16::MAX as u64)
                        * value as u64
                        / self.logical_sector_size()
                }
            }
        }
    }
}

/// Used with the `Disk::get_sector` method for converting a more human-readable unit
/// into the corresponding sector for the given disk.
#[derive(Debug, PartialEq, Clone, Copy, Hash)]
pub enum Sector {
    /// The first sector in the disk where partitions should be created.
    Start,
    /// The last sector in the disk where partitions should be created.
    End,
    /// A raw value that directly corrects to the exact number of sectors that
    /// will be used.
    Unit(u64),
    /// Similar to the above, but subtracting from the end.
    UnitFromEnd(u64),
    /// Rather than specifying the sector count, the user can specify the actual size in megabytes.
    /// This value will later be used to get the exact sector count based on the sector size.
    Megabyte(u64),
    /// Similar to the above, but subtracting from the end.
    MegabyteFromEnd(u64),
    /// The percent can be represented by specifying a value between 0 and
    /// u16::MAX, where u16::MAX is 100%.
    Percent(u16),
}

impl From<u64> for Sector {
    fn from(sectors: u64) -> Sector { Sector::Unit(sectors) }
}

impl FromStr for Sector {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.ends_with('M') {
            if input.starts_with('-') {
                if let Ok(value) = input[1..input.len() - 1].parse::<u64>() {
                    return Ok(Sector::MegabyteFromEnd(value));
                }
            } else if let Ok(value) = input[..input.len() - 1].parse::<u64>() {
                return Ok(Sector::Megabyte(value));
            }
        } else if input.ends_with('%') {
            if let Ok(value) = input[..input.len() - 1].parse::<u16>() {
                if value <= 100 {
                    return Ok(Sector::Percent(value));
                }
            }
        } else if input == "start" {
            return Ok(Sector::Start);
        } else if input == "end" {
            return Ok(Sector::End);
        } else if input.starts_with('-') {
            if let Ok(value) = input[1..input.len()].parse::<u64>() {
                return Ok(Sector::UnitFromEnd(value));
            }
        } else if let Ok(value) = input[..input.len()].parse::<u64>() {
            return Ok(Sector::Unit(value));
        }

        Err("invalid sector value")
    }
}
