pub mod flex {
    mod handshake;
    mod bitfield;
    mod telemetry;

    pub fn handshake() {
        handshake::new();
    }

    pub fn bitfield() {
        bitfield::new();
    }

    pub fn telemetry() {
        telemetry::new();
    }
}
