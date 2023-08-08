fn inspect(file: std::fs::File) -> std::io::Result<()> {
    use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

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

    if magic_number != 0x377f0682 && magic_number != 0x377f0683 {
        println!("WARNING: invalid magic number - not a WAL file or file corrupted?");
    }
    let little_endian_checksum = magic_number == 0x377f0682;

    let mut checksum_acc_1 = 0u32;
    let mut checksum_acc_2 = 0u32;

    if little_endian_checksum {
        // Interpret input as little-endian
        checksum_acc_1 = checksum_acc_1
            .wrapping_add(magic_number.swap_bytes())
            .wrapping_add(checksum_acc_2);
        checksum_acc_2 = checksum_acc_2
            .wrapping_add(file_format_version.swap_bytes())
            .wrapping_add(checksum_acc_1);
        checksum_acc_1 = checksum_acc_1
            .wrapping_add(page_size.swap_bytes())
            .wrapping_add(checksum_acc_2);
        checksum_acc_2 = checksum_acc_2
            .wrapping_add(seq.swap_bytes())
            .wrapping_add(checksum_acc_1);
        checksum_acc_1 = checksum_acc_1
            .wrapping_add(salt_1.swap_bytes())
            .wrapping_add(checksum_acc_2);
        checksum_acc_2 = checksum_acc_2
            .wrapping_add(salt_2.swap_bytes())
            .wrapping_add(checksum_acc_1);
    } else {
        checksum_acc_1 = checksum_acc_1
            .wrapping_add(magic_number)
            .wrapping_add(checksum_acc_2);
        checksum_acc_2 = checksum_acc_2
            .wrapping_add(file_format_version)
            .wrapping_add(checksum_acc_1);
        checksum_acc_1 = checksum_acc_1
            .wrapping_add(page_size)
            .wrapping_add(checksum_acc_2);
        checksum_acc_2 = checksum_acc_2
            .wrapping_add(seq)
            .wrapping_add(checksum_acc_1);
        checksum_acc_1 = checksum_acc_1
            .wrapping_add(salt_1)
            .wrapping_add(checksum_acc_2);
        checksum_acc_2 = checksum_acc_2
            .wrapping_add(salt_2)
            .wrapping_add(checksum_acc_1);
    }

    if checksum_acc_1 != checksum_1 || checksum_acc_2 != checksum_2 {
        println!("WARNING: checksum mismatch - file corrupted? {checksum_1:08x}-{checksum_2:08x} != {checksum_acc_1:08x}-{checksum_acc_2:08x}");
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
        let frame_salt_1 = file.read_u32::<BigEndian>()?;
        let frame_salt_2 = file.read_u32::<BigEndian>()?;
        let checksum_1 = file.read_u32::<BigEndian>()?;
        let checksum_2 = file.read_u32::<BigEndian>()?;

        if little_endian_checksum {
            checksum_acc_1 = checksum_acc_1
                .wrapping_add(page_number.swap_bytes())
                .wrapping_add(checksum_acc_2);
            checksum_acc_2 = checksum_acc_2
                .wrapping_add(commit_size.swap_bytes())
                .wrapping_add(checksum_acc_1);
        } else {
            checksum_acc_1 = checksum_acc_1
                .wrapping_add(page_number)
                .wrapping_add(checksum_acc_2);
            checksum_acc_2 = checksum_acc_2
                .wrapping_add(commit_size)
                .wrapping_add(checksum_acc_1);
        }

        for _ in 0..(page_size / 8) {
            let (x1, x2) = if little_endian_checksum {
                (
                    file.read_u32::<LittleEndian>()?,
                    file.read_u32::<LittleEndian>()?,
                )
            } else {
                (file.read_u32::<BigEndian>()?, file.read_u32::<BigEndian>()?)
            };
            checksum_acc_1 = checksum_acc_1.wrapping_add(x1).wrapping_add(checksum_acc_2);
            checksum_acc_2 = checksum_acc_2.wrapping_add(x2).wrapping_add(checksum_acc_1);
        }

        if salt_1 != frame_salt_1 || salt_2 != frame_salt_2 {
            println!("--- WAL end (it contains more frames with mismatched SALT values, which means they're leftovers from previous checkpoints: {frame_salt_1:08x}-{frame_salt_2:08x})");
            break;
        }

        // Print the decoded information for each frame header
        println!("{i}: page={page_number} size_after={commit_size}");
        if checksum_acc_1 != checksum_1 || checksum_acc_2 != checksum_2 {
            println!("WARNING: checksum mismatch - file corrupted? {checksum_1:08x}-{checksum_2:08x} != {checksum_acc_1:08x}-{checksum_acc_2:08x}");
        }
    }

    Ok(())
}

fn main() {
    let path = std::env::args().nth(1).expect("missing path");
    let file = std::fs::File::open(path).expect("failed to open file");

    inspect(file).expect("failed to inspect header");
}
