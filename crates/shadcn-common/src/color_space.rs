//! RGB / HSB / HSL color math for pickers (Zag `@zag-js/color-utils`).
//!
//! Separate from theme [`crate::color`] registries — these types model channel
//! editing, not shadcn semantic tokens.

use crate::value_mapping::{finite_or_zero, modulo, round_to_step_precision};

/// sRGB color with alpha in `0.0..=1.0` and channels in `0..=255`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    /// Red `0..=255`.
    pub red: f32,
    /// Green `0..=255`.
    pub green: f32,
    /// Blue `0..=255`.
    pub blue: f32,
    /// Alpha `0.0..=1.0`.
    pub alpha: f32,
}

/// Hue / saturation / brightness (HSV) with alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsba {
    /// Hue degrees `0..360`.
    pub hue: f32,
    /// Saturation percent `0..=100`.
    pub saturation: f32,
    /// Brightness percent `0..=100`.
    pub brightness: f32,
    /// Alpha `0.0..=1.0`.
    pub alpha: f32,
}

/// Hue / saturation / lightness with alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsla {
    /// Hue degrees `0..360`.
    pub hue: f32,
    /// Saturation percent `0..=100`.
    pub saturation: f32,
    /// Lightness percent `0..=100`.
    pub lightness: f32,
    /// Alpha `0.0..=1.0`.
    pub alpha: f32,
}

impl Rgba {
    /// Clamped opaque RGB color.
    #[must_use]
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red: clamp_channel(red, 0.0, 255.0),
            green: clamp_channel(green, 0.0, 255.0),
            blue: clamp_channel(blue, 0.0, 255.0),
            alpha: clamp_channel(alpha, 0.0, 1.0),
        }
    }

    /// Parses `#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`.
    #[must_use]
    pub fn parse_hex(input: &str) -> Option<Self> {
        let input = input.trim();
        if !input.starts_with('#') {
            return None;
        }
        let hex = &input[1..];
        let expanded = match hex.len() {
            3 | 4 => {
                let mut out = String::with_capacity(hex.len() * 2);
                for ch in hex.chars() {
                    out.push(ch);
                    out.push(ch);
                }
                out
            }
            6 | 8 => hex.to_owned(),
            _ => return None,
        };
        let value = u32::from_str_radix(&expanded, 16).ok()?;
        match expanded.len() {
            6 => Some(Self::new(
                ((value >> 16) & 0xff) as f32,
                ((value >> 8) & 0xff) as f32,
                (value & 0xff) as f32,
                1.0,
            )),
            8 => Some(Self::new(
                ((value >> 24) & 0xff) as f32,
                ((value >> 16) & 0xff) as f32,
                ((value >> 8) & 0xff) as f32,
                ((value & 0xff) as f32) / 255.0,
            )),
            _ => None,
        }
    }

    /// `#RRGGBB` (no alpha).
    #[must_use]
    pub fn to_hex(self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            self.red.round().clamp(0.0, 255.0) as u8,
            self.green.round().clamp(0.0, 255.0) as u8,
            self.blue.round().clamp(0.0, 255.0) as u8,
        )
    }

    /// Converts to HSB.
    #[must_use]
    pub fn to_hsba(self) -> Hsba {
        let red = self.red / 255.0;
        let green = self.green / 255.0;
        let blue = self.blue / 255.0;
        let min = red.min(green).min(blue);
        let brightness = red.max(green).max(blue);
        let chroma = brightness - min;
        let saturation = if brightness == 0.0 {
            0.0
        } else {
            chroma / brightness
        };
        let mut hue = 0.0;
        if chroma != 0.0 {
            hue = if (brightness - red).abs() <= f32::EPSILON {
                (green - blue) / chroma + if green < blue { 6.0 } else { 0.0 }
            } else if (brightness - green).abs() <= f32::EPSILON {
                (blue - red) / chroma + 2.0
            } else {
                (red - green) / chroma + 4.0
            };
            hue /= 6.0;
        }
        Hsba {
            hue: fixed(hue * 360.0, 2),
            saturation: fixed(saturation * 100.0, 2),
            brightness: fixed(brightness * 100.0, 2),
            alpha: fixed(self.alpha, 2),
        }
    }

    /// Converts to HSL.
    #[must_use]
    pub fn to_hsla(self) -> Hsla {
        let red = self.red / 255.0;
        let green = self.green / 255.0;
        let blue = self.blue / 255.0;
        let min = red.min(green).min(blue);
        let max = red.max(green).max(blue);
        let lightness = (max + min) / 2.0;
        let chroma = max - min;
        let (hue, saturation) = if chroma == 0.0 {
            (0.0, 0.0)
        } else {
            let saturation = chroma
                / if lightness < 0.5 {
                    max + min
                } else {
                    2.0 - max - min
                };
            let hue = if (max - red).abs() <= f32::EPSILON {
                (green - blue) / chroma + if green < blue { 6.0 } else { 0.0 }
            } else if (max - green).abs() <= f32::EPSILON {
                (blue - red) / chroma + 2.0
            } else {
                (red - green) / chroma + 4.0
            } / 6.0;
            (hue, saturation)
        };
        Hsla {
            hue: fixed(hue * 360.0, 2),
            saturation: fixed(saturation * 100.0, 2),
            lightness: fixed(lightness * 100.0, 2),
            alpha: fixed(self.alpha, 2),
        }
    }
}

impl Hsba {
    /// Clamped HSB color.
    #[must_use]
    pub fn new(hue: f32, saturation: f32, brightness: f32, alpha: f32) -> Self {
        Self {
            hue: modulo(finite_or_zero(hue), 360.0),
            saturation: clamp_channel(saturation, 0.0, 100.0),
            brightness: clamp_channel(brightness, 0.0, 100.0),
            alpha: clamp_channel(alpha, 0.0, 1.0),
        }
    }

    /// Converts to sRGB.
    #[must_use]
    pub fn to_rgba(self) -> Rgba {
        let hue = self.hue;
        let saturation = self.saturation / 100.0;
        let brightness = self.brightness / 100.0;
        let channel = |n: f32| {
            let k = (n + hue / 60.0) % 6.0;
            brightness - saturation * brightness * k.min(4.0 - k).clamp(0.0, 1.0)
        };
        Rgba::new(
            (channel(5.0) * 255.0).round(),
            (channel(3.0) * 255.0).round(),
            (channel(1.0) * 255.0).round(),
            fixed(self.alpha, 2),
        )
    }
}

impl Hsla {
    /// Clamped HSL color.
    #[must_use]
    pub fn new(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        Self {
            hue: modulo(finite_or_zero(hue), 360.0),
            saturation: clamp_channel(saturation, 0.0, 100.0),
            lightness: clamp_channel(lightness, 0.0, 100.0),
            alpha: clamp_channel(alpha, 0.0, 1.0),
        }
    }

    /// Converts to sRGB.
    #[must_use]
    pub fn to_rgba(self) -> Rgba {
        let hue = self.hue;
        let saturation = self.saturation / 100.0;
        let lightness = self.lightness / 100.0;
        let a = saturation * lightness.min(1.0 - lightness);
        let channel = |n: f32| {
            let k = (n + hue / 30.0) % 12.0;
            lightness - a * k.min(9.0 - k).clamp(-1.0, 1.0)
        };
        Rgba::new(
            (channel(0.0) * 255.0).round(),
            (channel(8.0) * 255.0).round(),
            (channel(4.0) * 255.0).round(),
            fixed(self.alpha, 2),
        )
    }
}

fn clamp_channel(value: f32, min: f32, max: f32) -> f32 {
    finite_or_zero(value).clamp(min, max)
}

fn fixed(value: f32, decimals: u32) -> f32 {
    let step = 10f32.powi(-(decimals as i32));
    round_to_step_precision(value, step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_round_trip() {
        let color = Rgba::parse_hex("#ff8800").expect("hex");
        assert_eq!(color.to_hex(), "#FF8800");
        let short = Rgba::parse_hex("#f80").expect("short hex");
        assert_eq!(short.to_hex(), "#FF8800");
    }

    #[test]
    fn rgb_hsb_rgb_preserves_primary() {
        let rgb = Rgba::new(255.0, 0.0, 0.0, 1.0);
        let back = rgb.to_hsba().to_rgba();
        assert!((back.red - 255.0).abs() < 1.0);
        assert!(back.green.abs() < 1.0);
        assert!(back.blue.abs() < 1.0);
    }

    #[test]
    fn hsl_to_rgb_black_and_white() {
        let white = Hsla::new(0.0, 0.0, 100.0, 1.0).to_rgba();
        assert!((white.red - 255.0).abs() < 1.0);
        let black = Hsla::new(0.0, 0.0, 0.0, 1.0).to_rgba();
        assert!(black.red.abs() < 1.0);
    }
}
