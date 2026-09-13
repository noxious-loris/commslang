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
    }
}
