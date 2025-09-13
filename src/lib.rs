#![no_std]
cfg_if! {
    if #[cfg(feature = "std")] {
        extern crate std;
    } else {
        use num_traits::real::Real;
    }
}
use cfg_if::cfg_if;

#[must_use]
pub const fn kelvin_to_celsius(val: f64) -> f64 {
    val - 273.15
}

#[must_use]
pub fn kelvin_to_fahrenheit(val: f64) -> f64 {
    cfg_if! {
        if #[cfg(feature = "std")] {
            f64::mul_add(val - 273.15, 9. / 5., 32.)
        } else {
            Real::mul_add(val - 273.15, 9. / 5., 32.)
        }
    }
}

#[must_use]
pub const fn celsius_to_kelvin(val: f64) -> f64 {
    val + 273.15
}

#[must_use]
pub fn celsius_to_fahrenheit(val: f64) -> f64 {
    cfg_if! {
        if #[cfg(feature = "std")] {
            f64::mul_add(val, 9. / 5., 32.)
        } else {
            Real::mul_add(val, 9. / 5., 32.)
        }
    }
}

#[must_use]
pub fn fahrenheit_to_kelvin(val: f64) -> f64 {
    cfg_if! {
        if #[cfg(feature = "std")] {
            f64::mul_add(val - 32., 5. / 9., 273.15)
        } else {
            Real::mul_add(val - 32., 5. / 9., 273.15)
        }
    }
}

#[must_use]
pub const fn fahrenheit_to_celsius(val: f64) -> f64 {
    (val - 32.) * (5. / 9.)
}

#[cfg(test)]
mod temperature_tests {
    use super::*;
    use rstest::rstest;

    const PRECISION: f64 = f64::EPSILON;

    /// Helper function to compare f64 values within precision
    fn approx_equal(x: f64, y: f64) -> bool {
        (x - y).abs() < PRECISION
    }

    #[rstest]
    #[case(0.0, -273.15)]
    #[case(100.0, -173.14999999999998)]
    #[case(1000.0, 726.85)]
    #[case(5000.0, 4726.85)]
    fn test_kelvin_to_celsius_precision(#[case] kelvin_val: f64, #[case] celsius_val: f64) {
        let converted = kelvin_to_celsius(kelvin_val);
        assert!(
            approx_equal(converted, celsius_val),
            "High precision K->C failed - Expected: {celsius_val}, Got: {converted}"
        );
    }

    #[rstest]
    #[case(0.0, -459.66999999999996)]
    #[case(100.0, -279.66999999999996)]
    #[case(1000.0, 1340.3300000000002)]
    #[case(5000.0, 8540.330000000002)]
    #[case(69420.0, 124496.33000000002)]
    fn test_kelvin_to_fahrenheit_precision(#[case] kelvin_val: f64, #[case] fahrenheit_val: f64) {
        let converted = kelvin_to_fahrenheit(kelvin_val);
        assert!(
            approx_equal(converted, fahrenheit_val),
            "High precision K->F failed - Expected: {fahrenheit_val}, Got: {converted}"
        );
    }

    #[rstest]
    #[case(0.0, 273.15)]
    #[case(100.0, 373.15)]
    #[case(1000.0, 1273.15)]
    #[case(5000.0, 5273.15)]
    #[case(69420.0, 69693.15)]
    fn test_celsius_to_kelvin_precision(#[case] celsius_val: f64, #[case] kelvin_val: f64) {
        let converted = celsius_to_kelvin(celsius_val);
        assert!(
            approx_equal(converted, kelvin_val),
            "High precision C->K failed - Expected: {kelvin_val}, Got: {converted}"
        );
    }

    #[rstest]
    #[case(0.0, 32.0)]
    #[case(100.0, 212.0)]
    #[case(1000.0, 1832.0)]
    #[case(5000.0, 9032.0)]
    #[case(69420.0, 124988.0)]
    fn test_celsius_to_fahrenheit_precision(#[case] celsius_val: f64, #[case] fahrenheit_val: f64) {
        let converted = celsius_to_fahrenheit(celsius_val);
        assert!(
            approx_equal(converted, fahrenheit_val),
            "High precision C->F failed - Expected: {fahrenheit_val}, Got: {converted}"
        );
    }

    #[rstest]
    #[case(0.0, 255.3722222222222)]
    #[case(100.0, 310.92777777777775)]
    #[case(1000.0, 810.9277777777778)]
    #[case(5000.0, 3033.15)]
    #[case(69420.0, 38822.03888888889)]
    fn test_fahrenheit_to_kelvin_precision(#[case] fahrenheit_val: f64, #[case] kelvin_val: f64) {
        let converted = fahrenheit_to_kelvin(fahrenheit_val);
        assert!(
            approx_equal(converted, kelvin_val),
            "High precision F->K failed - Expected: {kelvin_val}, Got: {converted}"
        );
    }

    #[rstest]
    #[case(0.0, -17.77777777777778)]
    #[case(100.0, 37.77777777777778)]
    #[case(1000.0, 537.7777777777778)]
    #[case(5000.0, 2760.0)]
    #[case(69420.0, 38548.88888888889)]
    fn test_fahrenheit_to_celsius_precision(#[case] fahrenheit_val: f64, #[case] celsius_val: f64) {
        let converted = fahrenheit_to_celsius(fahrenheit_val);
        assert!(
            approx_equal(converted, celsius_val),
            "High precision F->C failed - Expected: {celsius_val}, Got: {converted}"
        );
    }
}
