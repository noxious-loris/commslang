use crate::ir::Modulation;

use super::complex::Complex;

use std::f64::consts::FRAC_1_SQRT_2;

/// Trait implemented by digital modulators.
pub trait Modulator {
    fn modulate(&self, bits: &[u8]) -> Vec<Complex>;
}

/// Trait implemented by digital demodulators.
pub trait Demodulator {
    fn demodulate(&self, samples: &[Complex]) -> Vec<u8>;
}

/// Runtime implementation of a modulation scheme.
#[derive(Debug, Clone)]
pub struct DigitalModulator {
    pub modulation: Modulation,
}

impl DigitalModulator {
    pub fn new(modulation: Modulation) -> Self {
        Self { modulation }
    }
}

impl Modulator for DigitalModulator {
    fn modulate(&self, bits: &[u8]) -> Vec<Complex> {
        match self.modulation {
            Modulation::BPSK => bpsk_modulate(bits),

            Modulation::QPSK => qpsk_modulate(bits),

            _ => panic!("Modulation not yet implemented: {:?}", self.modulation),
        }
    }
}

impl Demodulator for DigitalModulator {
    fn demodulate(&self, samples: &[Complex]) -> Vec<u8> {
        match self.modulation {
            Modulation::BPSK => bpsk_demodulate(samples),

            Modulation::QPSK => qpsk_demodulate(samples),

            _ => panic!("Demodulation not yet implemented: {:?}", self.modulation),
        }
    }
}

/// Convenience function.
pub fn modulate(bits: &[u8], modulation: &Modulation) -> Vec<Complex> {
    let modulator = DigitalModulator::new(modulation.clone());

    modulator.modulate(bits)
}

/// Convenience function.
pub fn demodulate(samples: &[Complex], modulation: &Modulation) -> Vec<u8> {
    let modulator = DigitalModulator::new(modulation.clone());

    modulator.demodulate(samples)
}

// ============================================================
// BPSK
// ============================================================

fn bpsk_modulate(bits: &[u8]) -> Vec<Complex> {
    bits.iter()
        .map(|&bit| match bit {
            0 => Complex::new(1.0, 0.0),
            1 => Complex::new(-1.0, 0.0),
            _ => panic!("Invalid bit value: {}", bit),
        })
        .collect()
}

fn bpsk_demodulate(samples: &[Complex]) -> Vec<u8> {
    samples
        .iter()
        .map(|sample| if sample.re >= 0.0 { 0 } else { 1 })
        .collect()
}

// ============================================================
// QPSK
// ============================================================

/// Gray-coded QPSK:
///
/// 00 -> (+1,+1)
/// 01 -> (-1,+1)
/// 11 -> (-1,-1)
/// 10 -> (+1,-1)
///
/// Normalized to unit symbol power.
fn qpsk_modulate(bits: &[u8]) -> Vec<Complex> {
    let mut symbols = Vec::with_capacity((bits.len() + 1) / 2);

    let mut i = 0;

    while i < bits.len() {
        let b0 = bits[i];

        let b1 = if i + 1 < bits.len() { bits[i + 1] } else { 0 };

        let symbol = match (b0, b1) {
            (0, 0) => Complex::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),

            (0, 1) => Complex::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),

            (1, 1) => Complex::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),

            (1, 0) => Complex::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),

            _ => unreachable!(),
        };

        symbols.push(symbol);

        i += 2;
    }

    symbols
}

fn qpsk_demodulate(samples: &[Complex]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(samples.len() * 2);

    for sample in samples {
        if sample.re >= 0.0 && sample.im >= 0.0 {
            bits.push(0);
            bits.push(0);
        } else if sample.re < 0.0 && sample.im >= 0.0 {
            bits.push(0);
            bits.push(1);
        } else if sample.re < 0.0 && sample.im < 0.0 {
            bits.push(1);
            bits.push(1);
        } else {
            bits.push(1);
            bits.push(0);
        }
    }

    bits
}
