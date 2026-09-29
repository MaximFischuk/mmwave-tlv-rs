use std::{fs::File, io::BufReader};

use mmwave_tlv::{reader::FrameStreamReader, tlvs::StandardTlv, types::Frame};

#[test]
fn decodes_and_prints_levm_oob_raw_frames() {
    let file = File::open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/levm_oob_raw.bin"
    ))
    .expect("raw TLV capture should be present");
    let mut reader = FrameStreamReader::new(BufReader::new(file));

    while let Ok(Some(frame)) = reader.read_frame::<Vec<StandardTlv>>() {
        print_frame(&frame);
    }
}

fn print_frame(frame: &Frame<Vec<StandardTlv>>) {
    println!(
        "Frame {{ number: {}, detected_objects: {}, tlvs: {} }}",
        frame.header.frame_number, frame.header.num_detected_obj, frame.header.num_tlvs,
    );

    for tlv in &frame.payload {
        match tlv {
            StandardTlv::PointCloud(point_cloud) => {
                println!("  PointCloud {{ points: [");
                for point in &point_cloud.0 {
                    println!(
                        "    Point {{ x: {}, y: {}, z: {}, doppler: {} }},",
                        point.x, point.y, point.z, point.doppler,
                    );
                }
                println!("  ] }}");
            }
            StandardTlv::PointCloudSideInfo(side_info) => {
                println!("  PointCloudSideInfo {{ points: [");
                for point in side_info.iter() {
                    println!(
                        "    PointSideInfo {{ snr: {}, noise: {} }},",
                        point.snr, point.noise,
                    );
                }
                println!("  ] }}");
            }
            StandardTlv::RangeProfile(range_profile) => {
                println!("  RangeProfile {{ bins: [");
                for bin in range_profile.iter() {
                    println!("    RangeBin {{ value: {} }},", bin.as_ref());
                }
                println!("  ] }}");
            }
            StandardTlv::NoiseProfile(noise_profile) => {
                println!("  NoiseProfile {{ bins: [");
                for bin in noise_profile.iter() {
                    println!("    NoiseBin {{ value: {} }},", bin.as_ref());
                }
                println!("  ] }}");
            }
            StandardTlv::Stats(stats) => {
                println!(
                    "  Stats {{ arm_processing_time: {}, uart_transmit_time: {}, dsp_processing_time: {}, power_1v8: {}, power_3v3: {}, power_1v2: {}, power_1v2_rf: {} }}",
                    stats.arm_processing_time,
                    stats.uart_transmit_time,
                    stats.dsp_processing_time,
                    stats.power_1v8,
                    stats.power_3v3,
                    stats.power_1v2,
                    stats.power_1v2_rf,
                );
            }
            StandardTlv::TemperatureStats(temperature_stats) => {
                println!(
                    "  TemperatureStats {{ temp_report_valid: {}, time: {}, sensors: [",
                    temperature_stats.temp_report_valid, temperature_stats.time,
                );
                for sensor in temperature_stats.sensors.iter() {
                    println!("    TemperatureSensor {{ value: {} }},", sensor.as_ref());
                }
                println!("  ] }}");
            }
        }
    }
}
