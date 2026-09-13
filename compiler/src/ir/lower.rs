use crate::parser::{
    Channel, Connection, Program, Property, Receiver, Signal, Simulation, System, SystemItem,
    Transmitter, Unit as AstUnit, Value,
};

use super::{
    ChannelModel, IrChannel, IrComponent, IrConnection, IrReceiver, IrSignal, IrSimulation,
    IrSystem, IrTransmitter, Modulation, Module, Quantity, Unit,
};

pub fn lower_program(program: &Program) -> Module {
    let systems = program.systems.iter().map(lower_system).collect();

    Module { systems }
}

fn lower_system(system: &System) -> IrSystem {
    let mut components = Vec::new();
    let mut connections = Vec::new();
    let mut simulation = None;

    for item in &system.body {
        match item {
            SystemItem::Signal(signal) => {
                components.push(IrComponent::Signal(lower_signal(signal)));
            }

            SystemItem::Channel(channel) => {
                components.push(IrComponent::Channel(lower_channel(channel)));
            }

            SystemItem::Receiver(receiver) => {
                components.push(IrComponent::Receiver(lower_receiver(receiver)));
            }

            SystemItem::Transmitter(transmitter) => {
                components.push(IrComponent::Transmitter(lower_transmitter(transmitter)));
            }

            SystemItem::Connect(connection) => {
                connections.push(lower_connection(connection));
            }

            SystemItem::Simulate(sim) => {
                simulation = Some(lower_simulation(sim));
            }
        }
    }

    IrSystem {
        name: system.name.clone(),
        components,
        connections,
        simulation,
    }
}

fn lower_signal(signal: &Signal) -> IrSignal {
    let modulation = find_property(&signal.properties, "modulation").and_then(value_to_modulation);

    let power = find_property(&signal.properties, "power").and_then(value_to_quantity);

    IrSignal {
        name: signal.name.clone(),
        modulation,
        power,
    }
}

fn lower_channel(channel: &Channel) -> IrChannel {
    let model = find_property(&channel.properties, "model")
        .and_then(value_to_channel_model)
        .unwrap_or(ChannelModel::AWGN);

    let snr = find_property(&channel.properties, "snr").and_then(value_to_quantity);

    IrChannel {
        name: channel.name.clone(),
        model,
        snr,
    }
}

fn lower_receiver(receiver: &Receiver) -> IrReceiver {
    let demodulation =
        find_property(&receiver.properties, "demodulation").and_then(value_to_modulation);

    IrReceiver {
        name: receiver.name.clone(),
        demodulation,
    }
}

fn lower_transmitter(transmitter: &Transmitter) -> IrTransmitter {
    let modulation =
        find_property(&transmitter.properties, "modulation").and_then(value_to_modulation);

    let power = find_property(&transmitter.properties, "power").and_then(value_to_quantity);

    IrTransmitter {
        name: transmitter.name.clone(),
        modulation,
        power,
    }
}

fn lower_connection(connection: &Connection) -> IrConnection {
    IrConnection {
        nodes: connection.nodes.clone(),
    }
}

fn lower_simulation(simulation: &Simulation) -> IrSimulation {
    let bits = find_property(&simulation.properties, "bits")
        .and_then(value_to_u64)
        .unwrap_or(1000);

    let seed = find_property(&simulation.properties, "seed")
        .and_then(value_to_u64)
        .unwrap_or(42);

    IrSimulation { bits, seed }
}

fn find_property<'a>(properties: &'a [Property], name: &str) -> Option<&'a Value> {
    properties
        .iter()
        .find(|property| property.name == name)
        .map(|property| &property.value)
}

fn value_to_u64(value: &Value) -> Option<u64> {
    match value {
        Value::Integer(value) if *value >= 0 => Some(*value as u64),
        _ => None,
    }
}

fn value_to_quantity(value: &Value) -> Option<Quantity> {
    match value {
        Value::Quantity { value, unit } => {
            let numeric_value = match value.as_ref() {
                Value::Integer(value) => *value as f64,
                Value::Float(value) => *value,
                _ => return None,
            };

            Some(Quantity {
                value: numeric_value,
                unit: lower_unit(unit),
            })
        }

        _ => None,
    }
}

fn lower_unit(unit: &AstUnit) -> Unit {
    match unit {
        AstUnit::Hz => Unit::Hz,
        AstUnit::KHz => Unit::KHz,
        AstUnit::MHz => Unit::MHz,
        AstUnit::GHz => Unit::GHz,
        AstUnit::W => Unit::W,
        AstUnit::DB => Unit::DB,
        AstUnit::DBm => Unit::DBm,
    }
}

fn value_to_modulation(value: &Value) -> Option<Modulation> {
    let name = match value {
        Value::Identifier(name) => name,
        _ => return None,
    };

    match name.as_str() {
        "BPSK" => Some(Modulation::BPSK),
        "QPSK" => Some(Modulation::QPSK),
        "8PSK" => Some(Modulation::PSK8),
        "16QAM" => Some(Modulation::QAM16),
        "64QAM" => Some(Modulation::QAM64),
        "256QAM" => Some(Modulation::QAM256),
        "OFDM" => Some(Modulation::OFDM),
        "FSK" => Some(Modulation::FSK),
        "GFSK" => Some(Modulation::GFSK),
        "LoRa" => Some(Modulation::LoRa),
        _ => None,
    }
}

fn value_to_channel_model(value: &Value) -> Option<ChannelModel> {
    let name = match value {
        Value::Identifier(name) => name,
        _ => return None,
    };

    match name.as_str() {
        "AWGN" => Some(ChannelModel::AWGN),
        "Rayleigh" => Some(ChannelModel::Rayleigh),
        "Rician" => Some(ChannelModel::Rician),
        "FreeSpace" => Some(ChannelModel::FreeSpace),
        "Multipath" => Some(ChannelModel::Multipath),
        _ => None,
    }
}
