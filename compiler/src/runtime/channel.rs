use crate::ir::ChannelModel;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal};

use super::complex::Complex;

/// Trait implemented by communication channels.
pub trait Channel {
    fn transmit(&self, samples: &[Complex]) -> Vec<Complex>;
}

/// AWGN channel.
#[derive(Debug, Clone)]
pub struct AwgnChannel {
    pub snr_db: f64,
    pub seed: u64,
}

impl AwgnChannel {
    pub fn new(snr_db: f64, seed: u64) -> Self {
        Self { snr_db, seed }
    }
}

impl Channel for AwgnChannel {
    fn transmit(&self, samples: &[Complex]) -> Vec<Complex> {
        awgn(samples, self.snr_db, self.seed)
    }
}

/// Apply a channel model.
pub fn apply_channel(
    samples: &[Complex],
    model: &ChannelModel,
    snr_db: f64,
    seed: u64,
) -> Vec<Complex> {
    match model {
        ChannelModel::AWGN => {
            let channel = AwgnChannel::new(snr_db, seed);

            channel.transmit(samples)
        }

        _ => panic!("Channel model not yet implemented: {:?}", model),
    }
}

/// Additive White Gaussian Noise.
///
/// This implementation interprets `snr_db` as
/// signal-power / noise-power.
fn awgn(samples: &[Complex], snr_db: f64, seed: u64) -> Vec<Complex> {
    if samples.is_empty() {
        return Vec::new();
    }

    let snr_linear = 10.0_f64.powf(snr_db / 10.0);

    let signal_power = samples
        .iter()
        .map(|sample| sample.magnitude_squared())
        .sum::<f64>()
        / samples.len() as f64;

    let noise_power = signal_power / snr_linear;

    // Complex noise:
    //
    // N = nI + j*nQ
    //
    // Each component has half the total noise power.
    let component_variance = noise_power / 2.0;

    let component_std = component_variance.sqrt();

    let normal = Normal::new(0.0, component_std).expect("Invalid Gaussian distribution");

    let mut rng = StdRng::seed_from_u64(seed);

    samples
        .iter()
        .map(|sample| {
            let noise_i = normal.sample(&mut rng);

            let noise_q = normal.sample(&mut rng);

            Complex::new(sample.re + noise_i, sample.im + noise_q)
        })
        .collect()
}

/// Generate deterministic random bits.
pub fn random_bits(count: usize, seed: u64) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(seed);

    (0..count).map(|_| rng.gen_range(0..=1)).collect()
}

/// Calculate bit errors and BER.
pub fn calculate_ber(transmitted: &[u8], received: &[u8]) -> (u64, f64) {
    let count = transmitted.len().min(received.len());

    if count == 0 {
        return (0, 0.0);
    }

    let errors = transmitted[..count]
        .iter()
        .zip(received[..count].iter())
        .filter(|(a, b)| a != b)
        .count() as u64;

    let ber = errors as f64 / count as f64;

    (errors, ber)
}

/// Convert Eb/N0 from dB to linear scale.
pub fn db_to_linear(db: f64) -> f64 {
    10.0_f64.powf(db / 10.0)
}

/// Convert linear ratio to dB.
pub fn linear_to_db(value: f64) -> f64 {
    10.0 * value.log10()
}
