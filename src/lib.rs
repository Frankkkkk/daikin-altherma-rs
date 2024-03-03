use std::{fmt::Debug, net::TcpStream};
use thiserror::Error;
use uuid::Uuid;

use serde_json::{json, Value};
use tungstenite::{connect, stream, Message, WebSocket};
use url::Url;

#[derive(Error, Debug)]
pub enum DAError {
    #[error("Communication error")]
    CommunicationError,
    #[error("Conversion error")]
    ConversionError,
    #[error("Set value error")]
    SetValueError(String),
    #[error("No such field")]
    NoSuchFieldError,
}

pub struct DaikinAlthermaClient {
    ws_client: WebSocket<stream::MaybeTlsStream<TcpStream>>,
}

//
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

trait FromJsonValue<T>: Sized {
    fn from_json_value(value: &Value) -> Result<T, DAError>;
}

// Implement the trait for i64
impl FromJsonValue<i64> for i64 {
    fn from_json_value(value: &Value) -> Result<Self, DAError> {
        let v: Option<i64> = value.as_i64();
        match v {
            Some(x) => Ok(x),
            _ => Err(DAError::ConversionError),
        }
    }
}

// Implement the trait for f64
impl FromJsonValue<f64> for f64 {
    fn from_json_value(value: &Value) -> Result<Self, DAError> {
        let v: Option<f64> = value.as_f64();
        match v {
            Some(x) => Ok(x),
            _ => Err(DAError::ConversionError),
        }
    }
}

// Implement the trait for String
impl FromJsonValue<String> for String {
    fn from_json_value(value: &Value) -> Result<Self, DAError> {
        let v: Option<&str> = value.as_str();
        match v {
            Some(x) => Ok(x.to_string()),
            _ => Err(DAError::ConversionError),
        }
    }
}

// Implement the trait for bool
impl FromJsonValue<bool> for bool {
    fn from_json_value(value: &Value) -> Result<Self, DAError> {
        let v: Option<bool> = value.as_bool();
        match v {
            Some(x) => Ok(x),
            _ => Err(DAError::ConversionError),
        }
    }
}

impl DaikinAlthermaClient {
    /// Creates a new client to a Daikin Altherma LAN adapter.
    pub fn new(adapter_hostname: String) -> Self {
        let url_str = format!("ws://{adapter_hostname}/mca");
        let url = Url::parse(&url_str).unwrap();
        let ws_client = connect(url).unwrap();
        DaikinAlthermaClient {
            ws_client: ws_client.0,
        }
    }

    /// Returns the model of the LAN adapter. E.g. BRP069A61
    pub fn get_adapter_model(&mut self) -> String {
        let v = self
            .request_value("MNCSE-node/deviceInfo", None, "/m2m:rsp/pc/m2m:dvi/mod")
            .unwrap();

        return v.as_str().unwrap().to_string();
    }

    pub fn get_tank_parameters(&mut self) -> Result<TankParameters, DAError> {
        let temperature: f64 = self
            .request_value_hp_dft("2/Sensor/TankTemperature/la")
            .unwrap();

        let setpoint_temperature: f64 = self
            .request_value_hp_dft("2/Operation/TargetTemperature/la")
            .unwrap();

        let enabled_str: String = self.request_value_hp_dft("2/Operation/Power/la").unwrap();
        let powerful_i: i64 = self
            .request_value_hp_dft("2/Operation/Powerful/la")
            .unwrap();

        Ok(TankParameters {
            temperature,
            setpoint_temperature,
            enabled: enabled_str == "on",
            powerful: powerful_i == 1,
        })
    }

    /// Enables or disables the tank heating
    pub fn set_tank_enabled(&mut self, enabled: bool) -> Result<(), DAError> {
        let value = match enabled {
            true => "on",
            false => "off",
        };

        let payload = serde_json::json!({
        "con": value,
        "cnf": "text/plain:0"
        });

        self.set_value_hp("2/Operation/Power", Some(payload), "/")
            .unwrap();
        Ok(())
    }

    /// Enables or disable the tank powerful mode
    pub fn set_tank_powerful(&mut self, powerful: bool) -> Result<(), DAError> {
        let value = match powerful {
            true => 1,
            false => 0,
        };

        let payload = serde_json::json!({
        "con": value,
        "cnf": "text/plain:0"
        });

        self.set_value_hp("2/Operation/Powerful", Some(payload), "/")
            .unwrap();
        Ok(())
    }

    pub fn get_heating_parameters(&mut self) -> Result<HeatingParameters, DAError> {
        let indoor_temperature: f64 = self
            .request_value_hp_dft("1/Sensor/IndoorTemperature/la")
            .unwrap();

        let outdoor_temperature: f64 = self
            .request_value_hp_dft("1/Sensor/OutdoorTemperature/la")
            .unwrap();

        let indoor_setpoint_temperature: f64 = self
            .request_value_hp_dft("1/Operation/TargetTemperature/la")
            .unwrap();

        let leaving_water_temperature: f64 = self
            .request_value_hp_dft("1/Sensor/LeavingWaterTemperatureCurrent/la")
            .unwrap();

        let enabled_str: String = self.request_value_hp_dft("1/Operation/Power/la").unwrap();

        let on_holiday: i64 = self
            .request_value_hp_dft("1/Holiday/HolidayState/la")
            .unwrap();

        Ok(HeatingParameters {
            indoor_temperature,
            outdoor_temperature,
            indoor_setpoint_temperature,
            leaving_water_temperature,
            enabled: enabled_str == "on",
            on_holiday: on_holiday == 1,
        })
    }

    pub fn set_holiday_mode(&mut self, holiday_mode: bool) -> Result<(), DAError> {
        let value = match holiday_mode {
            true => 1,
            false => 0,
        };

        let payload = serde_json::json!({
        "con": value,
        "cnf": "text/plain:0"
        });

        self.set_value_hp("1/Holiday/HolidayState", Some(payload), "/")
            .unwrap();
        Ok(())
    }

    /// Sets the heating setpoint (target) temperature, in °C
    pub fn set_heating_setpoint_temperature(&mut self, temperature: f64) -> Result<(), DAError> {
        let payload = serde_json::json!({
        "con": temperature,
        "cnf": "text/plain:0"
        });

        self.set_value_hp("1/Operation/TargetTemperature", Some(payload), "/")
            .unwrap();
        Ok(())
    }

    /// Enables or disables the heating
    pub fn set_heating_enabled(&mut self, is_enabled: bool) -> Result<(), DAError> {
        let value = match is_enabled {
            true => "on",
            false => "standby",
        };

        let payload = serde_json::json!({
        "con": value,
        "cnf": "text/plain:0"
        });

        self.set_value_hp("1/Operation/Power", Some(payload), "/")
            .unwrap();
        Ok(())
    }

    fn request_value_hp_dft<T: FromJsonValue<T>>(&mut self, item: &str) -> Result<T, DAError> {
        let hp_item = format!("MNAE/{item}");
        let json_val = self
            .request_value(hp_item.as_str(), None, "/m2m:rsp/pc/m2m:cin/con")
            .unwrap();
        T::from_json_value(&json_val)
    }

    fn set_value_hp(
        &mut self,
        item: &str,
        payload: Option<Value>,
        output_path: &str,
    ) -> Result<(), DAError> {
        let hp_item = format!("MNAE/{item}");
        self.request_value(hp_item.as_str(), payload, output_path);
        /*
        match result {
            Ok(x) => Ok(()),
            Err(x) => Err(x),
        }
        */
        Ok(())
    }

    fn request_value(
        &mut self,
        item: &str,
        payload: Option<Value>,
        output_path: &str,
    ) -> Result<Value, DAError> {
        let reqid = Uuid::new_v4().to_string();

        let mut js_request = json!({
            "m2m:rqp": {
                "fr": "hello", //xxx,
                "rqi": reqid,
                "op": 2,
                "to": format!("/[0]/{}", item),
            }
        });

        if let Some(p) = payload {
            let set_value_params = json!({
                "ty": 4,
                "op": 1,
                "pc": {
                    "m2m:cin": p,
                },
            });
            js_request["m2m:rqp"]
                .as_object_mut()
                .unwrap()
                .extend(set_value_params.as_object().unwrap().clone());
        }

        println!(">>> {js_request}");

        self.ws_client
            .send(Message::Text(js_request.to_string()))
            .expect("Can't write message");
        let msg = self
            .ws_client
            .read()
            .expect("Error reading message")
            .into_text()
            .expect("Expected text");

        let result: Value = serde_json::from_str(&msg).expect("Can't parse JSON");

        assert_eq!(result["m2m:rsp"]["rqi"], reqid);
        assert_eq!(result["m2m:rsp"]["to"], "hello"); //XXX
                                                      //
        println!("<<< {result}");
        match result.pointer(output_path) {
            Some(v) => Ok(v.clone()),
            None => Err(DAError::NoSuchFieldError),
        }
    }
}
