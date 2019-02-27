extern crate serialport;
extern crate tokio;

pub mod flex {
    mod handshake;
    mod bitfield;
    mod telemetry;
    mod head;

    use serialport::SerialPort;
    use tokio::net::tcp::Incoming;

    struct Handshake;

    trait OverloadedHandshake<T> {
        fn OverloadedHandshake(&self, listener: T);
    }

    impl Handshake {
        fn new<T>(&self, listener: T) where Self: OverloadedHandshake<T> {
            self.OverloadedHandshake(listener)
        }
    }

    impl OverloadedHandshake<Incoming> for Handshake {
        fn OverloadedHandshake(&self, listener: Incoming) {
            handshake::socket(listener);
        }
    }

    impl OverloadedHandshake<&SerialPort> for Handshake {
            fn OverloadedHandshake(&self, listener: &SerialPort) {
            handshake::com_port(listener);
        }
    }

    pub fn bitfield() {
        bitfield::new();
    }

    pub fn telemetry() {
        telemetry::new();
    }
}
