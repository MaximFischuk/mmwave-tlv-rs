#[derive(crate::Tlv)]
#[tlv(type = 1)]
pub struct PointCloud(pub Vec<Point>);

#[derive(crate::TlvReader)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub doppler: f32,
}

#[derive(crate::Tlv)]
pub struct PeopleTracking3dTlvS {
    pub tracks: TrackList,
    pub target_indexes: Vec<TargetIndexes>,
    pub track_heights: Vec<TrackHeights>,
    pub point_cloud: Option<CompressedSphericalPointCloud>,
}

#[derive(crate::Tlv)]
pub enum StandardTlv {
    PointCloud(PointCloud),
    PointCloudSideInfo(PointCloudSideInfo),
    RangeProfile(RangeProfile),
    NoiseProfile(NoiseProfile),
    Stats(Stats),
    TemperatureStats(TemperatureStats),
}

#[derive(crate::Tlv)]
pub enum PeopleTracking3dTlv {
    TrackList(TrackList),
    TargetIndexes(TargetIndexes),
    TrackHeights(TrackHeights),

    PointCloud(PointCloud),
}

// TLV type ID: 316
#[derive(crate::Tlv)]
#[tlv(type = 316)]
pub struct AdcSamples(pub Vec<AdcSample>);

#[derive(crate::TlvReader)]
pub struct AdcSample {
    pub value: i16,
}

// TLV type ID: 301
#[derive(crate::Tlv)]
#[tlv(type = 301)]
pub struct ExtendedPointCloud {
    pub units: ExtendedPointCloudUnits,
    pub points: Vec<ExtendedPoint>,
}

#[derive(crate::TlvReader)]
pub struct ExtendedPointCloudUnits {
    pub position: f32,
    pub doppler: f32,
    pub snr: f32,
    pub noise: f32,
    pub _reserved: [i16; 2],
}

#[derive(crate::TlvReader)]
pub struct ExtendedPoint {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub doppler: i16,
    pub snr: u8,
    pub noise: u8,
}

// TLV type ID: 315
#[derive(crate::Tlv)]
#[tlv(type = 315)]
pub struct EnhancedPresence(pub Vec<PresenceZone>);

#[derive(crate::TlvReader)]
pub struct PresenceZone {
    pub state: u8,
}

// TLV type ID: 7
#[derive(crate::Tlv)]
#[tlv(type = 7)]
pub struct PointCloudSideInfo {
    pub points: Vec<PointSideInfo>,
}

#[derive(crate::TlvReader)]
pub struct PointSideInfo {
    pub snr: u16,
    pub noise: u16,
}

// TLV type IDs: 2, 302, 303
#[derive(crate::Tlv)]
#[tlv(type = 2)]
pub struct RangeProfile {
    pub bins: Vec<RangeBin>,
}

#[derive(crate::TlvReader)]
pub struct RangeBin {
    pub value: u32,
}

// TLV type ID: 3
#[derive(crate::Tlv)]
#[tlv(type = 3)]
pub struct NoiseProfile {
    pub bins: Vec<NoiseBin>,
}

#[derive(crate::TlvReader)]
pub struct NoiseBin {
    pub value: u32,
}

// TLV type ID: 304
#[derive(crate::Tlv)]
#[tlv(type = 304)]
pub struct RangeAzimuthMajorHeatmap(pub Vec<HeatmapCell>);

// TLV type ID: 305
#[derive(crate::Tlv)]
#[tlv(type = 305)]
pub struct RangeAzimuthMinorHeatmap(pub Vec<HeatmapCell>);

// TLV type ID: 5
#[derive(crate::Tlv)]
#[tlv(type = 5)]
pub struct RangeDopplerHeatmap(pub Vec<HeatmapCell>);

#[derive(crate::TlvReader)]
pub struct HeatmapCell {
    pub value: u32,
}

// TLV type ID: 1030
#[derive(crate::Tlv)]
#[tlv(type = 1030)]
pub struct OccupancyStateMachine {
    pub occupied: [bool; 32],
}

// TLV type ID: 1000
#[derive(crate::Tlv)]
#[tlv(type = 1000)]
pub struct SphericalPointCloud(pub Vec<SphericalPoint>);

#[derive(crate::TlvReader)]
pub struct SphericalPoint {
    pub range: f32,
    pub azimuth: f32,
    pub elevation: f32,
    pub doppler: f32,
}

// TLV type ID: 1020
#[derive(crate::Tlv)]
#[tlv(type = 1020)]
pub struct CompressedSphericalPointCloud {
    pub units: CompressedPointUnits,
    pub points: Vec<CompressedSphericalPoint>,
}

#[derive(crate::TlvReader)]
pub struct CompressedPointUnits {
    pub elevation: f32,
    pub azimuth: f32,
    pub doppler: f32,
    pub range: f32,
    pub snr: f32,
}

#[derive(crate::TlvReader)]
pub struct CompressedSphericalPoint {
    pub elevation: i8,
    pub azimuth: i8,
    pub doppler: i16,
    pub range: u16,
    pub snr: u16,
}

// TLV type IDs: 1010, 308
#[derive(crate::Tlv)]
#[tlv(type = 1010)]
pub struct TrackList(pub Vec<Track>);

#[derive(crate::TlvReader)]
pub struct Track {
    pub id: u32,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub acceleration: [f32; 3],
    pub error_covariance: [f32; 16],
    pub g: f32,
    pub confidence: f32,
}

// TLV type ID: 1035
#[derive(crate::Tlv)]
#[tlv(type = 1035)]
pub struct TrackList2d(pub Vec<Track2d>);

#[derive(crate::TlvReader)]
pub struct Track2d {
    pub id: u32,
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub acceleration: [f32; 2],
    pub error_covariance: [f32; 9],
    pub g: f32,
    pub confidence: f32,
}

// TLV type ID: 1012
#[derive(crate::Tlv)]
#[tlv(type = 1012)]
pub struct TrackHeights {
    pub heights: Vec<TrackHeight>,
}

#[derive(crate::TlvReader)]
pub struct TrackHeight {
    pub id: u32,
    pub max_z: f32,
    pub min_z: f32,
}

// TLV type ID: 3000
#[derive(crate::Tlv)]
#[tlv(type = 3000)]
pub struct CameraTriggers {
    pub active_tracks: u32,
    pub trigger: u32,
    pub recording: u32,
    pub reserved: u32,
}

// TLV type IDs: 1011, 309
#[derive(crate::Tlv)]
#[tlv(type = 1011)]
pub struct TargetIndexes {
    pub indexes: Vec<TargetIndex>,
}

#[derive(crate::TlvReader)]
pub struct TargetIndex {
    pub value: u8,
}

// TLV type ID: 1040
#[derive(crate::Tlv)]
#[tlv(type = 1040)]
pub struct VitalSigns {
    pub id: u16,
    pub range_bin: u16,
    pub breath_deviation: f32,
    pub heart_rate: f32,
    pub breath_rate: f32,
    pub heart_waveform: [f32; 15],
    pub breath_waveform: [f32; 15],
}

// TLV type ID: 317
#[derive(crate::Tlv)]
#[tlv(type = 317)]
pub struct ClassifierOutput(pub Vec<ClassifierProbability>);

#[derive(crate::TlvReader)]
pub struct ClassifierProbability {
    pub value: u8,
}

// TLV type ID: 1031
#[derive(crate::Tlv)]
#[tlv(type = 1031)]
pub struct PointCloudClassification {
    pub class_type: u32,
    pub class_count: u32,
    pub probabilities: [f32; 5],
}

// TLV type ID: 1050
#[derive(crate::Tlv)]
#[tlv(type = 1050)]
pub struct GestureFeatures6843 {
    pub weighted_doppler: f32,
    pub weighted_positive_doppler: f32,
    pub weighted_negative_doppler: f32,
    pub weighted_range: f32,
    pub detections: f32,
    pub weighted_azimuth_mean: f32,
    pub weighted_elevation_mean: f32,
    pub azimuth_doppler_correlation: f32,
    pub weighted_azimuth_std_dev: f32,
    pub weighted_elevation_std_dev: f32,
}

// TLV type ID: 1051
#[derive(crate::Tlv)]
#[tlv(type = 1051)]
pub struct GestureProbabilities6843 {
    pub probabilities: [f32; 10],
}

// TLV type ID: 350
#[derive(crate::Tlv)]
#[tlv(type = 350)]
pub struct GestureFeatures6432 {
    pub values: [f32; 16],
}

// TLV type ID: 351
#[derive(crate::Tlv)]
#[tlv(type = 351)]
pub struct GestureClassifier6432 {
    pub gesture: i8,
}

// TLV type ID: 352
#[derive(crate::Tlv)]
#[tlv(type = 352)]
pub struct GesturePresence6432 {
    pub state: i8,
}

// TLV type ID: 353
#[derive(crate::Tlv)]
#[tlv(type = 353)]
pub struct PresenceThreshold {
    pub value: u32,
}

// TLV type ID: 1060
#[derive(crate::Tlv)]
#[tlv(type = 1060)]
pub struct ModeSwitchInfo {
    pub state: i8,
}

// TLV type ID: 1061
#[derive(crate::Tlv)]
#[tlv(type = 1061)]
pub struct CameraOn {
    pub state: i8,
}

// TLV type ID: 10312
#[derive(crate::Tlv)]
#[tlv(type = 10312)]
pub struct SurfaceClassification {
    pub value: f32,
}

// TLV type ID: 1033
#[derive(crate::Tlv)]
#[tlv(type = 1033)]
pub struct Velocity {
    pub value: f32,
    pub valid: bool,
}

// TLV type ID: 362
#[derive(crate::Tlv)]
#[tlv(type = 362)]
pub struct GestureMinorMotionPointCloud(pub Vec<Point>);

// TLV type ID: 318
#[derive(crate::Tlv)]
#[tlv(type = 318)]
pub struct RxChannelCompensation {
    pub coefficients: [f32; 13],
}

// TLV type ID: 306
#[derive(crate::Tlv)]
#[tlv(type = 306)]
pub struct ExtendedStats {
    pub inter_frame_processing_time: u32,
    pub transmit_output_time: u32,
    pub power_1v8: u16,
    pub power_3v3: u16,
    pub power_1v2: u16,
    pub power_1v2_rf: u16,
    pub temperature_rx: u16,
    pub temperature_tx: u16,
    pub temperature_pm: u16,
    pub temperature_digital: u16,
}

// TLV type ID: 1034
#[derive(crate::Tlv)]
#[tlv(type = 1034)]
pub struct ExtendedStatsBsd {
    pub inter_frame_processing_time: u32,
    pub transmit_output_time: u32,
    pub power_1v8: u16,
    pub power_3v3: u16,
    pub power_1v2: u16,
    pub power_1v2_rf: u16,
    pub temperature_rx: u16,
    pub temperature_tx: u16,
    pub temperature_pm: u16,
    pub temperature_digital: u16,
    pub ego_speed: f32,
    pub alpha_angle: f32,
}

// TLV type ID: 1062
#[derive(crate::Tlv)]
#[tlv(type = 1062)]
pub struct ClusterLocations(pub Vec<ClusterLocation>);

#[derive(crate::TlvReader)]
pub struct ClusterLocation {
    pub x: f32,
    pub y: f32,
}

// TLV type ID: 12
#[derive(crate::Tlv)]
#[tlv(type = 12)]
pub struct IntrusionDetectionInfo {
    pub signals: Vec<OccupancyBoxSignal>,
    pub decisions: Vec<OccupancyBoxDecision>,
}

#[derive(crate::TlvReader)]
pub struct OccupancyBoxSignal {
    pub value: f32,
}

#[derive(crate::TlvReader)]
pub struct OccupancyBoxDecision {
    pub value: u8,
}

// TLV type ID: 6
#[derive(crate::Tlv)]
#[tlv(type = 6)]
pub struct Stats {
    pub arm_processing_time: u32,
    pub uart_transmit_time: u32,
    pub dsp_processing_time: u32,
    pub power_1v8: u16,
    pub power_3v3: u16,
    pub power_1v2: u16,
    pub power_1v2_rf: u16,
}

// TLV type ID: 9
#[derive(crate::Tlv)]
#[tlv(type = 9)]
pub struct TemperatureStats {
    pub temp_report_valid: u32,
    pub time: u32,
    pub sensors: Vec<TemperatureSensor>,
}

#[derive(crate::TlvReader)]
pub struct TemperatureSensor {
    pub value: i16,
}

// TLV type ID: 3002
#[derive(crate::Tlv)]
#[tlv(type = 3002)]
pub struct OccupancyFeatures(pub Vec<OccupancyFeature>);

#[derive(crate::TlvReader)]
pub struct OccupancyFeature {
    pub value: f32,
}

// TLV type ID: 1041
#[derive(crate::Tlv)]
#[tlv(type = 1041)]
pub struct OccupancyClassificationResults(pub Vec<OccupancyClassificationResult>);

#[derive(crate::TlvReader)]
pub struct OccupancyClassificationResult {
    pub value: u8,
}

// TLV type ID: 1042
#[derive(crate::Tlv)]
#[tlv(type = 1042)]
pub struct OccupancyHeightResults(pub Vec<OccupancyHeightResult>);

#[derive(crate::TlvReader)]
pub struct OccupancyHeightResult {
    pub value: f32,
}

// TLV type ID: 1070
#[derive(crate::Tlv)]
#[tlv(type = 1070)]
pub struct LevelSensing(pub Vec<LevelSensingPoint>);

#[derive(crate::TlvReader)]
pub struct LevelSensingPoint {
    pub y: f32,
    pub snr: f32,
    pub noise: f32,
}

// TLV type ID: 1080
#[derive(crate::Tlv)]
#[tlv(type = 1080)]
pub struct SleepMonitoring {
    pub energy_average: u32,
    pub range: f32,
    pub activity_buffer_length_seconds: u32,
    pub activity_buffer_length: u32,
    pub activity_buffer_energy_average: u32,
    pub write_to_csv: u32,
    pub normalization_factor: u32,
    pub restless_class_lower_threshold: u32,
    pub restless_class_upper_threshold: u32,
}

// TLV type ID: 2008
#[derive(crate::Tlv)]
#[tlv(type = 2008)]
pub struct MacroDopplerFft(pub Vec<MacroDopplerValue>);

#[derive(crate::TlvReader)]
pub struct MacroDopplerValue {
    pub value: f32,
}

// TLV type ID: 368
#[derive(crate::Tlv)]
#[tlv(type = 368)]
pub struct DpcPoint {
    pub x: f32,
    pub y: f32,
    pub range: f32,
    pub velocity: f32,
    pub angle: f32,
    pub flags: u16,
    pub toward_radar: bool,
    pub gesture_region: bool,
    pub tracking_region: bool,
}

// TLV type ID: 3001
#[derive(crate::Tlv)]
#[tlv(type = 3001)]
pub struct MinorPointCloud {
    pub units: CompressedPointUnits,
    pub points: Vec<CompressedSphericalPoint>,
}

// TLV type ID: 410
#[derive(crate::Tlv)]
#[tlv(type = 410)]
pub struct ModelFlag {
    pub value: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Tlv, TlvReader,
        types::{Frame, Tag, TlvHeader, TlvPacket},
    };

    fn point_packet(type_id: u32) -> (TlvHeader, [u8; 16]) {
        let mut payload = [0; 16];
        for (index, value) in [1.0_f32, 2.0, 3.0, 4.0].iter().enumerate() {
            payload[index * 4..(index + 1) * 4].copy_from_slice(&value.to_le_bytes());
        }
        (
            TlvHeader {
                r#type: Tag::try_from(type_id).unwrap(),
                length: payload.len() as u32,
            },
            payload,
        )
    }

    #[test]
    fn derives_decode_tagged_struct_and_enum() {
        let (header, payload) = point_packet(1);
        let point_cloud = PointCloud::from_packet(TlvPacket {
            header: &header,
            payload: &payload,
        })
        .unwrap();
        assert_eq!(point_cloud.0.len(), 1);
        assert_eq!(point_cloud.0[0].doppler, 4.0);

        let tlv = StandardTlv::from_packet(TlvPacket {
            header: &header,
            payload: &payload,
        })
        .unwrap();
        assert!(matches!(tlv, StandardTlv::PointCloud(_)));
    }

    #[test]
    fn tagged_struct_rejects_wrong_type() {
        let (header, payload) = point_packet(2);
        assert!(
            PointCloud::from_packet(TlvPacket {
                header: &header,
                payload: &payload,
            })
            .is_err()
        );
    }

    #[test]
    fn frame_decodes_tlvs_into_struct_fields() {
        let mut bytes = Vec::new();
        for value in [0_u32, 0, 0, 1, 0, 0, 4, 0] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        let track = [0_u8; 112];
        bytes.extend_from_slice(&1010_u32.to_le_bytes());
        bytes.extend_from_slice(&(track.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&track);
        for indexes in [&[1_u8, 2][..], &[3_u8][..]] {
            bytes.extend_from_slice(&1011_u32.to_le_bytes());
            bytes.extend_from_slice(&(indexes.len() as u32).to_le_bytes());
            bytes.extend_from_slice(indexes);
        }
        let compressed_point_cloud = [0_u8; 20];
        bytes.extend_from_slice(&1020_u32.to_le_bytes());
        bytes.extend_from_slice(&(compressed_point_cloud.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&compressed_point_cloud);

        let frame = Frame::<PeopleTracking3dTlvS>::read(&mut &bytes[..]).unwrap();

        assert_eq!(frame.payload.tracks.0.len(), 1);
        assert_eq!(frame.payload.target_indexes.len(), 2);
        assert_eq!(frame.payload.target_indexes[0].indexes[1].value, 2);
        assert_eq!(frame.payload.target_indexes[1].indexes[0].value, 3);
        assert!(frame.payload.point_cloud.is_some());
    }

    #[test]
    fn frame_requires_direct_tlv_fields() {
        let bytes = [0_u8; 32];
        let error = match Frame::<PeopleTracking3dTlvS>::read(&mut &bytes[..]) {
            Err(error) => error,
            Ok(_) => panic!("frame without tracks should fail"),
        };

        assert!(
            matches!(&error, &crate::error::Error::MissingRequiredTlv),
            "{error:?}"
        );
    }
}
