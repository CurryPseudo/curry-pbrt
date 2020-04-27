use crate::*;
use ply_rs as ply;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
type Map = ply::ply::KeyMap<ply::ply::Property>;
lazy_static! {
    static ref PLYMESH_CACHE: Mutex<HashMap<PathBuf, Vec<Arc<dyn Shape>>>> =
        Mutex::new(HashMap::new());
}
pub fn create_plymesh(path: &std::path::Path) -> Vec<Arc<dyn Shape>> {
    PLYMESH_CACHE
        .lock()
        .unwrap()
        .entry(path.into())
        .or_insert_with(|| {
            let mut f = std::fs::File::open(path).unwrap();
            let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();
            let ply = p.read_ply(&mut f);
            assert!(ply.is_ok());
            let ply = ply.unwrap();
            let mut vertices = Vec::new();
            for vertex in &ply.payload["vertex"] {
                let point: Point3f = read_vector(vertex, "").into();
                vertices.push(point);
            }
            let mut indices = Vec::new();
            for face in &ply.payload["face"] {
                if let ply::ply::Property::ListInt(xs) = &face["vertex_indices"] {
                    indices.push(xs[0] as usize);
                    indices.push(xs[1] as usize);
                    indices.push(xs[2] as usize);
                } else {
                    panic!()
                }
            }
            create_triangles(Arc::new(TriangleMesh::new(indices, vertices, None, None)))
        })
        .clone()
}

fn read_vector(map: &Map, prefix: &str) -> Vector3f {
    let s = String::from(prefix) + "x";
    let x = get_float(&map[&s]);
    let s = String::from(prefix) + "y";
    let y = get_float(&map[&s]);
    let s = String::from(prefix) + "z";
    let z = get_float(&map[&s]);
    Vector3f::new(x, y, z)
}

fn get_float(property: &ply::ply::Property) -> Float {
    match property {
        ply::ply::Property::Float(f) => *f,
        _ => panic!(),
    }
}
