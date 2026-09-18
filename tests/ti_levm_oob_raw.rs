use std::{fs::File, io::BufReader};

use titlv::{reader::FrameStreamReader, tlvs::StandardTlv, types::Frame};

#[test]
fn decodes_and_prints_ti_levm_oob_raw_frames() {
    let file = File::open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/ti_levm_oob_raw.bin"
    ))
    .expect("raw TLV capture should be present");
    let mut reader = FrameStreamReader::new(BufReader::new(file));
    let mut frames: Vec<Frame<StandardTlv>> = Vec::new();

    while let Ok(Some(frame)) = reader.read_frame::<StandardTlv>() {
        print_frame(&frame);
        frames.push(frame);
    }
}

fn print_frame(frame: &Frame<StandardTlv>) {
    println!(
        "Frame {{ number: {}, detected_objects: {}, tlvs: {} }}",
        frame.header.frame_number, frame.header.num_detected_obj, frame.header.num_tlvs,
    );

    for tlv in &frame.payload {
        match tlv {
            StandardTlv::PointCloud(point_cloud) => {
                println!("  PointCloud {{ points: [");
                for point in &point_cloud.points {
                    println!(
                        "    Point {{ x: {}, y: {}, z: {}, doppler: {} }},",
                        point.x, point.y, point.z, point.doppler,
                    );
                }
                println!("  ] }}");
            }
        }
    }
}
