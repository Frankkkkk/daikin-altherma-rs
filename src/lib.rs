use std::{fmt::Debug, net::TcpStream};
use uuid::Uuid;

use serde_json::{json, Value};
use tungstenite::{connect, stream, Message, WebSocket};
use url::Url;

mod errors;
mod params;
mod traits;
use crate::errors::DAError;
use crate::params::HeatingParameters;
use crate::params::TankParameters;

pub struct DaikinAlthermaClient {
    ws_client: WebSocket<stream::MaybeTlsStream<TcpStream>>,
}

impl DaikinAlthermaClient {
    /// Creates a new client to a Daikin Altherma LAN adapter.
    pub fn new(adapter_hostname: String) -> Result<Self, DAError> {
        let url_str = format!("ws://{adapter_hostname}/mca");
        let url = Url::parse(&url_str).map_err(|_| DAError::UrlParseError)?;
        let ws_client = connect(url).map_err(|_| DAError::WebSocketError)?;

        Ok(DaikinAlthermaClient {
            ws_client: ws_client.0,
        })
    }

    /// Returns the model of the LAN adapter. E.g. BRP069A61
    pub fn get_adapter_model(&mut self) -> Result<String, DAError> {
        let v = self.request_value("MNCSE-node/deviceInfo", None, "/m2m:rsp/pc/m2m:dvi/mod")?;

        match v.as_str() {
            Some(x) => Ok(x.to_string()),
            None => Err(DAError::NoSuchFieldError),
        }
    }

    pub fn get_tank_parameters(&mut self) -> Result<TankParameters, DAError> {
        let temperature: f64 = self.request_value_hp_dft("2/Sensor/TankTemperature/la")?;

        let setpoint_temperature: f64 =
            self.request_value_hp_dft("2/Operation/TargetTemperature/la")?;

        let enabled_str: String = self.request_value_hp_dft("2/Operation/Power/la")?;
        let powerful_i: i64 = self.request_value_hp_dft("2/Operation/Powerful/la")?;

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
    }

    pub fn get_heating_parameters(&mut self) -> Result<HeatingParameters, DAError> {
        let indoor_temperature: f64 = self.request_value_hp_dft("1/Sensor/IndoorTemperature/la")?;

        let outdoor_temperature: f64 =
            self.request_value_hp_dft("1/Sensor/OutdoorTemperature/la")?;

        let indoor_setpoint_temperature: f64 =
            self.request_value_hp_dft("1/Operation/TargetTemperature/la")?;

        let leaving_water_temperature: f64 =
            self.request_value_hp_dft("1/Sensor/LeavingWaterTemperatureCurrent/la")?;

        let enabled_str: String = self.request_value_hp_dft("1/Operation/Power/la")?;

        let on_holiday: i64 = self.request_value_hp_dft("1/Holiday/HolidayState/la")?;

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
    }

    /// Sets the heating setpoint (target) temperature, in °C
    pub fn set_heating_setpoint_temperature(&mut self, temperature: f64) -> Result<(), DAError> {
        let payload = serde_json::json!({
        "con": temperature,
        "cnf": "text/plain:0"
        });

        self.set_value_hp("1/Operation/TargetTemperature", Some(payload), "/")
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
    }

    fn request_value_hp_dft<T: traits::FromJsonValue<T>>(
        &mut self,
        item: &str,
    ) -> Result<T, DAError> {
        let hp_item = format!("MNAE/{item}");
        let json_val = self.request_value(hp_item.as_str(), None, "/m2m:rsp/pc/m2m:cin/con")?;
        T::from_json_value(&json_val)
    }

    fn set_value_hp(
        &mut self,
        item: &str,
        payload: Option<Value>,
        output_path: &str,
    ) -> Result<(), DAError> {
        let hp_item = format!("MNAE/{item}");
        self.request_value(hp_item.as_str(), payload, output_path)
            .map(|_| ())
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

        match result.pointer(output_path) {
            Some(v) => Ok(v.clone()),
            None => Err(DAError::NoSuchFieldError),
        }
    }
}
