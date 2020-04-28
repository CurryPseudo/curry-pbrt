use crate::*;
use byteorder::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
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
            let file = File::open(path).unwrap();
            let (indices, vertices) = PlyParser::new(file).parse();
            info!("Loaded ply mesh {}", path.to_str().unwrap());
            create_triangles(Arc::new(TriangleMesh::new(indices, vertices, None, None)))
        })
        .clone()
}

type Endian = LittleEndian;
pub struct PlyParser {
    reader: BufReader<File>,
    require_vertex_property: Vec<(String, bool)>,
    line: String,
}

impl PlyParser {
    pub fn new(file: File) -> Self {
        let require_vertex_property = vec!["x", "y", "z", "nx", "ny", "nz", "u", "v"]
            .into_iter()
            .map(|s| (String::from(s), false))
            .collect();
        Self {
            reader: BufReader::new(file),
            require_vertex_property,
            line: String::new(),
        }
    }
}
impl PlyParser {
    pub fn parse(mut self) -> (Vec<usize>, Vec<Point3f>) {
        self.read_line();
        assert_eq!(&self.line, "ply\n");
        self.read_line();
        assert_eq!(&self.line, "format binary_little_endian 1.0\n");
        self.read_line();
        assert_eq!(&self.line[0..14], "element vertex");
        let vertices_count = self.line_end(15).parse::<usize>().unwrap();
        while self.read_property() {}
        self.require_all_satisfied();
        assert_eq!(&self.line[0..12], "element face");
        let face_count = self.line_end(13).parse::<usize>().unwrap();
        self.read_line();
        assert_eq!(&self.line, "property list uint8 int vertex_indices\n");
        self.read_line();
        assert_eq!(&self.line, "end_header\n");
        let mut vertices = Vec::new();
        for _ in 0..vertices_count {
            vertices.push(self.read_point3f());
            self.read_normal3f();
            self.read_point2f();
        }
        let mut indices = Vec::new();
        for _ in 0..face_count {
            let count = self.read_u8();
            assert_eq!(count, 3);
            for _ in 0..count {
                indices.push(self.read_integer() as usize);
            }
        }
        (indices, vertices)
    }
    fn read_line(&mut self) -> bool {
        self.line.clear();
        self.reader.read_line(&mut self.line).is_ok()
    }
    fn read_float(&mut self) -> Float {
        self.reader.read_f32::<Endian>().unwrap()
    }
    fn read_point3f(&mut self) -> Point3f {
        Point3f::new(self.read_float(), self.read_float(), self.read_float())
    }
    fn read_u8(&mut self) -> u8 {
        self.reader.read_u8().unwrap()
    }
    fn read_integer(&mut self) -> i32 {
        self.reader.read_i32::<Endian>().unwrap()
    }
    fn read_normal3f(&mut self) -> Normal3f {
        Normal3f::from(Vector3f::new(
            self.read_float(),
            self.read_float(),
            self.read_float(),
        ))
    }
    fn read_point2f(&mut self) -> Point2f {
        Point2f::new(self.read_float(), self.read_float())
    }
    fn read_property(&mut self) -> bool {
        self.read_line();
        if &self.line[0..8] != "property" {
            return false;
        }
        assert_eq!(&self.line[9..14], "float");
        let property = &self.line[15..self.line.len() - 1];
        for (p, b) in &mut self.require_vertex_property {
            if p == property {
                *b = true;
            }
        }
        true
    }
    fn line_end(&self, begin: usize) -> &str {
        &self.line[begin..self.line.len() - 1]
    }
    fn require_all_satisfied(&self) {
        for (p, b) in &self.require_vertex_property {
            if !b {
                panic!("Required property {} not satisfied", p);
            }
        }
    }
}
