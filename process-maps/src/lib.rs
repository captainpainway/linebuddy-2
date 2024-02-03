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
    let rel_info = relation_reference_statistics(&doc);
    let way_info = way_reference_statistics(&doc);
    let poly_count = doc.ways.values().fold(0, |acc, way| {
        if way.is_polygon() {
            return acc + 1
        }

        acc
    });

    log(&format!("Node count {:?}", doc.nodes.len()));
    log(&format!("Way count {:?}", doc.ways.len()));
    log(&format!("Polygon count {:?}", poly_count));
    log(&format!("Relation count {:?}", doc.relations.len()));
    log(&format!("Tag count {:?}", tag_count(&doc)));

    log(&format!("Way reference count: {:?}, invalid references: {:?}",  way_info.0, way_info.1));
    log(&format!("Relation reference count: {:?}, resolved: {:?}, unresolved: {:?}", rel_info.0, rel_info.1, rel_info.2));
    // let parser = EventReader::from_str(&text);
    // for e in parser {
    //     match e {
    //         Ok(XmlEvent::StartElement { name, attributes, .. }) => {
    //             log(&format!("Start: {:?}", name));
    //             for attr in attributes {
    //                 log(&format!("  attr: {:?}", attr));
    //             }
    //         }
    //         Ok(XmlEvent::EndElement { name }) => {
    //             log(&format!("End: {:?}", name));
    //         }
    //         Ok(XmlEvent::Characters(s)) => {
    //             log(&format!("Text: {:?}", s));
    //         }
    //         Err(e) => {
    //             log(&format!("Error: {:?}", e));
    //         }
    //         _ => {}
    //     }
    // }
}

fn relation_reference_statistics(doc: &osm::OSM) -> (usize, usize, usize) {
    doc.relations.values()
        .flat_map(|relation| relation.members.iter())
        .fold((0, 0, 0), |acc, member| {
            let el_ref = match *member {
                 osm::Member::Node(ref el_ref, _) => el_ref,
                 osm::Member::Way(ref el_ref, _) => el_ref,
                 osm::Member::Relation(ref el_ref, _) => el_ref,
            };

            match doc.resolve_reference(&el_ref) {
                osm::Reference::Unresolved => (acc.0 + 1, acc.1, acc.2 + 1),
                osm::Reference::Node(_)     |
                osm::Reference::Way(_)      |
                osm::Reference::Relation(_) => (acc.0 + 1, acc.1 + 1, acc.2)
            }
        })
}

fn way_reference_statistics(doc: &osm::OSM) -> (usize, usize) {
    doc.ways.values()
        .flat_map(|way| way.nodes.iter())
        .fold((0, 0), |acc, node| {
            match doc.resolve_reference(&node) {
                osm::Reference::Node(_) => (acc.0 + 1, acc.1),
                osm::Reference::Unresolved  |
                osm::Reference::Way(_)      |
                osm::Reference::Relation(_) => (acc.0, acc.1 + 1)
            }
        })
}

fn tag_count(doc: &osm::OSM) -> usize {
    let node_tag_count = doc.nodes.values()
        .map(|node| node.tags.len())
        .fold(0, |acc, c| acc + c);
    let way_tag_count = doc.ways.values()
        .map(|way| way.tags.len())
        .fold(0, |acc, c| acc + c);
    let relation_tag_count = doc.relations.values()
        .map(|relation| relation.tags.len())
        .fold(0, |acc, c| acc + c);

    node_tag_count + way_tag_count + relation_tag_count
}
