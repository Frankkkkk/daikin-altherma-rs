use daikin_altherma::DaikinAlthermaClient;
fn main() {
    let mut a = DaikinAlthermaClient::new("192.168.11.100".to_string()).unwrap();
    //    let am = a.get_adapter_model();
    //    println!("Adapter model: {am}");
    //

    //a.set_holiday_mode(false);

    //#   let hm = a.is_holiday_mode();
    //#   println!("Holiday mode: {hm}");

    let tp = a.get_tank_parameters().unwrap();
    println!("Tank: {:?}", tp);

    a.set_tank_powerful(false);

    let tp = a.get_tank_parameters().unwrap();
    println!("Tank: {:?}", tp);

    //let hp = a.get_heating_parameters().unwrap();
    //println!("Heating: {:?}", hp);
}
