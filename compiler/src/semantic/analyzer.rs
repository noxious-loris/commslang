use std::collections::HashSet;

use crate::parser::{
    Channel, Connection, Program, Property, Receiver, Signal, Simulation, System, SystemItem,
    Transmitter, Unit, Value,
};

use super::error::SemanticError;

pub struct SemanticAnalyzer {
    symbols: HashSet<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbols: HashSet::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), SemanticError> {
        for system in &program.systems {
            self.analyze_system(system)?;
        }

        Ok(())
    }

    fn analyze_system(&mut self, system: &System) -> Result<(), SemanticError> {
        self.symbols.clear();

        for item in &system.body {
            match item {
                SystemItem::Signal(signal) => {
                    self.register_component(&signal.name)?;
                    self.analyze_signal(signal)?;
                }

                SystemItem::Channel(channel) => {
                    self.register_component(&channel.name)?;
                    self.analyze_channel(channel)?;
                }

                SystemItem::Receiver(receiver) => {
                    self.register_component(&receiver.name)?;
                    self.analyze_receiver(receiver)?;
                }

                SystemItem::Transmitter(transmitter) => {
                    self.register_component(&transmitter.name)?;
                    self.analyze_transmitter(transmitter)?;
                }

                SystemItem::Connect(connection) => {
                    self.analyze_connection(connection)?;
                }

                SystemItem::Simulate(simulation) => {
                    self.analyze_simulation(simulation)?;
                }
            }
        }

        Ok(())
    }

    fn register_component(&mut self, name: &str) -> Result<(), SemanticError> {
        if !self.symbols.insert(name.to_string()) {
            return Err(SemanticError {
                message: format!("Duplicate component name '{}'", name),
            });
        }

        Ok(())
    }

    fn analyze_signal(&self, signal: &Signal) -> Result<(), SemanticError> {
        for property in &signal.properties {
            match property.name.as_str() {
                "modulation" => {
                    self.validate_modulation(&property.value)?;
                }

                "power" => {
                    self.validate_unit(property, &[Unit::W, Unit::DBm])?;
                }

                _ => {
                    return Err(SemanticError {
                        message: format!("Unknown signal property '{}'", property.name),
                    });
                }
            }
        }

        Ok(())
    }

    fn analyze_channel(&self, channel: &Channel) -> Result<(), SemanticError> {
        let mut has_model = false;

        for property in &channel.properties {
            match property.name.as_str() {
                "model" => {
                    has_model = true;
                    self.validate_channel_model(&property.value)?;
                }

                "snr" => {
                    self.validate_unit(property, &[Unit::DB])?;
                }

                _ => {
                    return Err(SemanticError {
                        message: format!("Unknown channel property '{}'", property.name),
                    });
                }
            }
        }

        if !has_model {
            return Err(SemanticError {
                message: format!("Channel '{}' must specify a model", channel.name),
            });
        }

        Ok(())
    }

    fn analyze_receiver(&self, receiver: &Receiver) -> Result<(), SemanticError> {
        for property in &receiver.properties {
            match property.name.as_str() {
                "demodulation" => {
                    self.validate_modulation(&property.value)?;
                }

                _ => {
                    return Err(SemanticError {
                        message: format!("Unknown receiver property '{}'", property.name),
                    });
                }
            }
        }

        Ok(())
    }

    fn analyze_transmitter(&self, transmitter: &Transmitter) -> Result<(), SemanticError> {
        for property in &transmitter.properties {
            match property.name.as_str() {
                "modulation" => {
                    self.validate_modulation(&property.value)?;
                }

                "power" => {
                    self.validate_unit(property, &[Unit::W, Unit::DBm])?;
                }

                _ => {
                    return Err(SemanticError {
                        message: format!("Unknown transmitter property '{}'", property.name),
                    });
                }
            }
        }

        Ok(())
    }

    fn analyze_connection(&self, connection: &Connection) -> Result<(), SemanticError> {
        if connection.nodes.len() < 2 {
            return Err(SemanticError {
                message: "A connection must contain at least two components".to_string(),
            });
        }

        for node in &connection.nodes {
            if !self.symbols.contains(node) {
                return Err(SemanticError {
                    message: format!("Connection references undefined component '{}'", node),
                });
            }
        }

        Ok(())
    }

    fn analyze_simulation(&self, simulation: &Simulation) -> Result<(), SemanticError> {
        for property in &simulation.properties {
            match property.name.as_str() {
                "bits" => match &property.value {
                    Value::Integer(value) if *value > 0 => {}

                    _ => {
                        return Err(SemanticError {
                            message: "Simulation 'bits' must be a positive integer".to_string(),
                        });
                    }
                },

                "seed" => match &property.value {
                    Value::Integer(value) if *value >= 0 => {}

                    _ => {
                        return Err(SemanticError {
                            message: "Simulation 'seed' must be a non-negative integer".to_string(),
                        });
                    }
                },

                _ => {
                    return Err(SemanticError {
                        message: format!("Unknown simulation property '{}'", property.name),
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_modulation(&self, value: &Value) -> Result<(), SemanticError> {
        let name = match value {
            Value::Identifier(name) => name,
            _ => {
                return Err(SemanticError {
                    message: "Modulation must be an identifier".to_string(),
                });
            }
        };

        let valid = matches!(
            name.as_str(),
            "BPSK"
                | "QPSK"
                | "8PSK"
                | "16QAM"
                | "64QAM"
                | "256QAM"
                | "OFDM"
                | "FSK"
                | "GFSK"
                | "LoRa"
        );

        if !valid {
            return Err(SemanticError {
                message: format!("Unknown modulation '{}'", name),
            });
        }

        Ok(())
    }

    fn validate_channel_model(&self, value: &Value) -> Result<(), SemanticError> {
        let name = match value {
            Value::Identifier(name) => name,
            _ => {
                return Err(SemanticError {
                    message: "Channel model must be an identifier".to_string(),
                });
            }
        };

        let valid = matches!(
            name.as_str(),
            "AWGN" | "Rayleigh" | "Rician" | "FreeSpace" | "Multipath"
        );

        if !valid {
            return Err(SemanticError {
                message: format!("Unknown channel model '{}'", name),
            });
        }

        Ok(())
    }

    fn validate_unit(
        &self,
        property: &Property,
        allowed_units: &[Unit],
    ) -> Result<(), SemanticError> {
        let unit = match &property.value {
            Value::Quantity { unit, .. } => unit,
            _ => {
                return Err(SemanticError {
                    message: format!("Property '{}' must have a unit", property.name),
                });
            }
        };

        if !allowed_units.contains(unit) {
            return Err(SemanticError {
                message: format!("Invalid unit for property '{}'", property.name),
            });
        }

        Ok(())
    }
}
