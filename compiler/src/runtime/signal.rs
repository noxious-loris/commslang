use super::complex::Complex;

/// Complex baseband signal.
///
/// A signal is represented as a sequence of complex IQ samples.
#[derive(Debug, Clone)]
pub struct Signal {
    pub samples: Vec<Complex>,

    /// Samples per second.
    pub sample_rate: f64,

    /// Center frequency in Hz.
    pub center_frequency: f64,

    /// Occupied bandwidth in Hz.
    pub bandwidth: f64,
}

impl Signal {
    pub fn new(samples: Vec<Complex>, sample_rate: f64) -> Self {
        Self {
            samples,
            sample_rate,
            center_frequency: 0.0,
            bandwidth: 0.0,
        }
    }

    pub fn with_rf_parameters(mut self, center_frequency: f64, bandwidth: f64) -> Self {
        self.center_frequency = center_frequency;
        self.bandwidth = bandwidth;
        self
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Average complex-signal power.
    pub fn average_power(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }

        self.samples
            .iter()
            .map(|sample| sample.magnitude_squared())
            .sum::<f64>()
            / self.samples.len() as f64
    }

    /// RMS signal magnitude.
    pub fn rms(&self) -> f64 {
        self.average_power().sqrt()
    }

    pub fn samples(&self) -> &[Complex] {
        &self.samples
    }

    pub fn samples_mut(&mut self) -> &mut [Complex] {
        &mut self.samples
    }
}
