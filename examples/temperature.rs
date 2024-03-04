use daikin_altherma::DaikinAlthermaClient;
fn main() {
    let mut a = DaikinAlthermaClient::new("192.168.11.100".to_string()).unwrap();

    let hp = a.get_heating_parameters().unwrap();
    println!("Heating: {:?}", hp);
    a.set_heating_setpoint_temperature(20.0);
    let hp = a.get_heating_parameters().unwrap();
    println!("Heating: {:?}", hp);
}
