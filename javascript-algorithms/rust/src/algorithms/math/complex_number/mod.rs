use crate::algorithms::math::radian::radian_to_degree::radian_to_degree;
use std::f64::consts::PI;

/// A complex number's parts, each 0 where it's left out.
#[derive(Default)]
pub struct Parts {
    pub re: Option<f64>,
    pub im: Option<f64>,
}

/// A complex number in polar form.
pub struct PolarForm {
    pub radius: f64,
    pub phase: f64,
}

/// A number, real or complex, that a complex number's arithmetic takes.
#[cfg_attr(rust_js, rust_js::untagged)]
pub enum Operand {
    Real(f64),
    Complex(ComplexNumber),
}

/// `re + im·i`.
#[cfg_attr(rust_js, rust_js::class)]
pub struct ComplexNumber {
    pub re: f64,
    pub im: f64,
}

unsafe impl js::Defined for ComplexNumber {}

#[cfg_attr(rust_js, rust_js::class)]
impl ComplexNumber {
    pub fn new(parts: Option<Parts>) -> Self {
        let parts = parts.unwrap_or_default();
        ComplexNumber {
            re: parts.re.unwrap_or(0.0),
            im: parts.im.unwrap_or(0.0),
        }
    }

    pub fn add(&self, addend: Operand) -> ComplexNumber {
        // Make sure we're dealing with complex number.
        let complex_addend = self.to_complex_number(addend);

        ComplexNumber::new(Some(Parts {
            re: Some(self.re + complex_addend.re),
            im: Some(self.im + complex_addend.im),
        }))
    }

    pub fn subtract(&self, subtrahend: Operand) -> ComplexNumber {
        // Make sure we're dealing with complex number.
        let complex_subtrahend = self.to_complex_number(subtrahend);

        ComplexNumber::new(Some(Parts {
            re: Some(self.re - complex_subtrahend.re),
            im: Some(self.im - complex_subtrahend.im),
        }))
    }

    pub fn multiply(&self, multiplicand: Operand) -> ComplexNumber {
        // Make sure we're dealing with complex number.
        let complex_multiplicand = self.to_complex_number(multiplicand);

        ComplexNumber::new(Some(Parts {
            re: Some(self.re * complex_multiplicand.re - self.im * complex_multiplicand.im),
            im: Some(self.re * complex_multiplicand.im + self.im * complex_multiplicand.re),
        }))
    }

    pub fn divide(&self, divider: Operand) -> ComplexNumber {
        // Make sure we're dealing with complex number.
        let complex_divider = self.to_complex_number(divider);

        // Calculating final divider using formula (a + bi)(a − bi) = a^2 + b^2
        let final_divider = complex_divider.re.powi(2) + complex_divider.im.powi(2);

        // Get divider conjugate.
        let divider_conjugate = self.conjugate(Operand::Complex(complex_divider));

        // Multiply dividend by divider's conjugate.
        let final_divident = self.multiply(Operand::Complex(divider_conjugate));

        ComplexNumber::new(Some(Parts {
            re: Some(final_divident.re / final_divider),
            im: Some(final_divident.im / final_divider),
        }))
    }

    pub fn conjugate(&self, number: Operand) -> ComplexNumber {
        // Make sure we're dealing with complex number.
        let complex_number = self.to_complex_number(number);

        ComplexNumber::new(Some(Parts {
            re: Some(complex_number.re),
            im: Some(-complex_number.im),
        }))
    }

    pub fn get_radius(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    pub fn get_phase(&self, in_radians: Option<bool>) -> f64 {
        let mut phase = (self.im.abs() / self.re.abs()).atan();

        if self.re < 0.0 && self.im > 0.0 {
            phase = PI - phase;
        } else if self.re < 0.0 && self.im < 0.0 {
            phase = -(PI - phase);
        } else if self.re > 0.0 && self.im < 0.0 {
            phase = -phase;
        } else if self.re == 0.0 && self.im > 0.0 {
            phase = PI / 2.0;
        } else if self.re == 0.0 && self.im < 0.0 {
            phase = -PI / 2.0;
        } else if self.re < 0.0 && self.im == 0.0 {
            phase = PI;
        } else if self.re > 0.0 && self.im == 0.0 {
            phase = 0.0;
        } else if self.re == 0.0 && self.im == 0.0 {
            // More correctly would be to set 'indeterminate'.
            // But just for simplicity reasons let's set zero.
            phase = 0.0;
        }

        if !in_radians.unwrap_or(true) {
            phase = radian_to_degree(phase);
        }

        phase
    }

    pub fn get_polar_form(&self, in_radians: Option<bool>) -> PolarForm {
        PolarForm {
            radius: self.get_radius(),
            phase: self.get_phase(in_radians),
        }
    }

    pub fn to_complex_number(&self, number: Operand) -> ComplexNumber {
        match number {
            Operand::Complex(number) => number,
            Operand::Real(re) => ComplexNumber::new(Some(Parts {
                re: Some(re),
                ..Parts::default()
            })),
        }
    }
}

js::export_default!(ComplexNumber);
