AFRAME.registerGeometry('map-item', {
    schema: {
        height: {default: 1},
        vertices: {
            default: ['-2 -2', '-2 0', '1 1', '2 0'],
        }
    },
    init: function(data) {
        const shape = new THREE.Shape();
        for (let i = 0; i < data.vertices.length; i++) {
            let vertex = data.vertices[i];
            let [x, y] = vertex.split(' ').map(val => parseFloat(val));
            if (i === 0) {
                shape.moveTo(x, y);
            } else {
                shape.lineTo(x, y);
            }
        }
        const extrudeSettings = {
            steps: 2,
            depth: data.height,
            bevelEnabled: true,
            bevelThickness: 0.02,
            bevelSize: 0.01,
            bevelSegments: 8,
        };
        const geometry = new THREE.ExtrudeGeometry(shape, extrudeSettings);
        // Geometry needs to be rotated and translated to be in the right position
        geometry.rotateX(Math.PI / 2);
        geometry.translate(0, data.height, 0);
        geometry.computeBoundingBox();
        this.geometry = geometry;
    }
});