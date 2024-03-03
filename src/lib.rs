use std::{fmt::Debug, net::TcpStream};
use uuid::Uuid;

use serde_json::{json, Value};
use tungstenite::{connect, stream, Error, Message, WebSocket};
use url::Url;

pub struct DaikinAlthermaClient {
    //todo WS con
    ws_client: WebSocket<stream::MaybeTlsStream<TcpStream>>,
}

impl DaikinAlthermaClient {
    pub fn new(adapter_hostname: String) -> Self {
        let url_str = format!("ws://{adapter_hostname}/mca");
        let url = Url::parse(&url_str).unwrap();
        let ws_client = connect(url).unwrap();
        DaikinAlthermaClient {
            ws_client: ws_client.0,
        }
    }

    pub fn get_adapter_model(&mut self) -> String {
        let v = self
            .request_value("MNCSE-node/deviceInfo", None, "/m2m:rsp/pc/m2m:dvi/mod")
            .unwrap();

        return v.as_str().unwrap().to_string();
    }

    pub fn is_holiday_mode(&mut self) -> bool {
        let v = self
            .request_value("1/Holiday/HolidayState/la", None, "/m2m:rsp/pc/m2m:cin/con")
            .unwrap();

        return v.as_i64().unwrap() == 1;
    }

    fn request_value(
        &mut self,
        item: &str,
        payload: Option<Value>,
        output_path: &str,
    ) -> Result<Value, String> {
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
                                                      //
        println!("Rslt is {result}. Opath is {output_path}");

        match result.pointer(output_path) {
            Some(v) => Ok(v.clone()),
            None => Err("Todo".to_string()),
        }
    }
}
