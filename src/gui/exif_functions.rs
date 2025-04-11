use nom_exif::*;

pub fn get_exif_data(path: &str) -> Result<Exif, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut reader = std::io::BufReader::new(file);
    let exif = Exif::from_reader(&mut reader).map_err(|e| e.to_string())?;
    Ok(exif)
}