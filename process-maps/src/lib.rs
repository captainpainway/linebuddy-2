use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use osm_xml as osm;
use js_sys;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn process_map(text: String, width: f64, height: f64) -> js_sys::Array {
    let doc = osm::OSM::parse(text.as_bytes()).unwrap();
    // log(&format!("{:?}", doc));

    let bounds = doc.bounds.unwrap();
    let arr = js_sys::Array::new();

    for (_id, way) in doc.ways.iter() {
        let coords = js_sys::Array::new();
                for node in way.nodes.iter() {
                    let n = &doc.resolve_reference(&node);
                    match n {
                        osm::Reference::Node(node) => {
                            let n_lat = map_points(node.lat, bounds.minlat, bounds.maxlat, 0.0, width);
                            let n_lon = map_points(node.lon, bounds.minlon, bounds.maxlon, 0.0, height);
                            let point = js_sys::Array::new();
                            point.push(&JsValue::from_f64(n_lat));
                            point.push(&JsValue::from_f64(n_lon));
                            coords.push(&point);
                        },
                        _ => {}
                    }
                }
        arr.push(&coords);
    }
    return arr;
}

fn map_points(value: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    return ((value - start1) / (stop1 - start1) * (stop2 - start2) + start2).floor();
}