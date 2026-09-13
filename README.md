# CommsLang

**CommsLang** is a domain-specific programming language and compiler foundation for modeling, configuring, simulating, and eventually deploying communication systems.

The project is designed around a simple idea:

> **Communication systems should be programmable as systems, not assembled manually from disconnected simulation scripts.**

CommsLang aims to provide a unified language for describing communication-system components such as transmitters, signals, channels, receivers, modulation schemes, noise models, protocols, DSP pipelines, SDR systems, embedded targets, and hardware-oriented implementations.

The current release, **CommsLang v1.0.0**, establishes the core compiler architecture and a working end-to-end communication-system simulation pipeline.

---

## Project Status

**Current version:** `0.1.0` / v1 baseline

**Status:** Working compiler foundation and simulation runtime

The current implementation supports:

* CommsLang source files
* Lexical analysis
* Tokenization
* Parsing
* Abstract Syntax Tree generation
* Semantic analysis
* Symbol checking
* Domain-specific validation
* Intermediate Representation (IR)
* IR lowering
* Complex-number signal representation
* Digital modulation
* BPSK
* QPSK
* AWGN channel simulation
* Deterministic random generation
* BER calculation
* Communication-system simulation
* Command-line interface
* Project creation
* Project auto-discovery
* Source checking
* IR generation

The architecture is intentionally designed so that additional communication-system capabilities can be added without replacing the compiler foundation.

---

# Why CommsLang?

Communication-system development normally involves many different tools and programming environments.

A typical workflow might involve:

* Python for simulation
* MATLAB/Octave for DSP
* C/C++ for embedded systems
* GNU Radio for SDR
* Verilog/SystemVerilog for FPGA implementations
* KiCad for hardware
* separate configuration files
* separate protocol definitions
* separate test scripts

This creates a large gap between **system-level design** and **implementation**.

CommsLang is intended to eventually provide a common abstraction layer.

A communication system should be describable directly:

```text
system SatelliteLink {

    signal tx {
        modulation = QPSK
        power = 5 W
    }

    channel space {
        model = AWGN
        snr = 10 dB
    }

    receiver rx {
        demodulation = QPSK
    }

    connect tx -> space -> rx

    simulate {
        bits = 1000000
        seed = 42
    }
}
```

The compiler can then transform this description into an internal representation suitable for simulation, analysis, testing, or eventually code generation.

---

# Design Philosophy

CommsLang is being developed around several principles.

## 1. Domain-first

The language should understand communication-system concepts directly.

Instead of writing:

```text
some_function(...)
```

a CommsLang program should be able to express:

```text
signal tx
channel space
receiver rx
```

and eventually:

```text
modulation
filter
spectrum
noise
packet
frame
protocol
antenna
link
```

---

## 2. Strong semantic validation

Communication systems have domain-specific constraints.

For example:

```text
power = 5 W
```

and:

```text
snr = 10 dB
```

are not simply arbitrary numbers.

The compiler should eventually understand units, types, physical quantities, and valid combinations of parameters.

The semantic-analysis layer already provides the foundation for this.

---

## 3. Compiler architecture

CommsLang follows a conventional compiler architecture:

```text
                 CommsLang Source
                        |
                        v
                     Lexer
                        |
                        v
                     Tokens
                        |
                        v
                     Parser
                        |
                        v
                       AST
                        |
                        v
              Semantic Analysis
                        |
                        v
                       IR
                        |
             +----------+----------+
             |                     |
             v                     v
          Runtime              Codegen
             |                     |
             v                     v
        Simulation          Future Targets
```

The separation between these stages is intentional.

The language frontend should not need to know how a particular simulation backend or hardware target works.

---

# Compiler Architecture

The compiler is currently organized into several major components.

```text
compiler/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── compiler.rs
    │
    ├── cli/
    │   ├── mod.rs
    │   ├── commands.rs
    │   ├── project.rs
    │   └── output.rs
    │
    ├── lexer/
    │   ├── mod.rs
    │   ├── token.rs
    │   ├── lexer.rs
    │   └── error.rs
    │
    ├── parser/
    │   ├── mod.rs
    │   ├── ast.rs
    │   ├── parser.rs
    │   └── error.rs
    │
    ├── semantic/
    │   ├── mod.rs
    │   ├── analyzer.rs
    │   └── error.rs
    │
    ├── ir/
    │   ├── mod.rs
    │   ├── ir.rs
    │   └── lower.rs
    │
    └── runtime/
        ├── mod.rs
        ├── complex.rs
        ├── signal.rs
        ├── channel.rs
        ├── modulation.rs
        └── simulation.rs
```

---

# Lexer

The lexer converts raw CommsLang source code into tokens.

For example:

```text
power = 5 W
```

becomes a sequence conceptually similar to:

```text
Identifier("power")
Assign
Integer(5)
W
```

The lexer currently supports:

* identifiers
* keywords
* integers
* floating-point numbers
* scientific notation
* strings
* comments
* units
* operators
* punctuation
* connection arrows
* line tracking
* column tracking
* EOF
* lexical error reporting

### Comments

Single-line comments:

```text
// transmission power
```

Multi-line comments:

```text
/*
   communication
   system configuration
*/
```

---

# Keywords

The current language foundation includes keywords such as:

```text
system
signal
channel
receiver
transmitter
connect
simulate
library
function
if
else
for
while
return
const
let
true
false
```

Some of these are already recognized by the lexer while their complete language semantics are planned for later versions.

---

# Units

The lexer currently recognizes communication-oriented units including:

```text
Hz
kHz
MHz
GHz
W
dB
dBm
```

Example:

```text
power = 5 W
snr = 10 dB
```

The AST represents these as quantities containing a numerical value and unit.

More extensive unit support is planned.

Future units include concepts such as:

```text
ms
us
ns
V
A
m
km
bps
kbps
Mbps
Gbps
rad
```

as the language develops.

---

# Parser

The parser converts tokens into an Abstract Syntax Tree.

The current AST contains concepts including:

* programs
* systems
* signals
* channels
* receivers
* transmitters
* connections
* simulations
* properties
* values
* quantities
* units

For example:

```text
system SatelliteLink {
    ...
}
```

is represented internally as a `System`.

A declaration such as:

```text
signal tx {
    modulation = QPSK
    power = 5 W
}
```

becomes a signal node with properties.

---

# Abstract Syntax Tree

The current AST contains the following major structures:

```text
Program
 └── System
      ├── Signal
      ├── Channel
      ├── Receiver
      ├── Transmitter
      ├── Connection
      └── Simulation
```

Values currently include:

```text
Identifier
Integer
Float
String
Boolean
Quantity
```

Quantities contain:

```text
value
unit
```

This creates the foundation for future type- and unit-aware compilation.

---

# Semantic Analysis

Parsing only determines whether the source has valid syntax.

Semantic analysis determines whether the program makes sense.

For example, the following is syntactically valid:

```text
channel space {
    model = UnknownChannel
}
```

but `UnknownChannel` is not currently a valid channel model.

The semantic analyzer therefore rejects it.

---

## Symbol Checking

Components are registered in a symbol table.

Duplicate names are rejected.

For example:

```text
signal tx {
    modulation = QPSK
}

signal tx {
    modulation = BPSK
}
```

produces a duplicate-component error.

Connections are also checked.

For:

```text
connect tx -> space -> rx
```

the compiler verifies that:

```text
tx
space
rx
```

are actually defined.

---

# Supported Modulation Models

The language currently recognizes:

```text
BPSK
QPSK
8PSK
16QAM
64QAM
256QAM
OFDM
FSK
GFSK
LoRa
```

The runtime currently implements actual digital modulation for:

```text
BPSK
QPSK
```

The remaining modulation identifiers are part of the language's domain vocabulary and are planned for runtime implementation.

---

# Supported Channel Models

The language currently recognizes:

```text
AWGN
Rayleigh
Rician
FreeSpace
Multipath
```

The current runtime implements:

```text
AWGN
```

Additional channel models are planned for subsequent releases.

---

# Intermediate Representation

After semantic analysis, the AST is lowered into an Intermediate Representation.

The IR provides a boundary between the language frontend and execution backends.

The current IR contains:

```text
Module
 └── IrSystem
      ├── IrComponent
      │    ├── IrSignal
      │    ├── IrChannel
      │    ├── IrReceiver
      │    └── IrTransmitter
      │
      ├── IrConnection
      │
      └── IrSimulation
```

The IR also has explicit representations for:

* quantities
* units
* modulations
* channel models
* simulation parameters

This separation will become increasingly important when CommsLang gains multiple execution backends.

---

# Runtime

The runtime contains the actual communication-system execution primitives.

Current runtime components include:

```text
Complex
Signal
Modulator
Demodulator
Channel
AWGN channel
Simulation
```

---

# Complex Numbers

Communication systems operate heavily in the complex domain.

CommsLang therefore provides its own runtime `Complex` type.

It currently supports operations including:

* addition
* subtraction
* multiplication
* scaling
* conjugation
* magnitude
* magnitude squared
* phase

This provides the foundation for:

* IQ signals
* modulation
* demodulation
* filtering
* Fourier transforms
* RF simulation
* SDR processing

---

# Signal Representation

The runtime signal representation contains:

```text
samples
sample_rate
center_frequency
bandwidth
```

Samples are represented as complex values.

Conceptually:

```text
Signal
 ├── Complex samples
 ├── Sample rate
 ├── Center frequency
 └── Bandwidth
```

Future versions will expand this into a more complete DSP signal abstraction.

---

# Digital Modulation

The runtime currently provides a modulation interface through traits.

Conceptually:

```text
bits
  |
  v
Modulator
  |
  v
complex symbols
```

The current implementation includes:

## BPSK

Binary Phase Shift Keying maps individual bits to two constellation points.

## QPSK

Quadrature Phase Shift Keying maps two bits to each symbol.

The current QPSK implementation uses Gray-coded symbol mapping and normalized constellation points.

---

# AWGN Channel

The current runtime includes an Additive White Gaussian Noise channel.

Conceptually:

```text
Transmitted signal
       |
       v
      AWGN
       |
       v
Received signal
```

The channel accepts an SNR parameter in dB.

Example:

```text
channel space {
    model = AWGN
    snr = 10 dB
}
```

The simulation runtime uses deterministic random seeds so that experiments can be reproduced.

---

# Deterministic Simulation

Simulation configuration can specify:

```text
simulate {
    bits = 100000
    seed = 42
}
```

The seed makes the random experiment reproducible.

This is important for:

* debugging
* regression testing
* benchmarking
* comparing compiler/runtime changes
* research experiments

---

# BER Calculation

The current runtime calculates Bit Error Rate.

The simulation pipeline is approximately:

```text
Random bits
     |
     v
Modulator
     |
     v
Complex symbols
     |
     v
AWGN channel
     |
     v
Received symbols
     |
     v
Demodulator
     |
     v
Received bits
     |
     v
BER calculation
```

The simulation result contains information including:

* system name
* number of bits
* bit errors
* BER
* SNR
* Eb/N0
* modulation
* channel
* transmitted symbols
* received symbols
* random seed

---

# Command Line Interface

CommsLang is currently designed as a CLI-first tool.

The command structure is:

```text
commslang <COMMAND>
```

Available commands:

```text
new
run
check
build
version
```

---

# Creating a Project

Create a new CommsLang project:

```bash
commslang new MyProject
```

This creates a project containing the basic CommsLang source and configuration.

The generated project includes:

```text
MyProject/
├── main.cl
├── commslang.toml
├── README.md
├── src/
└── examples/
```

---

# Checking a Program

Check a source file without running the simulation:

```bash
commslang check main.cl
```

Inside a CommsLang project:

```bash
commslang check
```

Successful validation reports that the program is valid.

The check operation runs the compiler frontend:

```text
Source
  |
Lexer
  |
Parser
  |
Semantic Analyzer
  |
Success
```

---

# Running a Program

Run a CommsLang program:

```bash
commslang run main.cl
```

Or inside a project:

```bash
commslang run
```

The compiler will:

1. Read the source
2. Lex the source
3. Parse the source
4. Perform semantic analysis
5. Generate IR
6. Execute the simulation
7. Display the results

---

# Building a Program

Generate and display the current IR:

```bash
commslang build main.cl
```

Or:

```bash
commslang build
```

The build pipeline currently ends at IR generation.

Future versions will use this stage for:

* optimization
* simulation backend selection
* native code generation
* embedded code generation
* FPGA generation
* SDR backend generation

---

# Version Information

Check the installed CommsLang version:

```bash
commslang version
```

or:

```bash
commslang --version
```

---

# Example Program

A complete current example:

```text
system SatelliteLink {

    signal tx {
        modulation = QPSK
        power = 5 W
    }

    channel space {
        model = AWGN
        snr = 10 dB
    }

    receiver rx {
        demodulation = QPSK
    }

    connect tx -> space -> rx

    simulate {
        bits = 1000000
        seed = 42
    }
}
```

This represents:

```text
              AWGN
       ┌────────────────┐
       │                │
       v                │
TX ────────────────> Channel ────────────────> RX
QPSK                  10 dB                    QPSK
```

The runtime generates random bits, modulates them using QPSK, passes the symbols through AWGN, demodulates the received symbols, and calculates BER.

---

# Example Simulation Result

A successful simulation produces information similar to:

```text
========================================
       CommsLang Simulation Result
========================================

System:
  SatelliteLink

Modulation:
  QPSK

Bits / symbol:
  2

Channel:
  AWGN

Channel SNR:
  10.00 dB

Eb/N0:
  6.99 dB

Simulation:
  Bits:                 1000000
  Transmitted symbols:  500000
  Received symbols:     500000
  Random seed:          42

Results:
  Bit errors:           764
  BER:                  0.00076400

========================================
Simulation completed.
========================================
```

Exact numerical results depend on the implementation and deterministic random sequence used by the runtime.

---

# Building From Source

CommsLang is written in Rust.

Install a current Rust toolchain and Cargo.

Clone the repository:

```bash
git clone <repository-url>
cd commslang
```

Enter the compiler directory:

```bash
cd compiler
```

Build:

```bash
cargo build
```

For a release build:

```bash
cargo build --release
```

Run tests:

```bash
cargo test
```

Format the code:

```bash
cargo fmt
```

Check formatting:

```bash
cargo fmt -- --check
```

---

# Installing the CLI Locally

From the compiler directory:

```bash
cargo install --path .
```

Then verify:

```bash
commslang --version
```

---

# Technology Stack

CommsLang currently uses:

* **Rust** — compiler and runtime implementation
* **Cargo** — build and package management
* **Clap** — command-line interface
* **rand** — deterministic random number generation
* **rand_distr** — statistical distributions and Gaussian noise

The project intentionally keeps the core implementation relatively small and modular.

---

# Repository Structure

The current repository is organized around the compiler and examples.

```text
commslang/
│
├── compiler/
│   ├── Cargo.toml
│   └── src/
│       ├── cli/
│       ├── lexer/
│       ├── parser/
│       ├── semantic/
│       ├── ir/
│       ├── runtime/
│       ├── compiler.rs
│       ├── lib.rs
│       └── main.rs
│
└── examples/
    └── test_lexer.cl
```

Additional directories and runtime libraries will be introduced as the project expands.

---

# Development Roadmap

CommsLang is intended to evolve significantly beyond the current baseline.

## v1 — Compiler Foundation

Completed foundation:

* [x] Lexer
* [x] Token system
* [x] Parser
* [x] AST
* [x] Semantic analysis
* [x] Symbol validation
* [x] Domain validation
* [x] IR
* [x] AST → IR lowering
* [x] Complex numbers
* [x] Signal representation
* [x] BPSK runtime
* [x] QPSK runtime
* [x] AWGN runtime
* [x] BER calculation
* [x] Deterministic simulation
* [x] CLI
* [x] Project generator
* [x] Project discovery
* [x] `check`
* [x] `run`
* [x] `build`

---

## v0.2 — Language Expansion

Planned:

* [ ] `const`
* [ ] `let`
* [ ] expressions
* [ ] arithmetic
* [ ] expression evaluation
* [ ] constant propagation
* [ ] stronger type system
* [ ] unit compatibility checking
* [ ] additional physical units
* [ ] improved diagnostics

Example target:

```text
const tx_power = 5 W
const channel_snr = 10 dB
const simulation_bits = 1000000
```

---

## v0.3 — Modulation

Planned runtime implementations:

* [ ] 8PSK
* [ ] 16QAM
* [ ] 64QAM
* [ ] 256QAM
* [ ] FSK
* [ ] GFSK
* [ ] OFDM
* [ ] LoRa-oriented modulation primitives

Additional goals:

* [ ] constellation representation
* [ ] symbol mapping
* [ ] demapping
* [ ] modulation metrics
* [ ] EVM
* [ ] constellation analysis

---

## v0.4 — Channel Models

Planned:

* [ ] Rayleigh fading
* [ ] Rician fading
* [ ] Free-space propagation
* [ ] Multipath
* [ ] Doppler
* [ ] path loss
* [ ] shadowing
* [ ] configurable channel parameters

---

## v0.5 — DSP

Planned DSP primitives include:

* [ ] FIR filters
* [ ] IIR filters
* [ ] FFT
* [ ] IFFT
* [ ] convolution
* [ ] correlation
* [ ] resampling
* [ ] interpolation
* [ ] decimation
* [ ] mixers
* [ ] oscillators
* [ ] spectrum analysis
* [ ] IQ processing

The goal is eventually to make DSP pipelines first-class language constructs.

---

## v0.6 — Error-Control Coding

Planned:

* [ ] Hamming
* [ ] Reed-Solomon
* [ ] convolutional coding
* [ ] Turbo codes
* [ ] LDPC
* [ ] Polar codes
* [ ] encoder/decoder abstractions
* [ ] coding gain analysis

---

## v0.7 — RF and Link Budgets

Planned:

* [ ] frequency
* [ ] bandwidth
* [ ] antenna gain
* [ ] transmit power
* [ ] receiver sensitivity
* [ ] path loss
* [ ] noise figure
* [ ] thermal noise
* [ ] link budget
* [ ] carrier-to-noise ratio
* [ ] Eb/N0
* [ ] SNR
* [ ] RF chain modeling

Example target:

```text
link SatelliteToGround {

    frequency = 2.2 GHz
    bandwidth = 20 MHz

    transmitter tx {
        power = 20 W
        gain = 12 dBi
    }

    receiver rx {
        gain = 18 dBi
        noise_figure = 2 dB
    }
}
```

---

## v0.8 — Protocols and Data

Planned abstractions:

* [ ] bits
* [ ] bytes
* [ ] packets
* [ ] frames
* [ ] headers
* [ ] payloads
* [ ] CRC
* [ ] checksums
* [ ] serialization
* [ ] protocol state machines
* [ ] addressing
* [ ] networking

This stage will bridge physical-layer simulation and higher communication layers.

---

## v0.9 — SDR

Long-term SDR integration goals include:

* [ ] SDR device abstraction
* [ ] transmit streams
* [ ] receive streams
* [ ] IQ interfaces
* [ ] sample-rate configuration
* [ ] center-frequency configuration
* [ ] hardware-backed execution
* [ ] GNU Radio interoperability
* [ ] SoapySDR interoperability
* [ ] SDR testing

Possible target syntax:

```text
radio HackRF {

    frequency = 433 MHz
    sample_rate = 2 MHz
    bandwidth = 1 MHz

    transmit tx
    receive rx
}
```

---

# v1.0+ — Hardware and Code Generation

Long-term goals include compiling communication-system descriptions into implementation targets.

Potential targets include:

```text
CommsLang
    |
    +── Simulation
    |
    +── Native
    |
    +── C
    |
    +── C++
    |
    +── Rust
    |
    +── Embedded
    |
    +── FPGA
    |
    +── SDR
    |
    +── GPU
```

Potential hardware-oriented targets include:

* microcontrollers
* embedded Linux systems
* DSP processors
* FPGA
* SoC
* SDR hardware
* RISC-V systems

---

# Future Hardware Description

One long-term objective is to allow a communication system to move from simulation toward implementation without rewriting the entire system.

For example:

```text
system Radio {

    signal tx {
        modulation = QPSK
    }

    channel channel {
        model = AWGN
    }

    receiver rx {
        demodulation = QPSK
    }

    connect tx -> channel -> rx
}
```

could eventually have multiple execution interpretations:

```text
simulation
```

```text
native software
```

```text
embedded implementation
```

```text
FPGA implementation
```

The IR is the architectural boundary that makes this possible.

---

# Future Communication-System Domains

The long-term scope of CommsLang includes:

### Physical Layer

* modulation
* demodulation
* coding
* synchronization
* equalization
* channel estimation
* waveform generation

### RF

* carrier frequency
* bandwidth
* antennas
* propagation
* noise
* link budgets
* RF chains

### DSP

* filters
* FFT
* convolution
* correlation
* resampling
* spectral analysis

### Networking

* packets
* frames
* addressing
* routing
* protocol state machines

### Wireless

* cellular systems
* Wi-Fi
* satellite communications
* IoT
* LPWAN
* mesh networks

### SDR

* IQ streams
* SDR hardware
* real-time processing
* radio configuration

### Embedded

* microcontrollers
* embedded Linux
* hardware peripherals
* real-time execution

### FPGA / Hardware

* RTL generation
* hardware pipelines
* FPGA acceleration
* SoC integration

---

# Testing Philosophy

Communication-system software requires more than conventional compiler tests.

CommsLang is intended to eventually test three layers simultaneously:

```text
Language correctness
        +
Compiler correctness
        +
Communication-system correctness
```

Examples include:

### Compiler tests

```text
source
  -> lexer
  -> parser
  -> semantic analyzer
  -> IR
```

### Runtime tests

```text
bits
  -> modulation
  -> channel
  -> demodulation
  -> BER
```

### System-level tests

```text
communication system
  -> simulation
  -> expected performance
```

Future statistical regression tests will compare metrics such as:

* BER
* SNR
* EVM
* throughput
* packet error rate
* latency
* spectral efficiency

---

# Reproducibility

Simulation reproducibility is an important design goal.

CommsLang simulations can specify a seed:

```text
simulate {
    bits = 1000000
    seed = 42
}
```

This allows experiments to be repeated with the same random sequence.

Reproducibility will become increasingly important for:

* research
* benchmarking
* regression testing
* algorithm comparison
* communication-system optimization

---

# Error Handling

CommsLang separates errors according to compiler stages.

Current categories include:

```text
Lexer error
Parser error
Semantic error
```

Errors contain useful source-location information where applicable.

Example:

```text
Lexer error: Invalid character at 4:12
```

The diagnostic system will be expanded in future releases to provide:

* source snippets
* highlighted locations
* suggestions
* expected-token messages
* type errors
* unit mismatch explanations
* domain-specific diagnostics

---

# Development Principles

The project follows several development principles.

## Keep the compiler modular

Lexer, parser, semantic analysis, IR, and runtime should remain independently understandable.

## Keep domain concepts explicit

A communication system should be represented by communication concepts rather than generic data structures wherever practical.

## Make simulations reproducible

Randomized experiments should support deterministic seeds.

## Separate description from execution

The language describes a system.

The runtime executes it.

The IR connects the two.

## Build incrementally

New features should be added without destabilizing previously working functionality.

---

# Current Limitations

The v1 baseline is intentionally incomplete.

Currently:

* only a subset of modulation schemes execute
* only AWGN is implemented as a runtime channel
* the expression system is limited
* constants are not yet fully implemented
* the type system is minimal
* unit conversion is limited
* DSP primitives are not yet implemented
* coding schemes are not yet implemented
* RF propagation is not yet implemented
* protocol modeling is not yet implemented
* SDR integration is not yet implemented
* hardware/FPGA code generation is not yet implemented

These are planned development areas rather than failures of the current baseline.

---

# Contributing

Contributions should preserve the separation between:

```text
Language Frontend
Compiler IR
Runtime
Execution Backends
```

When adding a feature, consider whether it belongs in:

```text
lexer/
parser/
semantic/
ir/
runtime/
cli/
```

rather than placing all functionality into a single compiler module.

---

# Project Vision

The long-term vision of CommsLang is to become a general-purpose programming environment specifically designed for communication systems.

The intended progression is:

```text
Describe
   ↓
Validate
   ↓
Compile
   ↓
Simulate
   ↓
Analyze
   ↓
Test
   ↓
Optimize
   ↓
Generate
   ↓
Deploy
```

A single communication-system description should eventually be capable of moving across abstraction levels:

```text
Mathematical Model
        ↓
System Model
        ↓
Simulation
        ↓
DSP
        ↓
Protocol
        ↓
Software
        ↓
Embedded
        ↓
FPGA / Hardware
        ↓
Physical Radio
```

The goal is not simply to create another simulation scripting language.

The goal is to create a **programmable systems language for communication engineering**.

---

# License

License information will be added as the project reaches its public release stage.

---

# Author

**Jyotishka Chattopadhyay**

CommsLang is being developed as an independent communication-systems programming language and compiler project.

---

# Version History

## v1.0.0 — Compiler Foundation

Initial frozen baseline containing:

* lexer
* parser
* AST
* semantic analyzer
* IR
* runtime
* complex-number support
* BPSK
* QPSK
* AWGN
* BER simulation
* deterministic seeds
* CLI
* project generation
* project discovery
* source checking
* simulation execution
* IR generation

---

## Roadmap Summary

```text
v1.0  Compiler Foundation       [CURRENT]
  |
v0.2  Expressions + Units
  |
v0.3  Modulation
  |
v0.4  Channels
  |
v0.5  DSP
  |
v0.6  Error Correction
  |
v0.7  RF + Link Budget
  |
v0.8  Protocols + Packets
  |
v0.9  SDR
  |
v1.x  Code Generation
  |
  +---- Embedded
  +---- FPGA
  +---- Native
  +---- SDR
  +---- Hardware
```

---

**CommsLang — Communication Systems, Programmable.**
