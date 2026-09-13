#[derive(Debug, Clone)]
pub struct Program {
    pub systems: Vec<System>,
}

#[derive(Debug, Clone)]
pub struct System {
    pub name: String,
    pub body: Vec<SystemItem>,
}

#[derive(Debug, Clone)]
pub enum SystemItem {
    Signal(Signal),
    Channel(Channel),
    Receiver(Receiver),
    Transmitter(Transmitter),
    Connect(Connection),
    Simulate(Simulation),
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct Receiver {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct Transmitter {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Simulation {
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Quantity { value: Box<Value>, unit: Unit },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Hz,
    KHz,
    MHz,
    GHz,
    W,
    DB,
    DBm,
}
