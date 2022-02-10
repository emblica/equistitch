extern crate image;

use anyhow::{self};
use cgmath::{InnerSpace, Vector2, Vector3};
use clap::{Parser, Subcommand};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::path::PathBuf;
use std::{f32::consts::PI, fs};

/// Equistitch is utility for manipulating 360-degree equirectangular images
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Splits equirectangular image into cubemap faces (up, down, left, right, front, back) and in addition to tiles (with defined patch size)
    Split {
        /// Input file (image)
        #[clap(short, long, parse(from_os_str), value_name = "FILE")]
        input: PathBuf,
        /// Size for tiles
        #[clap(short, long, default_value_t = 480)]
        patch_size: u32,
        /// Output directory for cubemap faces
        #[clap(short, long, parse(from_os_str), value_name = "CUBEMAP_OUTPUT")]
        cubemap_faces_output: Option<PathBuf>,
        /// Output directory for tiles
        #[clap(short, long, parse(from_os_str), value_name = "TILES_OUTPUT")]
        tiles_output: Option<PathBuf>,
    },
    /// Stitches cubemap faces (up, down, left, right, front, back) or patches back into equirectangular image
    Stitch {
        /// Input directory (tiles or cubemap faces)
        #[clap(short, long, parse(from_os_str), value_name = "INPUT_DIR")]
        input_dir: PathBuf,
        /// Output file
        #[clap(short, long, parse(from_os_str), value_name = "OUTPUT")]
        output: PathBuf,
        /// Stitch tiles
        #[clap(short, long)]
        tiles: bool,
        /// File extension
        #[clap(short, long, default_value = "jpg")]
        extension: String,
    },
}

struct Cube {
    front: RgbaImage,
    back: RgbaImage,
    left: RgbaImage,
    right: RgbaImage,
    up: RgbaImage,
    down: RgbaImage,
}

impl Cube {
    pub fn from_directory_of_patches(prefix: &str, extension: &str) -> anyhow::Result<Cube> {
        let ext = format!(".{}", extension);
        let mut all_files = fs::read_dir(prefix)?
            .filter_map(|res| -> Option<(String, ImageBuffer<Rgba<u8>, Vec<u8>>)> {
                let e = res.ok()?;
                let filename = e.file_name().into_string().ok()?;
                if !filename.to_lowercase().contains(&ext) {
                    return None;
                }
                Some((filename, image::open(e.path()).ok()?.into_rgba8()))
            })
            .collect::<Vec<(String, RgbaImage)>>();

        all_files.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let front_files = all_files
            .iter()
            .filter(|(filename, _path)| filename.contains("front_"))
            .collect::<Vec<&(String, RgbaImage)>>();
        let (first_file, _) = front_files
            .first()
            .ok_or(anyhow::anyhow!("Front patches missing"))?;
        let patch_count = first_file
            .replace(&ext, "")
            .replace("front_p", "")
            .split_once("_")
            .ok_or(anyhow::anyhow!("Failed to parse tile count"))?
            .0
            .parse::<u32>()?;

        let front_patches = front_files
            .iter()
            .map(|(_, im)| im.to_owned())
            .collect::<Vec<_>>();
        let back_patches = all_files
            .iter()
            .filter_map(|(fname, im)| {
                if fname.contains("back_") {
                    Some(im.to_owned())
                } else {
                    None
                }
            })
            .collect();
        let left_patches = all_files
            .iter()
            .filter_map(|(fname, im)| {
                if fname.contains("left_") {
                    Some(im.to_owned())
                } else {
                    None
                }
            })
            .collect();
        let right_patches = all_files
            .iter()
            .filter_map(|(fname, im)| {
                if fname.contains("right_") {
                    Some(im.to_owned())
                } else {
                    None
                }
            })
            .collect();
        let up_patches = all_files
            .iter()
            .filter_map(|(fname, im)| {
                if fname.contains("up_") {
                    Some(im.to_owned())
                } else {
                    None
                }
            })
            .collect();
        let down_patches = all_files
            .iter()
            .filter_map(|(fname, im)| {
                if fname.contains("down_") {
                    Some(im.to_owned())
                } else {
                    None
                }
            })
            .collect();
        Ok(Cube {
            front: stitch_image(&front_patches, patch_count)?,
            back: stitch_image(&back_patches, patch_count)?,
            left: stitch_image(&left_patches, patch_count)?,
            right: stitch_image(&right_patches, patch_count)?,
            up: stitch_image(&up_patches, patch_count)?,
            down: stitch_image(&down_patches, patch_count)?,
        })
    }
    pub fn from_directory(prefix: &str) -> anyhow::Result<Cube> {
        Ok(Cube {
            front: image::open(&format!("{}/front.jpg", prefix))?.into_rgba8(),
            back: image::open(&format!("{}/back.jpg", prefix))?.into_rgba8(),
            left: image::open(&format!("{}/left.jpg", prefix))?.into_rgba8(),
            right: image::open(&format!("{}/right.jpg", prefix))?.into_rgba8(),
            up: image::open(&format!("{}/up.jpg", prefix))?.into_rgba8(),
            down: image::open(&format!("{}/down.jpg", prefix))?.into_rgba8(),
        })
    }
    pub fn save(&self, prefix: &str) -> anyhow::Result<()> {
        self.front.save(format!("{}/front.jpg", prefix))?;
        self.back.save(format!("{}/back.jpg", prefix))?;
        self.left.save(format!("{}/left.jpg", prefix))?;
        self.right.save(format!("{}/right.jpg", prefix))?;
        self.up.save(format!("{}/up.jpg", prefix))?;
        self.down.save(format!("{}/down.jpg", prefix))?;
        Ok(())
    }

    pub fn save_patches(&self, prefix: &str, patch_size: u32) -> anyhow::Result<()> {
        let (front_patches, front_pieces) = split_image(&self.front, patch_size);
        front_patches.iter().enumerate().try_for_each(|(i, p)| {
            p.save(format!("{}/front_p{}_{}.jpg", prefix, front_pieces, i))
        })?;
        // back
        let (back_patches, back_pieces) = split_image(&self.back, patch_size);
        back_patches
            .iter()
            .enumerate()
            .try_for_each(|(i, p)| p.save(format!("{}/back_p{}_{}.jpg", prefix, back_pieces, i)))?;
        // left
        let (left_patches, left_pieces) = split_image(&self.left, patch_size);
        left_patches
            .iter()
            .enumerate()
            .try_for_each(|(i, p)| p.save(format!("{}/left_p{}_{}.jpg", prefix, left_pieces, i)))?;
        // right
        let (right_patches, right_pieces) = split_image(&self.right, patch_size);
        right_patches.iter().enumerate().try_for_each(|(i, p)| {
            p.save(format!("{}/right_p{}_{}.jpg", prefix, right_pieces, i))
        })?;
        // up
        let (up_patches, up_pieces) = split_image(&self.up, patch_size);
        up_patches
            .iter()
            .enumerate()
            .try_for_each(|(i, p)| p.save(format!("{}/up_p{}_{}.jpg", prefix, up_pieces, i)))?;
        // down
        let (down_patches, down_pieces) = split_image(&self.down, patch_size);
        down_patches
            .iter()
            .enumerate()
            .try_for_each(|(i, p)| p.save(format!("{}/down_p{}_{}.jpg", prefix, down_pieces, i)))?;

        Ok(())
    }
}

static FRONT: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
static BACK: Vector3<f32> = Vector3::new(0.0, -1.0, 0.0);
static LEFT: Vector3<f32> = Vector3::new(-1.0, 0.0, 0.0);
static RIGHT: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
static UP: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
static DOWN: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);

fn split_image(im: &RgbaImage, patch_size: u32) -> (Vec<RgbaImage>, u32) {
    let (width, height) = im.dimensions();
    let x_pieces = width / patch_size;
    let y_pieces = height / patch_size;
    let mut patches = vec![];
    for px in 0..x_pieces {
        for py in 0..y_pieces {
            patches.push(
                im.view(px * patch_size, py * patch_size, patch_size, patch_size)
                    .to_image(),
            );
        }
    }
    (patches, x_pieces)
}

fn stitch_image(patches: &Vec<RgbaImage>, x_pieces: u32) -> anyhow::Result<RgbaImage> {
    let example_patch = patches
        .first()
        .ok_or(anyhow::anyhow!("No patches to stitch"))?;
    let (width, height) = example_patch.dimensions();
    let full_width = width * x_pieces;
    let y_pieces = patches.len() as u32 / x_pieces;
    let full_height = height * y_pieces;
    let mut full = image::ImageBuffer::new(full_width, full_height);
    let mut patches_it = patches.iter();
    for px in 0..x_pieces {
        for py in 0..y_pieces {
            full.copy_from(
                patches_it
                    .next()
                    .ok_or(anyhow::anyhow!("not single patch"))?,
                px * width,
                py * height,
            )?;
        }
    }
    Ok(full)
}

fn face_pixel2ray(
    center: &Vector3<f32>,
    down: &Vector3<f32>,
    right: &Vector3<f32>,
    pixel: &Vector2<u32>,
    dimensions: &Vector2<u32>,
) -> Vector3<f32> {
    let origo2d = center - right - down; // we want to head to upper left corner thats why multiply with -0.5
    let x_scaled = (pixel.x as f32) / (dimensions.x as f32);
    let y_scaled = (pixel.y as f32) / (dimensions.y as f32);

    let point_in_face = origo2d + right * x_scaled * 2.0 + down * y_scaled * 2.0;
    point_in_face.normalize()
}

fn ray2equ_pixel(dimensions: &Vector2<u32>, ray: &Vector3<f32>) -> Vector2<u32> {
    // 2d angles:
    // x-direction (longitude)
    let longitude = PI + ray.x.atan2(ray.y); // will be between 0 to 2pi
    let pixel_x = (dimensions.x as f32 / (PI * 2.0)) * longitude;
    //let pixel_x = ((longitude / PI)*(dimensions.x as f32));
    // y-direction (latitude)
    let latitude = ray.z.acos(); // will be between 0 to pi
    let pixel_y = (dimensions.y as f32 / PI) * (latitude);

    Vector2::new(pixel_x as u32, pixel_y as u32)
}
// equi -> cube
// 1. for each cube face pixel to xyz-vector from center
// 2. normalize
// 3. get pixel value using the xyz-vector
fn equ2cube(source: DynamicImage, face_size: u32) -> anyhow::Result<Cube> {
    let _v = Vector3::new(1.0, 2.0, 3.0);
    let (width, height) = source.dimensions();
    let source_dims = Vector2::new(width, height);
    let face_dimensions = Vector2::new(face_size, face_size);

    // front
    let front_face = ImageBuffer::from_fn(face_size, face_size, |x, y| {
        let pixel = Vector2::new(x, y);
        let ray = face_pixel2ray(&FRONT, &DOWN, &RIGHT, &pixel, &face_dimensions);
        let coordinate = ray2equ_pixel(&source_dims, &ray);
        let p: Rgba<u8> = source.get_pixel(coordinate.x as u32, coordinate.y as u32);
        p
    });
    // left
    let left_face = ImageBuffer::from_fn(face_size, face_size, |x, y| {
        let pixel = Vector2::new(x, y);
        let ray = face_pixel2ray(&LEFT, &DOWN, &FRONT, &pixel, &face_dimensions);
        let coordinate = ray2equ_pixel(&source_dims, &ray);
        let p: Rgba<u8> = source.get_pixel(coordinate.x as u32, coordinate.y as u32);
        p
    });
    // right
    let right_face = ImageBuffer::from_fn(face_size, face_size, |x, y| {
        let pixel = Vector2::new(x, y);
        let ray = face_pixel2ray(&RIGHT, &DOWN, &BACK, &pixel, &face_dimensions);
        let coordinate = ray2equ_pixel(&source_dims, &ray);
        let p: Rgba<u8> = source.get_pixel(coordinate.x as u32, coordinate.y as u32);
        p
    });
    // back
    let back_face = ImageBuffer::from_fn(face_size, face_size, |x, y| {
        let pixel = Vector2::new(x, y);
        let ray = face_pixel2ray(&BACK, &DOWN, &LEFT, &pixel, &face_dimensions);
        let coordinate = ray2equ_pixel(&source_dims, &ray);
        let p: Rgba<u8> = source.get_pixel(coordinate.x as u32, coordinate.y as u32);
        p
    });
    // up
    let up_face = ImageBuffer::from_fn(face_size, face_size, |x, y| {
        let pixel = Vector2::new(x, y);
        let ray = face_pixel2ray(&UP, &BACK, &LEFT, &pixel, &face_dimensions);
        let coordinate = ray2equ_pixel(&source_dims, &ray);
        let p: Rgba<u8> = source.get_pixel(coordinate.x as u32, coordinate.y as u32);
        p
    });
    // down
    let down_face = ImageBuffer::from_fn(face_size, face_size, |x, y| {
        let pixel = Vector2::new(x, y);
        let ray = face_pixel2ray(&DOWN, &BACK, &RIGHT, &pixel, &face_dimensions);
        let coordinate = ray2equ_pixel(&source_dims, &ray);
        let p: Rgba<u8> = source.get_pixel(coordinate.x as u32, coordinate.y as u32);
        p
    });

    Ok(Cube {
        front: front_face,
        back: back_face,
        left: left_face,
        right: right_face,
        up: up_face,
        down: down_face,
    })
}

fn spherical2cartesian(radius: f32, inclination: f32, azimuth: f32) -> Vector3<f32> {
    // inclination = y-direction = latitude
    // azimuth = x-direction = longitude
    Vector3::new(
        radius * azimuth.cos() * inclination.sin(),
        radius * azimuth.sin() * inclination.sin(),
        radius * inclination.cos(),
    )
}

// cube -> equi
// 1. for each pixel in equ image calculate xyz-vector
// 2. See which face it hits (up,down,left,right,front,back)
// 3. See which pixel coordinate it is and sample the pixel
fn cube2equ(source: Cube) -> anyhow::Result<RgbaImage> {
    // let faces = vec![
    //     (FRONT, source.front, xz as Cget),
    //     (BACK, source.back, xz as Cget),
    //     (LEFT, source.left, yz as Cget),
    //     (RIGHT, source.right, yz as Cget),
    //     (UP, source.up, xy as Cget),
    //     (DOWN, source.down, xy as Cget)
    // ];
    let (p_width, p_height) = source.front.dimensions();
    let width = p_width * 4;
    let height = p_height * 2;
    let equ = ImageBuffer::from_fn(width, height, |x, y| -> Rgba<u8> {
        let x_prop = (x as f32) / (width as f32);
        let y_prop = (y as f32) / (height as f32);
        let longitude = x_prop * PI * 2.0 + PI / 2.0;
        let latitude = y_prop * PI;
        let ray = spherical2cartesian(1.0, latitude, longitude).normalize();
        let max_xyz = ray.x.abs().max(ray.y.abs()).max(ray.z.abs());
        let ray_u = ray / max_xyz;
        // println!("{:?}", ray_u);
        match (ray_u.x, ray_u.y, ray_u.z) {
            (s, ix, iy) if s == 1.0 => {
                let imx = ((ix + 1.0) / 2.0) * (source.right.width() - 1) as f32;
                let imy = ((iy + 1.0) / 2.0) * (source.right.height() - 1) as f32;
                source.right[(imx as u32, source.right.height() - 1 - imy as u32)]
            }
            (s, ix, iy) if s == -1.0 => {
                let imx = ((ix + 1.0) / 2.0) * (source.left.width() - 1) as f32;
                let imy = ((iy + 1.0) / 2.0) * (source.left.height() - 1) as f32;
                source.left[(
                    source.left.width() - 1 - imx as u32,
                    source.left.height() - 1 - imy as u32,
                )]
            }
            (ix, s, iy) if s == 1.0 => {
                let imx = ((ix + 1.0) / 2.0) * (source.back.width() - 1) as f32;
                let imy = ((iy + 1.0) / 2.0) * (source.back.height() - 1) as f32;
                source.back[(
                    source.back.width() - 1 - imx as u32,
                    source.back.height() - 1 - imy as u32,
                )]
            }
            (ix, s, iy) if s == -1.0 => {
                let imx = ((ix + 1.0) / 2.0) * (source.front.width() - 1) as f32;
                let imy = ((iy + 1.0) / 2.0) * (source.front.height() - 1) as f32;
                source.front[(imx as u32, source.front.height() - 1 - imy as u32)]
            }
            (ix, iy, s) if s == 1.0 => {
                let imx = ((ix + 1.0) / 2.0) * (source.up.width() - 1) as f32;
                let imy = ((iy + 1.0) / 2.0) * (source.up.height() - 1) as f32;
                source.up[(source.up.width() - 1 - imx as u32, imy as u32)]
            }
            (ix, iy, s) if s == -1.0 => {
                let imx = ((ix + 1.0) / 2.0) * (source.down.width() - 1) as f32;
                let imy = ((iy + 1.0) / 2.0) * (source.down.height() - 1) as f32;
                source.down[(imx as u32, imy as u32)]
            }
            (_, _, _) => Rgba([0u8, 0u8, 0u8, 0u8]),
        }
    });
    Ok(equ)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Split {
            input,
            tiles_output,
            patch_size,
            cubemap_faces_output,
        }) => {
            println!("[main]: Load image...");
            let source = image::open(input)?;
            println!("[main]: image loaded.");
            let (width, _height) = source.dimensions();
            let face_size = width / 4;
            println!("[Equ -> Cube]: processing...");
            let cube = equ2cube(source, face_size)?;
            println!("[Equ -> Cube]: done.");
            if cubemap_faces_output.is_none() && tiles_output.is_none() {
                println!("[main]: Warning, no output type specified");
            }
            if let Some(cubemap_out) = cubemap_faces_output {
                println!("[main]: Saving cubemap...");
                cube.save(&cubemap_out.to_string_lossy())?;
            }
            if let Some(output) = tiles_output {
                println!("[main]: Saving tiles...");
                cube.save_patches(&output.to_string_lossy(), *patch_size)?;
            }
        }
        Some(Commands::Stitch {
            input_dir,
            output,
            tiles,
            extension,
        }) => {
            println!("[main]: Loading cube");
            let cube = if *tiles {
                println!("[main]: Loading from tiles");
                Cube::from_directory_of_patches(&input_dir.to_string_lossy(), extension)?
            } else {
                println!("[main]: Loading from cubemap");
                Cube::from_directory(&input_dir.to_string_lossy())?
            };
            println!("[main]: Cube loaded.");
            println!("[Cube -> Equ]: converting cubemap to equirectangular");
            let restitched = cube2equ(cube)?;
            println!("[Cube -> Equ]: done.");
            println!("[main]: Save output image...");
            restitched.save(output)?;
            println!("[main]: image saved.");
        }
        None => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use cgmath::Vector2;

    use crate::{face_pixel2ray, ray2equ_pixel, BACK, DOWN, FRONT, LEFT, RIGHT, UP};

    #[test]
    fn front_face_center_point_should_point_to_front_ray() {
        let ray = face_pixel2ray(
            &FRONT,
            &DOWN,
            &RIGHT,
            &Vector2::new(5, 5),
            &Vector2::new(10, 10),
        );
        assert_eq!(ray, FRONT);
    }
    #[test]
    fn left_face_center_point_should_point_to_left_ray() {
        let ray = face_pixel2ray(
            &LEFT,
            &DOWN,
            &FRONT,
            &Vector2::new(5, 5),
            &Vector2::new(10, 10),
        );
        assert_eq!(ray, LEFT);
    }
    #[test]
    fn front_center_should_point_to_center_of_equirectangular_image() {
        // Assume 360 pixels wide, 180 pixels tall equi image
        let coordinate = ray2equ_pixel(&Vector2::new(360, 180), &FRONT);
        // Front center pixel should point to (180, 90) in equi image
        assert_eq!(coordinate, Vector2::new(180, 90));
    }
    #[test]
    fn front_down_center_should_point_to_center_bottom_of_equirectangular_image() {
        // Assume 360 pixels wide, 180 pixels tall equi image
        let coordinate = ray2equ_pixel(&Vector2::new(360, 180), &(FRONT + DOWN));
        // Front center pixel should point to (180, 179) in equi image
        assert_eq!(coordinate, Vector2::new(180, 179));
    }
    #[test]
    fn back_center_should_point_to_start_of_equirectangular_image() {
        // Assume 360 pixels wide, 180 pixels tall equi image
        let coordinate = ray2equ_pixel(&Vector2::new(360, 180), &BACK);
        // Front center pixel should point to (359, 90) in equi image
        assert_eq!(coordinate, Vector2::new(359, 90));
    }
    #[test]
    fn left_center_should_point_to_90_deg_of_equirectangular_image() {
        // Assume 360 pixels wide, 180 pixels tall equi image
        let coordinate = ray2equ_pixel(&Vector2::new(360, 180), &LEFT);
        // Left center pixel should point to (90, 90) in equi image
        assert_eq!(coordinate, Vector2::new(90, 90));
    }
    #[test]
    fn right_center_should_point_to_270_deg_of_equirectangular_image() {
        // Assume 360 pixels wide, 180 pixels tall equi image
        let coordinate = ray2equ_pixel(&Vector2::new(360, 180), &RIGHT);
        // Right center pixel should point to (270, 90) in equi image
        assert_eq!(coordinate, Vector2::new(270, 90));
    }
}
