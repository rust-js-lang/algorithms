use crate::algorithms::math::complex_number::{ComplexNumber, Operand, Parts};
use std::f64::consts::PI;

const CLOSE_TO_ZERO_THRESHOLD: f64 = 1e-10;

/// The amplitudes of `frequencies`, by the inverse discrete Fourier
/// transform, each part below `zero_threshold`, `1e-10` where it's left
/// out, zero.
pub fn inverse_discrete_fourier_transform(
    frequencies: &[ComplexNumber],
    zero_threshold: Option<f64>,
) -> Vec<f64> {
    let zero_threshold = zero_threshold.unwrap_or(CLOSE_TO_ZERO_THRESHOLD);
    let n = frequencies.len();
    let mut amplitudes = Vec::with_capacity(n);

    // Go through every discrete point of time.
    for timer in 0..n {
        // Compound amplitude at current time.
        let mut amplitude = ComplexNumber::new(None);

        // Go through all discrete frequencies.
        for (frequency, current_frequency) in frequencies.iter().enumerate() {
            // Calculate rotation angle.
            let rotation_angle = (2.0 * PI) * frequency as f64 * (timer as f64 / n as f64);

            // Remember that e^ix = cos(x) + i * sin(x);
            let frequency_contribution = ComplexNumber::new(Some(Parts {
                re: Some(rotation_angle.cos()),
                im: Some(rotation_angle.sin()),
            }))
            .multiply(Operand::Complex(ComplexNumber::new(Some(Parts {
                re: Some(current_frequency.re),
                im: Some(current_frequency.im),
            }))));

            amplitude = amplitude.add(Operand::Complex(frequency_contribution));
        }

        // Close to zero? You're zero.
        if amplitude.re.abs() < zero_threshold {
            amplitude.re = 0.0;
        }

        if amplitude.im.abs() < zero_threshold {
            amplitude.im = 0.0;
        }

        // Add current frequency signal to the list of compound signals.
        amplitudes.push(amplitude.re);
    }

    amplitudes
}

js::export_default!(inverse_discrete_fourier_transform);
