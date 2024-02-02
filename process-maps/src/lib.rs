use wasm_bindgen::prelude::*;
use xml::reader::{EventReader, XmlEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> String {
    return "Hello, process-maps!".to_string();
}

#[wasm_bindgen]
pub fn process_map(text: String) {
    let parser = EventReader::from_str(&text);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                log(&format!("Start: {:?}", name));
                for attr in attributes {
                    log(&format!("  attr: {:?}", attr));
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                log(&format!("End: {:?}", name));
            }
            Ok(XmlEvent::Characters(s)) => {
                log(&format!("Text: {:?}", s));
            }
            Err(e) => {
                log(&format!("Error: {:?}", e));
            }
            _ => {}
        }
    }
}
