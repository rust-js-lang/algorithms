use crate::algorithms::math::complex_number::{ComplexNumber, Operand, Parts};
use std::f64::consts::PI;

const CLOSE_TO_ZERO_THRESHOLD: f64 = 1e-10;

/// The discrete Fourier transform of `input_amplitudes`: each frequency's
/// signal, each part below `zero_threshold`, `1e-10` where it's left out,
/// zero.
pub fn dft(input_amplitudes: &[f64], zero_threshold: Option<f64>) -> Vec<ComplexNumber> {
    let zero_threshold = zero_threshold.unwrap_or(CLOSE_TO_ZERO_THRESHOLD);
    let n = input_amplitudes.len();

    let mut signals = Vec::with_capacity(n);

    // Go through every discrete frequency.
    for frequency in 0..n {
        // Compound signal at current frequency that will ultimately
        // take part in forming input amplitudes.
        let mut frequency_signal = ComplexNumber::new(None);

        // Go through every discrete point in time.
        for (timer, &current_amplitude) in input_amplitudes.iter().enumerate() {
            // Calculate rotation angle.
            let rotation_angle = -(2.0 * PI) * frequency as f64 * (timer as f64 / n as f64);

            // Remember that e^ix = cos(x) + i * sin(x);
            let data_point_contribution = ComplexNumber::new(Some(Parts {
                re: Some(rotation_angle.cos()),
                im: Some(rotation_angle.sin()),
            }))
            .multiply(Operand::Real(current_amplitude));

            // Add this data point's contribution.
            frequency_signal = frequency_signal.add(Operand::Complex(data_point_contribution));
        }

        // Close to zero? You're zero.
        if frequency_signal.re.abs() < zero_threshold {
            frequency_signal.re = 0.0;
        }

        if frequency_signal.im.abs() < zero_threshold {
            frequency_signal.im = 0.0;
        }

        // Average contribution at this frequency.
        // The 1/N factor is usually moved to the reverse transform (going from frequencies
        // back to time). This is allowed, though it would be nice to have 1/N in the forward
        // transform since it gives the actual sizes for the time spikes.
        frequency_signal = frequency_signal.divide(Operand::Real(n as f64));

        // Add current frequency signal to the list of compound signals.
        signals.push(frequency_signal);
    }

    signals
}

js::export_default!(dft);
