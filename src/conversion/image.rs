use std::{error::Error, fs, path::Path};
use lopdf::{Document, Stream};
use image::ColorType;

/// Extracts embedded images directly into `[title]/public/img{num}.ext`
/// Returns a list of base filenames (e.g., ["img1.png", "img2.jpg"])
pub fn extract_pdf_images<P: AsRef<Path>>(
    pdf_path: P,
    public_dir: P,
) -> Result<Vec<String>, Box<dyn Error>> {
    let pdf_path = pdf_path.as_ref();
    let public_dir = public_dir.as_ref();
    
    // Ensure [title]/public exists
    fs::create_dir_all(public_dir)?;

    let doc = Document::load(pdf_path)?;
    let mut catalog_filenames = Vec::new();
    let mut img_counter = 1;

    for (_obj_id, object) in doc.objects.iter() {
        if let Ok(stream) = object.as_stream() {
            if is_image_stream(stream) {
                // Determine file extension based on filter
                let filter = stream.dict.get(b"Filter").ok().and_then(|f| f.as_name().ok());
                let ext = if filter == Some(b"DCTDecode") { "jpg" } else { "png" };

                let base_filename = format!("img{}.{}", img_counter, ext);
                let save_path = public_dir.join(&base_filename);

                if let Ok(()) = save_extracted_image(stream, &save_path, filter) {
                    catalog_filenames.push(base_filename);
                    img_counter += 1;
                }
            }
        }
    }

    Ok(catalog_filenames)
}

fn is_image_stream(stream: &Stream) -> bool {
    if let Ok(type_obj) = stream.dict.get(b"Type") {
        if let Ok(name) = type_obj.as_name() {
            if name != b"XObject" {
                return false;
            }
        }
    }

    if let Ok(subtype_obj) = stream.dict.get(b"Subtype") {
        if let Ok(name) = subtype_obj.as_name() {
            return name == b"Image";
        }
    }

    false
}

// voodoo slop function
fn save_extracted_image(
    stream: &Stream, 
    save_path: &Path, 
    filter: Option<&[u8]>
) -> Result<(), Box<dyn Error>> {
    // lopdf method to decompress zlib/Flate streams
    let raw_bytes = stream.decompressed_content()?;
    
    // save the raw bytes directly if jpeg
    if filter == Some(b"DCTDecode") {
        fs::write(save_path, &raw_bytes)?;
        return Ok(());
    }

    let width = stream.dict.get(b"Width")?.as_i64()? as u32;
    let height = stream.dict.get(b"Height")?.as_i64()? as u32;

    let color_space = stream.dict.get(b"ColorSpace")
        .ok()
        .and_then(|cs| cs.as_name().ok());

    match color_space {
        Some(b"DeviceRGB") => {
            image::save_buffer(save_path, &raw_bytes, width, height, ColorType::Rgb8)?;
        }
        Some(b"DeviceGray") => {
            image::save_buffer(save_path, &raw_bytes, width, height, ColorType::L8)?;
        }
        _ => {
            if raw_bytes.len() == (width * height * 4) as usize {
                image::save_buffer(save_path, &raw_bytes, width, height, ColorType::Rgba8)?;
            } else if raw_bytes.len() == (width * height * 3) as usize {
                image::save_buffer(save_path, &raw_bytes, width, height, ColorType::Rgb8)?;
            } else {
                return Err("Unsupported image buffer format".into());
            }
        }
    }

    Ok(())
}