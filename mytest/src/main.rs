use daikin_altherma::DaikinAlthermaClient;
fn main() {
    let mut a = DaikinAlthermaClient::new("192.168.11.100".to_string());
    let am = a.get_adapter_model();
    println!("Adapter model: {am}");

    let hm = a.is_holiday_mode();
    println!("Holiday mode: {hm}");
}
