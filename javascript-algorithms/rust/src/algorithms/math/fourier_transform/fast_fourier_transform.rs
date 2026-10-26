use crate::algorithms::math::bits::bit_length::bit_length;
use crate::algorithms::math::complex_number::{ComplexNumber, Operand, Parts};
use std::f64::consts::PI;

/// `input`'s lowest `bits_count` bits in reverse order.
fn reverse_bits(input: usize, bits_count: u32) -> usize {
    let mut reversed_bits = 0;

    for bit_index in 0..bits_count {
        reversed_bits *= 2;

        if (input / (1 << bit_index)) % 2 == 1 {
            reversed_bits += 1;
        }
    }

    reversed_bits
}

/// `number`'s parts, a complex number of its own.
fn copy_of(number: &ComplexNumber) -> ComplexNumber {
    ComplexNumber::new(Some(Parts {
        re: Some(number.re),
        im: Some(number.im),
    }))
}

/// The radix-2 fast Fourier transform of `input_data`, its inverse where
/// `inverse` is true; `input_data` padded with zeros to a power of two.
pub fn fast_fourier_transform(
    input_data: &mut Vec<ComplexNumber>,
    inverse: Option<bool>,
) -> Vec<ComplexNumber> {
    let inverse = inverse.unwrap_or(false);
    let bits_count = bit_length(input_data.len() as i32 - 1);
    let n = 1 << bits_count;

    while input_data.len() < n {
        input_data.push(ComplexNumber::new(None));
    }

    let mut output: Vec<ComplexNumber> = (0..n)
        .map(|data_sample_index| copy_of(&input_data[reverse_bits(data_sample_index, bits_count)]))
        .collect();

    let mut block_length = 2;
    while block_length <= n {
        let imaginary_sign = if inverse { -1.0 } else { 1.0 };
        let phase_step = ComplexNumber::new(Some(Parts {
            re: Some(((2.0 * PI) / block_length as f64).cos()),
            im: Some(imaginary_sign * ((2.0 * PI) / block_length as f64).sin()),
        }));

        for block_start in (0..n).step_by(block_length) {
            let mut phase = ComplexNumber::new(Some(Parts {
                re: Some(1.0),
                im: Some(0.0),
            }));

            for signal_id in block_start..(block_start + block_length / 2) {
                let component = output[signal_id + block_length / 2]
                    .multiply(Operand::Complex(copy_of(&phase)));

                let upd1 = output[signal_id].add(Operand::Complex(copy_of(&component)));
                let upd2 = output[signal_id].subtract(Operand::Complex(component));

                output[signal_id] = upd1;
                output[signal_id + block_length / 2] = upd2;

                phase = phase.multiply(Operand::Complex(copy_of(&phase_step)));
            }
        }
        block_length *= 2;
    }

    if inverse {
        for signal in &mut output {
            *signal = signal.divide(Operand::Real(n as f64));
        }
    }

    output
}

js::export_default!(fast_fourier_transform);
