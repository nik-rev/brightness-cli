//! Command-line interface for `lumina`

use clap::{
    Parser, Subcommand,
    builder::styling::{AnsiColor, Effects},
};

use crate::change_brightness::ChangeBrightness;

/// Styles for the CLI
const STYLES: clap::builder::Styles = clap::builder::Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::BrightCyan.on_default())
    .error(AnsiColor::BrightRed.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::BrightYellow.on_default().effects(Effects::BOLD));

/// Read and control device brightness
#[derive(Parser)]
#[command(version, styles = STYLES, long_about = None)]
pub struct Cli {
    /// Output JSON
    #[arg(long, global = true)]
    pub json: bool,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Set brightness of a device
    Set {
        /// Modify the percentage of brightness
        ///
        /// - Increase: `+5%`
        /// - Decrease: `-5%`
        /// - Set to specific: `40%`
        /// - `max` or `100%` sets to max brightness
        /// - `min` or `0%` sets to min brightness
        #[arg(default_value = "100%", allow_hyphen_values = true)]
        action: ChangeBrightness,
        /// Set brightness for this device (by default, sets the first device)
        #[arg(long)]
        device: Option<String>,
        /// Silence output
        #[arg(short, long)]
        silent: bool,
    },
    /// Get brightness of a device
    Get {
        /// Get brightness of this device (by default, get the first device)
        #[arg(long)]
        device: Option<String>,
        /// Silence output
        #[arg(short, long)]
        silent: bool,
    },
    /// List each device and its brightness
    List,
}
