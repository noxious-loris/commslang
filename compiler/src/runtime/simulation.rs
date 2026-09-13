use crate::ir::{ChannelModel, IrComponent, IrSystem, Modulation};

use super::channel::{apply_channel, calculate_ber, random_bits};

use super::complex::Complex;

use super::modulation::{demodulate, modulate};

/// Result produced by a communication simulation.
#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub system_name: String,

    pub bits: u64,
    pub errors: u64,

    pub ber: f64,

    /// Signal-to-noise ratio.
    pub snr_db: f64,

    /// Energy per bit to noise density ratio.
    pub eb_n0_db: f64,

    /// Bits represented by one modulation symbol.
    pub bits_per_symbol: usize,

    pub modulation: Modulation,
    pub channel: ChannelModel,

    pub transmitted_symbols: usize,
    pub received_symbols: usize,

    /// Random seed used for reproducibility.
    pub seed: u64,
}

/// Execute a complete communication-system simulation.
pub fn simulate_system(system: &IrSystem) -> Result<SimulationResult, String> {
    let simulation = system
        .simulation
        .as_ref()
        .ok_or("System has no simulation block")?;

    let mut modulation = None;
    let mut channel_model = None;
    let mut snr_db = 0.0;

    // ---------------------------------------------------------
    // Discover system components
    // ---------------------------------------------------------

    for component in &system.components {
        match component {
            IrComponent::Signal(signal) => {
                if signal.modulation.is_some() {
                    modulation = signal.modulation.clone();
                }
            }

            IrComponent::Channel(channel) => {
                channel_model = Some(channel.model.clone());

                if let Some(snr) = &channel.snr {
                    snr_db = snr.value;
                }
            }

            IrComponent::Receiver(_) => {}

            IrComponent::Transmitter(transmitter) => {
                if transmitter.modulation.is_some() {
                    modulation = transmitter.modulation.clone();
                }
            }
        }
    }

    let modulation = modulation.ok_or("No modulation specified")?;

    let channel_model = channel_model.ok_or("No channel specified")?;

    let bits_count = simulation.bits as usize;

    if bits_count == 0 {
        return Err("Simulation bit count must be greater than zero".into());
    }

    // ---------------------------------------------------------
    // Determine modulation order
    // ---------------------------------------------------------

    let bits_per_symbol = bits_per_symbol(&modulation);

    // ---------------------------------------------------------
    // Reproducibility
    //
    // Temporary fixed seed.
    //
    // Later this will come from:
    //
    // simulate {
    //     seed = 42
    // }
    // ---------------------------------------------------------

    let seed = 42_u64;

    // ---------------------------------------------------------
    // Generate source bits
    // ---------------------------------------------------------

    let transmitted_bits = random_bits(bits_count, seed);

    // ---------------------------------------------------------
    // Modulation
    // ---------------------------------------------------------

    let transmitted_symbols = modulate(&transmitted_bits, &modulation);

    // ---------------------------------------------------------
    // Channel
    // ---------------------------------------------------------

    let received_symbols = apply_channel(
        &transmitted_symbols,
        &channel_model,
        snr_db,
        seed.wrapping_add(1),
    );

    // ---------------------------------------------------------
    // Demodulation
    // ---------------------------------------------------------

    let mut received_bits = demodulate(&received_symbols, &modulation);

    // The final symbol may contain a padded bit.
    //
    // Never allow the padding bit to participate in BER.
    received_bits.truncate(bits_count);

    // ---------------------------------------------------------
    // BER
    // ---------------------------------------------------------

    let (errors, ber) = calculate_ber(&transmitted_bits, &received_bits);

    // ---------------------------------------------------------
    // Eb/N0
    //
    // For the current normalized symbol model:
    //
    // Es/N0 = SNR
    //
    // Eb/N0 = Es/N0 / bits_per_symbol
    //
    // Therefore:
    //
    // Eb/N0[dB] =
    //     SNR[dB] - 10 log10(bits/symbol)
    // ---------------------------------------------------------

    let eb_n0_db = snr_db - 10.0 * (bits_per_symbol as f64).log10();

    Ok(SimulationResult {
        system_name: system.name.clone(),

        bits: bits_count as u64,
        errors,

        ber,

        snr_db,
        eb_n0_db,

        bits_per_symbol,

        modulation,
        channel: channel_model,

        transmitted_symbols: transmitted_symbols.len(),

        received_symbols: received_symbols.len(),

        seed,
    })
}

/// Determine bits per modulation symbol.
pub fn bits_per_symbol(modulation: &Modulation) -> usize {
    match modulation {
        Modulation::BPSK => 1,

        Modulation::QPSK => 2,

        Modulation::PSK8 => 3,

        Modulation::QAM16 => 4,

        Modulation::QAM64 => 6,

        Modulation::QAM256 => 8,

        Modulation::FSK => 1,

        Modulation::GFSK => 1,

        Modulation::OFDM => 1,

        Modulation::LoRa => 1,
    }
}

/// Compute average complex signal power.
pub fn signal_power(samples: &[Complex]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }

    samples
        .iter()
        .map(|sample| sample.magnitude_squared())
        .sum::<f64>()
        / samples.len() as f64
}

/// Run every simulation in a module.
pub fn simulate_module(module: &crate::ir::Module) -> Result<Vec<SimulationResult>, String> {
    let mut results = Vec::new();

    for system in &module.systems {
        let result = simulate_system(system)?;

        results.push(result);
    }

    Ok(results)
}
