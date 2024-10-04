use std::fmt::Display;
use std::str::FromStr;

/// Holds the RGB values of a color
#[derive(Debug, Default, PartialEq)]
pub struct RGB<T>(pub T, pub T, pub T);

pub trait RGBColor<T> {
    /// Returns the red value of the RGB color
    fn r(&self) -> T;
    /// Returns the green value of the RGB color
    fn g(&self) -> T;
    /// Returns the blue value of the RGB color
    fn b(&self) -> T;
    /// Returns the ANSI code for the RGB color
    fn ansi_code(&self) -> String
    where
        T: Copy + Display + Into<u8>;
}

impl<T> RGBColor<T> for RGB<T>
where
    T: Copy,
{
    fn r(&self) -> T {
        self.0
    }

    fn g(&self) -> T {
        self.1
    }

    fn b(&self) -> T {
        self.2
    }

    fn ansi_code(&self) -> String
    where
        T: Copy + Display + Into<u8>,
    {
        format!("\x1b[38;2;{};{};{}m", self.r(), self.g(), self.b())
    }
}

/// The modes of gradient colors
#[derive(Default)]
pub enum GradientMode {
    #[default]
    Rainbow,
    Linear,
}

impl From<&String> for GradientMode {
    fn from(value: &String) -> Self {
        match value.to_lowercase().as_str() {
            "rainbow" => GradientMode::Rainbow,
            "linear" => GradientMode::Linear,
            _ => GradientMode::Rainbow,
        }
    }
}

/// Calculate the delta between the two RGB colors
fn calculate_delta(start: &RGB<u8>, end: &RGB<u8>) -> RGB<f32> {
    RGB(
        end.r() as f32 - start.r() as f32,
        end.g() as f32 - start.g() as f32,
        end.b() as f32 - start.b() as f32,
    )
}

/// Interpolate between two colors
pub fn interpolate_linear_gradient(start: &RGB<u8>, end: &RGB<u8>, factor: f32) -> RGB<u8> {
    let delta = calculate_delta(start, end);
    let r = start.r() as f32 + factor * delta.r();
    let g = start.g() as f32 + factor * delta.g();
    let b = start.b() as f32 + factor * delta.b();
    RGB(r.round() as u8, g.round() as u8, b.round() as u8)
}

pub fn rainbow(offset: f32, frequency: f32, spread: f32, shift: f32) -> RGB<u8> {
    let i = (offset + shift) / spread; // The current index, used to calculate the position on the sine wave.

    let amplitude = 127.0; // Amplitude of the sine wave. Describe the magnitude of the color.
    let shift_offset = 128.0; // Shifts the entire sine wave up so that it oscillates between 0 and 255 (128 + 127)

    let phase_r = (0.0 / 3.0) * std::f32::consts::PI; // The phase shift for red (0 or 360deg)
    let phase_g = (2.0 / 3.0) * std::f32::consts::PI; // The phase shift for green (2/3 PI = 120deg)
    let phase_b = (4.0 / 3.0) * std::f32::consts::PI; // The phase shift for blue (4/3 PI = 240 deg)

    // Calculate the RGB values
    let r = (amplitude * (frequency * i + phase_r).sin() + shift_offset) as u8;
    let g = (amplitude * (frequency * i + phase_g).sin() + shift_offset) as u8;
    let b = (amplitude * (frequency * i + phase_b).sin() + shift_offset) as u8;
    RGB(r, g, b)
}

// -----------
// PARSE COLOR
// -----------

#[derive(Debug, PartialEq)]
pub enum ParseColorError {
    InvalidFormat,
    InvalidValues,
}

impl FromStr for RGB<u8> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("#") && s.len() == 7 {
            RGB::from_hex_str(s)
        } else if s.to_lowercase().starts_with("rgb") {
            RGB::from_rgb_str(s)
        } else {
            Err(ParseColorError::InvalidFormat)
        }
    }
}

impl RGB<u8> {
    fn from_hex_str(s: &str) -> Result<Self, ParseColorError> {
        let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| ParseColorError::InvalidValues)?;
        let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| ParseColorError::InvalidValues)?;
        let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| ParseColorError::InvalidValues)?;
        Ok(Self(r, g, b))
    }

    fn from_rgb_str(s: &str) -> Result<Self, ParseColorError> {
        let parts: Vec<&str> = s.split(',').map(|part| part.trim()).collect();
        if parts.len() == 3 {
            if let (Ok(r), Ok(g), Ok(b)) = (parts[0].parse(), parts[1].parse(), parts[2].parse()) {
                return Ok(Self(r, g, b));
            } else {
                return Err(ParseColorError::InvalidFormat);
            }
        } else {
            Err(ParseColorError::InvalidFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_ansi_code() {
        let color = RGB(255, 0, 0);
        assert_eq!(color.ansi_code(), "\x1b[38;2;255;0;0m");

        let color = RGB(0, 255, 0);
        assert_eq!(color.ansi_code(), "\x1b[38;2;0;255;0m");

        let color = RGB(0, 0, 255);
        assert_eq!(color.ansi_code(), "\x1b[38;2;0;0;255m");
    }

    #[test]
    fn test_calculate_delta() {
        let start = RGB(255, 0, 0);
        let end = RGB(0, 0, 255);
        let delta = calculate_delta(&start, &end);
        assert_eq!(delta, RGB(-255.0, 0.0, 255.0));

        let start = RGB(0, 255, 0);
        let end = RGB(0, 0, 255);
        let delta = calculate_delta(&start, &end);
        assert_eq!(delta, RGB(0.0, -255.0, 255.0));
    }

    #[test]
    fn test_interpolate_linear_gradient() {
        let start = RGB(255, 0, 0);
        let end = RGB(0, 0, 255);
        let mid = interpolate_linear_gradient(&start, &end, 0.5);
        assert_eq!(mid, RGB(128, 0, 128));

        let start = RGB(0, 255, 0);
        let end = RGB(0, 0, 255);
        let mid = interpolate_linear_gradient(&start, &end, 0.5);
        assert_eq!(mid, RGB(0, 128, 128));
    }

    #[test]
    fn test_rgb_from_hex_str() {
        assert_eq!(RGB::from_hex_str("#000000"), Ok(RGB(0, 0, 0)));
        assert_eq!(RGB::from_hex_str("#FFFFFF"), Ok(RGB(255, 255, 255)));
        assert_eq!(RGB::from_hex_str("#FF0000"), Ok(RGB(255, 0, 0)));
        assert_eq!(RGB::from_hex_str("#00FF00"), Ok(RGB(0, 255, 0)));
        assert_eq!(RGB::from_hex_str("#0000FF"), Ok(RGB(0, 0, 255)));
        assert_eq!(
            RGB::from_str("#GGGGGG"),
            Err(ParseColorError::InvalidValues)
        );
    }

    #[test]
    fn test_rgb_from_rgb_str() {
        assert_eq!(RGB::from_rgb_str("0,0,0"), Ok(RGB(0, 0, 0)));
        assert_eq!(RGB::from_rgb_str("255,255,255"), Ok(RGB(255, 255, 255)));
        assert_eq!(RGB::from_rgb_str("255,0,0"), Ok(RGB(255, 0, 0)));
        assert_eq!(RGB::from_rgb_str("0,255,0"), Ok(RGB(0, 255, 0)));
        assert_eq!(RGB::from_rgb_str("0,0,255"), Ok(RGB(0, 0, 255)));
        assert_eq!(RGB::from_rgb_str("124, 64, 39"), Ok(RGB(124, 64, 39)));
        assert_eq!(
            RGB::from_str("255,255"),
            Err(ParseColorError::InvalidFormat)
        );
        assert_eq!(
            RGB::from_str("255,GGG,0"),
            Err(ParseColorError::InvalidFormat)
        );
    }
}
