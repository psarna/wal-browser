fn inspect(file: std::fs::File) -> std::io::Result<()> {
    use byteorder::{BigEndian, ReadBytesExt};
    use std::io::{Seek, SeekFrom};

    let file_len = file.metadata()?.len();
    let mut file = std::io::BufReader::new(file);

    if file_len < 32 {
        println!("WARNING: file too small to be a valid WAL file");
        return Ok(());
    }

    // Decode the header fields based on the provided specification
    let magic_number = file.read_u32::<BigEndian>()?;
    let file_format_version = file.read_u32::<BigEndian>()?;
    let page_size = file.read_u32::<BigEndian>()?;
    let seq = file.read_u32::<BigEndian>()?;
    let salt_1 = file.read_u32::<BigEndian>()?;
    let salt_2 = file.read_u32::<BigEndian>()?;
    let checksum_1 = file.read_u32::<BigEndian>()?;
    let checksum_2 = file.read_u32::<BigEndian>()?;

    if magic_number != 0x377f0682 {
        println!("WARNING: invalid magic number - not a WAL file or file corrupted?");
    }
    if file_format_version != 3007000 {
        println!("WARNING: invalid file format version - not a WAL file or file corrupted?");
    }
    // Print the decoded header
    println!("magic=0x{magic_number:x} version={file_format_version} page_size={page_size} seq={seq} salt={salt_1:08x}-{salt_2:08x} checksum={checksum_1:08x}-{checksum_2:08x}");

    let frame_count = (file_len - 32) / 4096;
    // Read and inspect each frame header
    for i in 0..frame_count {
        // Decode the frame header fields based on the provided specification
        let page_number = file.read_u32::<BigEndian>()?;
        let commit_size = file.read_u32::<BigEndian>()?;
        let salt_1 = file.read_u32::<BigEndian>()?;
        let salt_2 = file.read_u32::<BigEndian>()?;
        let checksum_1 = file.read_u32::<BigEndian>()?;
        let checksum_2 = file.read_u32::<BigEndian>()?;
        let _ = file.seek(SeekFrom::Current(page_size as i64));

        // Print the decoded information for each frame header
        println!("{i}: page={page_number} size_after={commit_size} salt={salt_1:08x}-{salt_2:08x} checksum={checksum_1:08x}-{checksum_2:08x}");
    }

    Ok(())
}

fn main() {
    let path = std::env::args().nth(1).expect("missing path");
    let file = std::fs::File::open(path).expect("failed to open file");

    inspect(file).expect("failed to inspect header");
}
