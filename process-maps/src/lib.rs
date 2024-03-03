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
pub fn process_ways(text: String, width: f64, height: f64, check_geom: bool) -> js_sys::Array {
    let doc = osm::OSM::parse(text.as_bytes()).unwrap();
    let bounds = doc.bounds.unwrap();
    let arr = js_sys::Array::new();
    for (_id, way) in doc.ways.iter() {
        if (check_geom && check_geometry(&way)) || !check_geom {
            arr.push(&process_coords(&doc, way, &bounds, width, height));
        }
    }
    arr
}

#[wasm_bindgen]
pub fn process_relations(text: String, width: f64, height: f64) -> js_sys::Array {
    let doc = osm::OSM::parse(text.as_bytes()).unwrap();
    let bounds = doc.bounds.unwrap();
    let arr = js_sys::Array::new();
    for (_id, relation) in doc.relations.iter() {
        // log(&format!("{:?}", relation));
        for member in relation.members.iter() {
            // log(&format!("{:?}", member));
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
    // log(&format!("{:?}", arr));
    arr
}

fn check_geometry(way: &osm::Way) -> bool {
    let first = way.nodes.first().unwrap();
    let last = way.nodes.last().unwrap();
    first == last
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

fn process_points(node: &osm::Node, bounds: &osm::Bounds, width: f64, height: f64) -> JsValue {
    let y = map_points(node.lat, bounds.minlat, bounds.maxlat, -width / 2.0, width / 2.0) / 50.0;
    let x = map_points(node.lon, bounds.minlon, bounds.maxlon, -height / 2.0, height / 2.0) / 50.0 * -1.0;
    JsValue::from_str(&format!("{} {}", x, y))
}

fn map_points(value: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    ((value - start1) / (stop1 - start1) * (stop2 - start2) + start2).floor()
}