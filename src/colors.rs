use std::fmt::Display;

/// Holds the RGB values of a color
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
    RGB(r as u8, g as u8, b as u8)
}
