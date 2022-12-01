use std::collections::HashMap;
use std::ops::AddAssign;

use cgmath::prelude::*;
use cgmath::Vector3;

const VERT_CACHE_PRECISION: f32 = 10000_f32;

#[derive(Debug)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Triangle {
    fn new(a: usize, b: usize, c: usize) -> Triangle {
        Triangle { a, b, c }
    }
}

pub struct ArraySerializedVector(pub Vector3<f32>);

pub struct Polyhedron {
    pub positions: Vec<ArraySerializedVector>,
    pub cells: Vec<Triangle>,
    pub normals: Vec<ArraySerializedVector>,
    pub colors: Vec<ArraySerializedVector>,
    added_vert_cache: HashMap<(i32, i32, i32), usize>,
    faces: Vec<Vec<usize>>,
}

impl AddAssign for ArraySerializedVector {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0);
    }
}

impl Polyhedron {
    pub fn new() -> Polyhedron {
        Polyhedron {
            positions: vec![],
            cells: vec![],
            normals: vec![],
            colors: vec![],
            added_vert_cache: HashMap::new(),
            faces: vec![],
        }
    }

    pub fn new_isocahedron(radius: f32, detail: u32) -> Polyhedron {
        let t = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let mut base_isocahedron = Polyhedron {
            positions: vec![],
            cells: vec![
                Triangle::new(0, 11, 5),
                Triangle::new(0, 5, 1),
                Triangle::new(0, 1, 7),
                Triangle::new(0, 7, 10),
                Triangle::new(0, 10, 11),
                Triangle::new(1, 5, 9),
                Triangle::new(5, 11, 4),
                Triangle::new(11, 10, 2),
                Triangle::new(10, 7, 6),
                Triangle::new(7, 1, 8),
                Triangle::new(3, 9, 4),
                Triangle::new(3, 4, 2),
                Triangle::new(3, 2, 6),
                Triangle::new(3, 6, 8),
                Triangle::new(3, 8, 9),
                Triangle::new(4, 9, 5),
                Triangle::new(2, 4, 11),
                Triangle::new(6, 2, 10),
                Triangle::new(8, 6, 7),
                Triangle::new(9, 8, 1),
            ],
            normals: vec![],
            colors: vec![],
            added_vert_cache: HashMap::new(),
            faces: vec![],
        };
        base_isocahedron.add_position(Vector3::new(-1.0, t, 0.0));
        base_isocahedron.add_position(Vector3::new(1.0, t, 0.0));
        base_isocahedron.add_position(Vector3::new(-1.0, -t, 0.0));
        base_isocahedron.add_position(Vector3::new(1.0, -t, 0.0));
        base_isocahedron.add_position(Vector3::new(0.0, -1.0, t));
        base_isocahedron.add_position(Vector3::new(0.0, 1.0, t));
        base_isocahedron.add_position(Vector3::new(0.0, -1.0, -t));
        base_isocahedron.add_position(Vector3::new(0.0, 1.0, -t));
        base_isocahedron.add_position(Vector3::new(t, 0.0, -1.0));
        base_isocahedron.add_position(Vector3::new(t, 0.0, 1.0));
        base_isocahedron.add_position(Vector3::new(-t, 0.0, -1.0));
        base_isocahedron.add_position(Vector3::new(-t, 0.0, 1.0));

        let mut subdivided = Polyhedron::new();
        subdivided.subdivide(base_isocahedron, radius, detail);
        subdivided.triangles_to_faces();
        subdivided
    }

    fn subdivide(&mut self, other: Polyhedron, radius: f32, detail: u32) {
        for triangle in other.cells {
            let a = other.positions[triangle.a].0;
            let b = other.positions[triangle.b].0;
            let c = other.positions[triangle.c].0;
            self.subdivide_triangle(a, b, c, radius, detail);
        }
    }

    fn triangles_to_faces(&mut self) {
        for (cell_index, _) in self.cells.iter().enumerate() {
            self.faces.push(vec![cell_index]);
        }
    }

    fn subdivide_triangle(
        &mut self,
        a: Vector3<f32>,
        b: Vector3<f32>,
        c: Vector3<f32>,
        radius: f32,
        detail: u32,
    ) {
        let cols = 2usize.pow(detail);
        let mut new_vertices: Vec<Vec<Vector3<f32>>> = vec![];

        for i in 0..=cols {
            new_vertices.push(vec![]);
            let aj = a.lerp(c, i as f32 / cols as f32);
            let bj = b.lerp(c, i as f32 / cols as f32);
            let rows = cols - i;

            for j in 0..=rows {
                if j == 0 && i == cols {
                    new_vertices[i].push(aj.normalize() * radius);
                } else {
                    new_vertices[i].push(
                        a.lerp(c, i as f32 / cols as f32)
                            .lerp(bj, j as f32 / rows as f32)
                            .normalize()
                            * radius,
                    );
                }
            }
        }

        for i in 0..cols {
            for j in 0..2 * (cols - i) - 1 {
                let k = j / 2;

                let mut triangle = Triangle { a: 0, b: 0, c: 0 };
                if j % 2 == 0 {
                    triangle.a = self.add_position(new_vertices[i][k + 1]);
                    triangle.b = self.add_position(new_vertices[i + 1][k]);
                    triangle.c = self.add_position(new_vertices[i][k]);
                } else {
                    triangle.a = self.add_position(new_vertices[i][k + 1]);
                    triangle.b = self.add_position(new_vertices[i + 1][k + 1]);
                    triangle.c = self.add_position(new_vertices[i + 1][k]);
                }

                self.cells.push(triangle);
            }
        }
    }

    fn add_position(&mut self, vertex: Vector3<f32>) -> usize {
        let vertex_key = (
            (vertex.x * VERT_CACHE_PRECISION).round() as i32,
            (vertex.y * VERT_CACHE_PRECISION).round() as i32,
            (vertex.z * VERT_CACHE_PRECISION).round() as i32,
        );
        if let Some(added_vert_index) = self.added_vert_cache.get(&vertex_key) {
            *added_vert_index
        } else {
            self.positions.push(ArraySerializedVector(vertex));
            self.normals
                .push(ArraySerializedVector(Vector3::new(0.0, 0.0, 0.0)));
            self.colors
                .push(ArraySerializedVector(Vector3::new(1.0, 1.0, 1.0)));
            let added_index = self.positions.len() - 1;
            self.added_vert_cache.insert(vertex_key, added_index);
            added_index
        }
    }
}
