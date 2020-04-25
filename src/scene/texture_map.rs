use std::path::PathBuf;
use crate::def::Float;
use crate::scene_file_parser::PropertySet;
use crate::spectrum::Spectrum;
use crate::texture;
use crate::texture::ImageTexture;
use crate::utility::AnyHashMap;
use std::path::Path;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct TextureMap {
    map: AnyHashMap<String>,
}

impl TextureMap {
    pub fn add_texture(&mut self, property_set: &PropertySet) {
        let file_name: PathBuf = property_set.get_value("filename").unwrap();
        let mut property_set = property_set.clone();
        let name = String::from(property_set
            .as_one_basic_types(1)
            .unwrap()
            .get_string()
            .unwrap());
        let texture_type = String::from(
            property_set
                .as_one_basic_types(1)
                .unwrap()
                .get_string()
                .unwrap(),
        );
        match texture_type.as_str() {
            "float" => self.map.insert(name, ImageTexture::<Float>::from_file(&Path::new(&file_name))),
            "spectrum" => self.map.insert(name, ImageTexture::<Spectrum>::from_file(&Path::new(&file_name))),
            _ => panic!(),
        }
    }
}

impl texture::TextureMap for TextureMap {
    fn get<T: Send + Sync + 'static>(&self, name: &str) -> Option<Arc<ImageTexture<T>>> {
        self.map.get(name)
    }
}
