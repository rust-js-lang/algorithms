use js::Unknown;
use std::rc::Rc;

/// A function that compares two values: negative where the first is
/// less, zero where they're equal, positive where it's greater.
pub type CompareFunction = Rc<dyn Fn(&Unknown, &Unknown) -> f64>;

/// Compares values, by a compare function given, or by JS's `<`.
#[cfg_attr(rust_js, rust_js::class)]
pub struct Comparator {
    pub compare: CompareFunction,
}

unsafe impl js::Defined for Comparator {}

#[cfg_attr(rust_js, rust_js::class)]
impl Comparator {
    /// Constructor.
    /// @param {function(a: *, b: *)} [compareFunction] - It may be custom compare function that, let's
    /// say may compare custom objects together.
    pub fn new(compare_function: Option<CompareFunction>) -> Self {
        Comparator {
            compare: compare_function
                .unwrap_or_else(|| Rc::new(Comparator::default_compare_function)),
        }
    }

    /// Default comparison function. It just assumes that "a" and "b" are strings or numbers.
    pub fn default_compare_function(a: &Unknown, b: &Unknown) -> f64 {
        if a == b {
            return 0.0;
        }

        if a < b { -1.0 } else { 1.0 }
    }

    /// Checks if two variables are equal.
    pub fn equal(&self, a: &Unknown, b: &Unknown) -> bool {
        (self.compare)(a, b) == 0.0
    }

    /// Checks if variable "a" is less than "b".
    pub fn less_than(&self, a: &Unknown, b: &Unknown) -> bool {
        (self.compare)(a, b) < 0.0
    }

    /// Checks if variable "a" is greater than "b".
    pub fn greater_than(&self, a: &Unknown, b: &Unknown) -> bool {
        (self.compare)(a, b) > 0.0
    }

    /// Checks if variable "a" is less than or equal to "b".
    pub fn less_than_or_equal(&self, a: &Unknown, b: &Unknown) -> bool {
        self.less_than(a, b) || self.equal(a, b)
    }

    /// Checks if variable "a" is greater than or equal to "b".
    pub fn greater_than_or_equal(&self, a: &Unknown, b: &Unknown) -> bool {
        self.greater_than(a, b) || self.equal(a, b)
    }

    /// Reverses the comparison order.
    pub fn reverse(&mut self) {
        let compare_original = Rc::clone(&self.compare);
        self.compare = Rc::new(move |a, b| compare_original(b, a));
    }
}

js::export_default!(Comparator);
