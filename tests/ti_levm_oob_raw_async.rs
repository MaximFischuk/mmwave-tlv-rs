use tokio::{fs::File, io::BufReader};

use titlv::{reader::AsyncFrameStreamReader, tlvs::StandardTlv, types::Frame};

#[tokio::test]
async fn decodes_and_prints_ti_levm_oob_raw_frames_async() {
    let file = File::open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/ti_levm_oob_raw.bin"
    ))
    .await
    .expect("raw TLV capture should be present");
    let mut reader = AsyncFrameStreamReader::new(BufReader::new(file));

    while let Ok(Some(frame)) = reader.read_frame::<StandardTlv>().await {
        print_frame(&frame);
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
            StandardTlv::ExtendedPointCloud(extended_point_cloud) => {
                println!("  ExtendedPointCloud {{ points: [");
                for point in &extended_point_cloud.points {
                    println!(
                        "    ExtendedPoint {{ x: {}, y: {}, z: {}, doppler: {}, snr: {}, noise: {} }},",
                        point.x, point.y, point.z, point.doppler, point.snr, point.noise,
                    );
                }
                println!("  ] }}");
            }
        }
    }
}
