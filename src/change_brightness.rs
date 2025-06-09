//! Modify a device's brightness

use std::str::FromStr;

use brightness::blocking::{Brightness as _, BrightnessDevice};

/// What to do with the brightness
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum ChangeBrightness {
    /// Increase brightness by an absolute percentage
    ///
    /// e.g.: `+10%`
    Increase(u32),
    /// Decrease brightness by an absolute percentage
    ///
    /// e.g.: `-10%`
    Decrease(u32),
    /// Set the brightness to a specific percentage
    ///
    /// e.g.: `50%`
    Set(u32),
}

impl ChangeBrightness {
    /// Change brightness of the given device
    pub fn change_brightness_of_device(
        self,
        device: &BrightnessDevice,
    ) -> Result<(), brightness::Error> {
        match self {
            Self::Increase(amount) => {
                let current = device.get().unwrap_or(0);
                device.set(current + amount)?;
            }
            Self::Decrease(amount) => {
                let current = device.get().unwrap_or(0);
                device.set(current.saturating_sub(amount))?;
            }
            Self::Set(amount) => {
                device.set(amount)?;
            }
        }

        Ok(())
    }
}

impl FromStr for ChangeBrightness {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "max" => Self::Set(100),
            "min" => Self::Set(0),
            _ => {
                let without_percent = s
                    .strip_suffix("%")
                    .ok_or_else(|| format!("expected percentage like `10%`, but found: {s}"))?;

                without_percent
                    .strip_prefix("-")
                    .map(|p| p.parse::<u32>().map(ChangeBrightness::Decrease))
                    .or_else(|| {
                        without_percent
                            .strip_prefix("+")
                            .map(|p| p.parse::<u32>().map(ChangeBrightness::Increase))
                    })
                    .unwrap_or_else(|| without_percent.parse::<u32>().map(ChangeBrightness::Set))
                    .map_err(|err| {
                        format!(
                            "failed to parse percentage `{s}`: {err}, expected \
                    for example `+10%`, `-10%` or `50%`"
                        )
                    })?
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_brightness() {
        assert_eq!(
            "-8%".parse::<ChangeBrightness>().unwrap(),
            ChangeBrightness::Decrease(8)
        );
        "-8".parse::<ChangeBrightness>().unwrap_err();

        assert_eq!(
            "+14%".parse::<ChangeBrightness>().unwrap(),
            ChangeBrightness::Increase(14)
        );
        "+14".parse::<ChangeBrightness>().unwrap_err();

        assert_eq!(
            "50%".parse::<ChangeBrightness>().unwrap(),
            ChangeBrightness::Set(50)
        );
        "50".parse::<ChangeBrightness>().unwrap_err();

        assert_eq!(
            "max".parse::<ChangeBrightness>().unwrap(),
            ChangeBrightness::Set(100)
        );
        assert_eq!(
            "min".parse::<ChangeBrightness>().unwrap(),
            ChangeBrightness::Set(0)
        );
    }
}
