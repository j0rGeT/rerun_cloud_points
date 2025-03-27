use rerun::RecordingStreamBuilder;
use csv::ReaderBuilder;
use std::path::Path;

fn load_cloud_points(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化Rerun
    let rec_stream = RecordingStreamBuilder::new("csv_point_cloud").spawn()?;

    // 读取txt文件（格式：x,y,z,r,g,b）
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    let mut positions = Vec::new();
    let mut colors = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let parts: Vec<&str> = record[0].split(' ').collect();
        let x: f32 = parts[0].parse()?;
        let y: f32 = parts[1].parse()?;
        let z: f32 = parts[2].parse()?;
        let r: u8 = parts[4].parse()?;
        let g: u8 = parts[5].parse()?;
        let b: u8 = parts[6].parse()?;
        positions.push([x, y, z]);
        colors.push([r, g, b]);
    }

    //let rec = rerun::RecordingStream::global(rerun::StoreKind::Recording)?;
    rec_stream.log("points", &rerun::archetypes::Points3D::new(positions).with_colors(colors))?;
    
    
    Ok(())
}

fn main() {
    let path = Path::new("points.txt");
    load_cloud_points(path).unwrap();
}