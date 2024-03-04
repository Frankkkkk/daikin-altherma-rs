use std::fmt::Debug;

#[derive(Debug)]
pub struct TankParameters {
    /// The current temperature of the water tank, in °C
    pub temperature: f64,
    /// The setpoint (wanted) temperature of the water tank, in °C
    pub setpoint_temperature: f64,
    /// Is the tank heating enabled
    pub enabled: bool,
    /// Is it on powerful (quick heating) mode
    pub powerful: bool,
}

#[derive(Debug)]
pub struct HeatingParameters {
    /// The current indoor temperature, in °C
    pub indoor_temperature: f64,
    /// The current outdoor temperature, in °C
    pub outdoor_temperature: f64,
    /// The current indoor setpoint (target) temperature, in °C
    pub indoor_setpoint_temperature: f64,
    /// The leaving water temperature, in °C
    pub leaving_water_temperature: f64,

    /// Is the heating enabled
    pub enabled: bool,

    /// Is the heating on holiday (disabled)
    pub on_holiday: bool,
    // Is it on powerful (quick heating) mode
    //mode: ,
}
