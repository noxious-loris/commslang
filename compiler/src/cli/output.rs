use crate::runtime::SimulationResult;

/// Print simulation results.
pub fn print_simulation_results(results: &[SimulationResult]) {
    for result in results {
        println!("========================================");

        println!("       CommsLang Simulation Result");

        println!("========================================");

        println!();

        println!("System:");
        println!("  {}", result.system_name);

        println!();

        println!("Modulation:");
        println!("  {:?}", result.modulation);

        println!("Bits / symbol:");
        println!("  {}", result.bits_per_symbol);

        println!();

        println!("Channel:");
        println!("  {:?}", result.channel);

        println!();

        println!("Channel SNR:");
        println!("  {:.2} dB", result.snr_db);

        println!("Eb/N0:");
        println!("  {:.2} dB", result.eb_n0_db);

        println!();

        println!("Simulation:");

        println!("  Bits:                 {}", result.bits);

        println!("  Transmitted symbols:  {}", result.transmitted_symbols);

        println!("  Received symbols:     {}", result.received_symbols);

        println!("  Random seed:          {}", result.seed);

        println!();

        println!("Results:");

        println!("  Bit errors:           {}", result.errors);

        println!("  BER:                  {:.8}", result.ber);

        println!();

        println!("========================================");

        println!("Simulation completed.");

        println!("========================================");
    }
}

/// Print a successful check result.
pub fn print_check_success(file: &std::path::Path) {
    println!("✓ {} is valid CommsLang.", file.display());
}
