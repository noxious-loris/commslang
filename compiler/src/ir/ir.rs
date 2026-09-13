#[derive(Debug, Clone)]
pub struct Module {
    pub systems: Vec<IrSystem>,
}

#[derive(Debug, Clone)]
pub struct IrSystem {
    pub name: String,
    pub components: Vec<IrComponent>,
    pub connections: Vec<IrConnection>,
    pub simulation: Option<IrSimulation>,
}

#[derive(Debug, Clone)]
pub enum IrComponent {
    Signal(IrSignal),
    Channel(IrChannel),
    Receiver(IrReceiver),
    Transmitter(IrTransmitter),
}

#[derive(Debug, Clone)]
pub struct IrSignal {
    pub name: String,
    pub modulation: Option<Modulation>,
    pub power: Option<Quantity>,
}

#[derive(Debug, Clone)]
pub struct IrChannel {
    pub name: String,
    pub model: ChannelModel,
    pub snr: Option<Quantity>,
}

#[derive(Debug, Clone)]
pub struct IrReceiver {
    pub name: String,
    pub demodulation: Option<Modulation>,
}

#[derive(Debug, Clone)]
pub struct IrTransmitter {
    pub name: String,
    pub modulation: Option<Modulation>,
    pub power: Option<Quantity>,
}

#[derive(Debug, Clone)]
pub struct IrConnection {
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct IrSimulation {
    pub bits: u64,
    pub seed: u64,
}

#[derive(Debug, Clone)]
pub struct Quantity {
    pub value: f64,
    pub unit: Unit,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Modulation {
    BPSK,
    QPSK,
    PSK8,
    QAM16,
    QAM64,
    QAM256,
    OFDM,
    FSK,
    GFSK,
    LoRa,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelModel {
    AWGN,
    Rayleigh,
    Rician,
    FreeSpace,
    Multipath,
}
