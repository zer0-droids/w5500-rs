# w5500-mqtt

MQTT v5 client implementation for use with the W5500.

## Limitations

This is very basic at the moment, and will be expanded in the future.

* Does not support password protected MQTT servers.
* Does not support TLS.
* Does not support unsubscribing.
* Only supports QoS 0: At most once delivery.

## Example

```rust
use w5500_mqtt::{
    ll::{
        net::{Ipv4Addr, SocketAddrV4},
        Sn,
    },
    Client, ClientId, Event, DST_PORT, SRC_PORT,
};

let mut client: Client = Client::new(
    Sn::Sn2,
    SRC_PORT,
    SocketAddrV4::new(Ipv4Addr::new(192, 168, 5, 6), DST_PORT),
);

// wait for a connection or die trying
while client.process(&mut w5500, monotonic_secs())? != Event::None {}

// publish to "duck" with a payload "quack"
client.publish(&mut w5500, "duck", b"quack")?;

// subscribe to "cow"
client.subscribe(&mut w5500, "cow")?;
```

## Relevant Specifications

* [MQTT Version 5.0](https://docs.oasis-open.org/mqtt/mqtt/v5.0/mqtt-v5.0.html)

## Feature Flags

All features are disabled by default.

* `embedded-hal`: Passthrough to [w5500-hl].
* `std`: Passthrough to [w5500-hl].
* `defmt`: Enable logging with `defmt`. Also a passthrough to [w5500-hl].
* `log`: Enable logging with `log`.

[w5500-hl]: https://crates.io/crates/w5500-hl
[`std::net`]: https://doc.rust-lang.org/std/net/index.html
[Wiznet W5500]: https://www.wiznet.io/product-item/w5500/