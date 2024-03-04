# Daikin Altherma Rust client

This project interfaces with Daikin Altherma LAN adapters (BRP069A61 BRP069A62).

This is a Rust port of the excellent [python-daikin-altherma](https://github.com/Frankkkkk/python-daikin-altherma)

<!-- cargo-rdme start -->

## API to Daikin Altherma LAN Adapters

This rust crate interfaces with Daikin Altherma LAN adapteros.

There are two firmware versions of the LAN adapters:
- Cloud connected
- LAN only

This library only supports the second one for the moment.
### Usage
Using this library is rather easy:
```rust
  let mut client = DaikinAlthermaClient::new("192.168.11.100".to_string()).unwrap();

  let hp = client.get_heating_parameters().unwrap();
  println!("Heating: {:?}", hp);

  client.set_heating_setpoint_temperature(20.0);
```

<!-- cargo-rdme end -->

