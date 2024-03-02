import init, {process_ways, process_relations, process_nodes} from '../process-maps/pkg/process_maps.js';
init();

const queryString = window.location.search;
const searchParams = new URLSearchParams(queryString);
let park = searchParams.get('park');
let n, w, s, e;

switch (park) {
    case 'epcot':
        n = 28.3768;
        w = -81.5553;
        s = 28.3661;
        e = -81.5425;
        break;
    case 'hollywood_studios':
    case 'hs':
        n = 28.3625;
        w = -81.5641;
        s = 28.3523;
        e = -81.5561;
        break;
    case 'animal_kingdom':
    case 'ak':
        n = 28.3692;
        w = -81.5984;
        s = 28.3524;
        e = -81.5831;
        break;
    case 'magic_kingdom':
    case 'mk':
    default:
        n = 28.42266;
        w = -81.58586;
        s = 28.41604;
        e = -81.57600;
        break;
}

const bbox = `${s},${w},${n},${e}`;
const buildings = `way[building][!name];foreach{(._;>;);out;}`;
const named_buildings = `way[building][name];foreach{(._;>;);out;}`;
const walkways = `way[highway];foreach{(._;>;);out;}`;
const trees = `node[natural=tree];foreach{(._;>;);out;}`;
const gardens = `(way[leisure=garden];way[landuse=forest];way[landuse=meadow];);foreach{(._;>;);out;}`;
const water = `relation[natural=water];foreach{(._;>;);out;}`;
const query = `[timeout:90][bbox:${bbox}];`;

let scene = document.querySelector('a-scene');
let ratio = (Math.abs(w) - Math.abs(e)) / (n - s);
let width = 1800;
let height = width * ratio;

const url = `https://overpass-api.de/api/interpreter?data=${query}`;
const water_data = getWater(url);
const garden_data = getGardens(url);
const walkway_data = getWalkways(url);
const tree_data = getTrees(url);
const building_data = getBuildings(url);
const nbuilding_data = getNamedBuildings(url);

Promise.all([water_data, garden_data, walkway_data, tree_data, building_data, nbuilding_data]).then(values => {
    const [water, gardens, walkways, trees, buildings, named_buildings] = values;
    createGeometry(water, 0.05, 'rgb(83,156,156)');
    createGeometry(gardens, 0.1, 'rgb(136,172,140)');
    createGeometry(buildings, 0.5, 'rgb(88,87,98)');
    createGeometry(named_buildings, 1.0, 'rgb(88,87,98)');
    createLineGeometry(walkways);
    // createGeometry(buildings, 0.5, 'rgb(220,177,102)');
    // createGeometry(named_buildings, 1.0, 'rgb(220,177,102)');
    // drawPolygons(water, 'rgb(83,156,156)', null);
    // drawPolygons(gardens, 'rgb(136,172,140)', null);
    // drawPolygons(walkways, null, 'rgb(0,0,0)');
    // for (let tree of trees) {
    //     ctx.beginPath();
    //     ctx.arc(tree[0], tree[1], 3, 0, 2 * Math.PI);
    //     ctx.fillStyle = 'rgb(40,107,83)';
    //     ctx.fill();
    //     ctx.closePath();
    // }
    // drawPolygons(buildings, 'rgb(98,90,87)', null);
    // drawPolygons(named_buildings, 'rgb(220,177,102)', null);
});

function createGeometry(p, height, color) {
    for (let polygon of p) {
        let vertices = polygon.map(point => {
            let [x, y] = point;
            return `${x / 50} ${y / 50}`;
        });
        let mapItem = document.createElement('a-entity');
        mapItem.setAttribute('geometry', {
            primitive: 'map-item',
            height,
            vertices,
        });
        mapItem.setAttribute('material', {
            color,
        });
        scene.appendChild(mapItem);
    }
}

function createLineGeometry(p) {
    for (let polygon of p) {
        let vertices = polygon.map(point => {
            let [x, y] = point;
            return `${x / 50} ${y / 50}`;
        });
        const scene = document.querySelector('a-scene').object3D;
        const points = vertices.map(point => {
            let [x, y] = point.split(' ').map(val => parseFloat(val));
            return new THREE.Vector3(x, 0.01, y);
        });
        const geometry = new THREE.BufferGeometry().setFromPoints(points);
        const line = new THREE.Line(geometry, new THREE.LineBasicMaterial({color: 0x000000}));
        scene.add(line);
    }
}

function getWater(url) {
    return fetch(`${url}${water};out;`).then(response => {
        return response.text();
    }).then(data => {
        return process_relations(data, width, height);
    });
}

function getWalkways(url) {
    return fetch(`${url}${walkways};out;`).then(response => {
        return response.text();
    }).then(data => {
        return process_ways(data, width, height);
    });
}

function getBuildings(url) {
    return fetch(`${url}${buildings};out;`).then(response => {
        return response.text();
    }).then(data => {
        return process_ways(data, width, height);
    });
}

function getNamedBuildings(url) {
    return fetch(`${url}${named_buildings};out;`).then(response => {
        return response.text();
    }).then(data => {
        return process_ways(data, width, height);
    });
}

function getGardens(url) {
    return fetch(`${url}${gardens};out;`).then(response => {
        return response.text();
    }).then(data => {
        return process_ways(data, width, height);
    });
}

function getTrees(url) {
    return fetch(`${url}${trees};out;`).then(response => {
        return response.text();
    }).then(data => {
        return process_nodes(data, width, height);
    });
}