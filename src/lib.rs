pub mod flex {
    mod handshake;
    mod bitfield;
    mod telemetry;
    mod head;

    use std::net::TcpListener;

    pub fn handshake(listener: TcpListener) {
        handshake::new(listener);
    }

    pub fn bitfield() {
        bitfield::new();
    }

    pub fn telemetry() {
        telemetry::new();
    }
}
