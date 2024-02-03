use wasm_bindgen::prelude::*;
use osm_xml as osm;

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
    let doc = osm::OSM::parse(text.as_bytes()).unwrap();

    // normalize_points(doc.bounds.unwrap());

    for (id, way) in doc.ways.iter() {
        if way.is_polygon() {
            for tag in way.tags.iter() {
                if tag.key == "name" && tag.val == "Prince Charming Regal Carrousel" {
                    log(&format!("{:?}", way));
                    for node in way.nodes.iter() {
                        let n = osm::OSM::resolve_reference(node);
                        log(&format!("{:?}", n));
                    } 
                }
            }
        }
    }
}

fn normalize_points(bounds: osm::Bounds) -> (u32, u32) {
    let min_x = (bounds.minlat, 0);
    let min_y = (bounds.minlon, 0);
    let max_x = (bounds.maxlat, 800);
    let max_y = (bounds.maxlon, 600);


    log(&format!("{:?}", bounds));
    return (0, 0);
}