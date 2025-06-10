//! lumina allows you to control brightness of devices

mod change_brightness;
mod cli;

use std::fmt::{Display, Write as _};

use ::brightness::blocking::{self as brightness, Brightness as _};
use clap::{CommandFactory as _, Parser as _};
use cli::{Cli, Command};
use colored::Colorize as _;

/// Information about a device
struct DeviceInfo {
    /// Name of device, e.g. `intel_backlight`
    name: String,
    /// Brightness of the device in percent, e.g. 40% is `40`
    brightness: u32,
    /// Whether to output JSON
    is_json: bool,
}

impl Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let brightness = self.brightness;

        if self.is_json {
            write!(
                f,
                r"{} {}{} {}{} {}{} {} {}",
                "{".bright_black(),
                r#""name""#.bright_green(),
                ":".bright_black(),
                format!(r#""{name}""#).bright_green(),
                ",".bright_black(),
                r#""brightness""#.bright_green(),
                ":".bright_black(),
                brightness.to_string().bright_yellow(),
                "}".bright_black()
            )
        } else {
            write!(
                f,
                "{}{} {}{}",
                name.bright_green(),
                ":".bright_black(),
                brightness.to_string().bright_yellow(),
                "%".bright_yellow()
            )
        }
    }
}

/// Colored prefix of an error
fn err_prefix() -> String {
    format!(
        "{}{}{}",
        "[".bright_black(),
        "ERROR".bright_red(),
        "]".bright_black()
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::MarkdownHelp => {
            clap_markdown::print_help_markdown::<Cli>();
        }
        Command::Completion { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
        }
        Command::Set {
            action,
            device,
            silent,
            json,
        } => {
            let device = device
                .map_or_else(
                    || brightness::brightness_devices().flatten().next(),
                    |x| {
                        brightness::brightness_devices()
                            .flatten()
                            .find(|brightness_device| {
                                brightness_device.device_name().is_ok_and(|name| name == x)
                            })
                    },
                )
                .ok_or_else(|| format!("{} no device found", err_prefix()))?;

            action.change_brightness_of_device(&device)?;

            if !silent {
                let info = DeviceInfo {
                    name: device.device_name()?,
                    brightness: device.get()?,
                    is_json: json,
                };

                println!("{info}");
            }
        }
        Command::Get {
            device,
            silent,
            json,
        } => {
            let dev = device
                .map_or_else(
                    || brightness::brightness_devices().flatten().next(),
                    |device_name| {
                        brightness::brightness_devices()
                            .flatten()
                            .find(|brightness_device| {
                                brightness_device
                                    .device_name()
                                    .is_ok_and(|nam| nam == device_name)
                            })
                    },
                )
                .ok_or_else(|| format!("{} no device found", err_prefix()))?;

            if !silent {
                let brightness = dev.get()?;

                let info = DeviceInfo {
                    name: dev.device_name()?,
                    brightness,
                    is_json: json,
                };

                println!("{info}");
            }
        }
        Command::List { json } => {
            let mut output = if json {
                format!(
                    "{}\n  {}{} {}\n",
                    "{".bright_black(),
                    r#""devices""#.bright_green(),
                    ":".bright_black(),
                    "[".bright_black()
                )
            } else {
                String::new()
            };

            for device in brightness::brightness_devices() {
                match device {
                    Ok(device) => {
                        let name = device.device_name().map_err(|err| {
                            format!(
                                "{} failed to get name of one of the devices: {err}",
                                err_prefix()
                            )
                        })?;

                        let brightness = device.get().map_err(|err| {
                            format!(
                                "{} failed to get brightness of device {name}: {err}",
                                err_prefix()
                            )
                        })?;

                        let device = DeviceInfo {
                            name,
                            brightness,
                            is_json: json,
                        };

                        if json {
                            writeln!(&mut output, "    {}{}", device, ",".bright_black())?;
                        } else {
                            writeln!(&mut output, "{device}")?;
                        }
                    }
                    Err(err) => {
                        if json {
                            eprintln!("{err}");
                        } else {
                            eprintln!("{} {err}", err_prefix());
                        }
                    }
                }
            }

            let output = if json {
                let mut output = output
                    .strip_suffix(",")
                    .map(ToString::to_string)
                    .unwrap_or(output);
                write!(&mut output, "{}", "  ]\n}".bright_black())?;
                output
            } else {
                output
            };

            print!("{output}");
        }
    }

    Ok(())
}
