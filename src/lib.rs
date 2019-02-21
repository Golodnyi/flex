pub mod flex {
    mod handshake;
    mod bitfield;
    mod telemetry;
    mod head;

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
