system BrokenLink {

    signal tx {
        modulation = Banana
        power = 5 W
    }

    channel space {
        model = AWGN
        snr = 10 W
    }

    receiver rx {
        demodulation = QPSK
    }

    connect rx -> maybe -> tx

    simulate {
        bits = 1000000
    }
}
