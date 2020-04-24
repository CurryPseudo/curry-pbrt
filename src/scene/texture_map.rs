use std::path::PathBuf;
use crate::def::Float;
use crate::scene_file_parser::PropertySet;
use crate::spectrum::Spectrum;
use crate::texture;
use crate::texture::ImageTexture;
use downcast_rs::DowncastSync;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

trait TextureBase: DowncastSync {}
impl_downcast!(sync TextureBase);

impl<T: Send + Sync + 'static> TextureBase for ImageTexture<T> {}

#[derive(Default, Clone)]
pub struct TextureMap {
    map: HashMap<String, Arc<dyn TextureBase>>,
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
        let texture: Arc<dyn TextureBase> = match texture_type.as_str() {
            "float" => Arc::new(ImageTexture::<Float>::from_file(&Path::new(&file_name))),
            "spectrum" => Arc::new(ImageTexture::<Spectrum>::from_file(&Path::new(&file_name))),
            _ => panic!(),
        };
        self.map.insert(name, texture);
    }
}

impl texture::TextureMap for TextureMap {
    fn get<T: Send + Sync + 'static>(&self, name: &str) -> Option<Arc<ImageTexture<T>>> {
        let any = self.map.get(name)?.clone();
        match any.downcast_arc::<ImageTexture<T>>() {
            Ok(r) => Some(r),
            Err(_) => panic!(),
        }
    }
}
