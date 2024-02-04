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
pub fn process_nodes(text: String, width: f64, height: f64) -> js_sys::Array {
    let doc = osm::OSM::parse(text.as_bytes()).unwrap();
    let bounds = doc.bounds.unwrap();
    let arr = js_sys::Array::new();
    for (_id, node) in doc.nodes.iter() {
        arr.push(&process_points(node, &bounds, width, height));
    }
    arr
}

#[wasm_bindgen]
pub fn process_ways(text: String, width: f64, height: f64) -> js_sys::Array {
    let doc = osm::OSM::parse(text.as_bytes()).unwrap();
    let bounds = doc.bounds.unwrap();
    let arr = js_sys::Array::new();
    for (_id, way) in doc.ways.iter() {
        arr.push(&process_coords(&doc, way, &bounds, width, height));
    }
    arr
}

#[wasm_bindgen]
pub fn process_relations(text: String, width: f64, height: f64) -> js_sys::Array {
    let doc = osm::OSM::parse(text.as_bytes()).unwrap();
    let bounds = doc.bounds.unwrap();
    let arr = js_sys::Array::new();
    for (_id, relation) in doc.relations.iter() {
        for member in relation.members.iter() {
            match member {
                osm::Member::Way(way, t) => {
                    let w = &doc.resolve_reference(&way);
                    match w {
                        osm::Reference::Way(way) => {
                            if t == "outer" {
                                arr.push(&process_coords(&doc, way, &bounds, width, height));
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
    arr
}

fn process_coords(doc: &osm::OSM, way: &osm::Way, bounds: &osm::Bounds, width: f64, height: f64) -> js_sys::Array {
    let coords = js_sys::Array::new();
    for node in way.nodes.iter() {
        let n = &doc.resolve_reference(&node);
        match n {
            osm::Reference::Node(node) => {
                let point = process_points(node, &bounds, width, height);
                coords.push(&point);
            },
            _ => {}
        }
    }
    coords
}

fn process_points(node: &osm::Node, bounds: &osm::Bounds, width: f64, height: f64) -> js_sys::Array {
    let y = map_points(node.lat, bounds.minlat, bounds.maxlat, 0.0, width);
    let x = map_points(node.lon, bounds.minlon, bounds.maxlon, 0.0, height);
    let point = js_sys::Array::new();
    point.push(&JsValue::from_f64(x));
    point.push(&JsValue::from_f64(y));
    point
}

fn map_points(value: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    ((value - start1) / (stop1 - start1) * (stop2 - start2) + start2).floor()
}