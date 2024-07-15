use bevy::asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext};
use bevy::ecs::prelude::{FromWorld, World};
use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::{
    CompressedImageFormats, Image, ImageFormatSetting, ImageLoaderSettings, ImageType, TextureError,
};

use image::DynamicImage;
use thiserror::Error;

use crate::HeightMap;

/// Loader for images that can be read by the `image` crate.
#[derive(Clone)]
pub struct HeightMapLoader {
    supported_compressed_formats: CompressedImageFormats,
}
impl HeightMapLoader {
    pub const EXTENSIONS: &'static [&'static str] = &["hmp.png"];
}

impl AssetLoader for HeightMapLoader {
    type Asset = Mesh;
    type Settings = ImageLoaderSettings;
    type Error = HeightMapLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a ImageLoaderSettings,
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Mesh, Self::Error>> {
        Box::pin(async move {
            // use the file extension for the image type
            let ext = load_context.path().extension().unwrap().to_str().unwrap();

            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let image_type = match settings.format {
                ImageFormatSetting::FromExtension => ImageType::Extension(ext),
                ImageFormatSetting::Format(format) => ImageType::Format(format),
            };
            let image: Image = Image::from_buffer(
                &bytes,
                image_type,
                self.supported_compressed_formats,
                settings.is_srgb,
                settings.sampler.clone(),
                settings.asset_usage,
            )
            .map_err(|err| HeightMapFileError {
                error: err,
                path: format!("{}", load_context.path().display()),
            })?;
            let size = image.size();
            let pixel_scale = (size - UVec2::ONE).as_vec2();
            if let Ok(DynamicImage::ImageRgba8(rgba)) = image.clone().try_into_dynamic() {
                let h = |p: Vec2| -> f32 {
                    let xy = (pixel_scale * (p + Vec2::ONE / 2.)).as_uvec2();
                    rgba.get_pixel(xy.x, xy.y)[0] as f32 / 255.
                };
                // let h = |p: Vec2| ((p.x * 10.).sin() + (p.y * 10.).sin()) / 2.;

                Ok(HeightMap { size, h }.into())
            } else {
                error!("Invalid image type. Generating empty plane...");
                Ok(Mesh::from(Rectangle {
                    half_size: Vec2::ONE,
                }))
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["png"]
    }
}

impl FromWorld for HeightMapLoader {
    fn from_world(world: &mut World) -> Self {
        let supported_compressed_formats = match world.get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),

            None => CompressedImageFormats::NONE,
        };
        Self {
            supported_compressed_formats,
        }
    }
}

/// An error that occurs when loading a texture from a file.
#[derive(Error, Debug)]
pub struct HeightMapFileError {
    error: TextureError,
    path: String,
}
impl std::fmt::Display for HeightMapFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Error reading image file {}: {}, this is an error in `bevy_render`.",
            self.path, self.error
        )
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum HeightMapLoaderError {
    #[error("Could load shader: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not load texture file: {0}")]
    FileTexture(#[from] HeightMapFileError),
}
