# titlv

`titlv` decodes Type-Length-Value (TLV) frames emitted by Texas Instruments
mmWave radar sensors, including IWR6843 and IWR1443 devices.

## Add the dependency

Add `titlv` to your `Cargo.toml`:

```toml
[dependencies]
titlv = "0.1.0"
```

For a local checkout, use a path dependency instead:

```toml
[dependencies]
titlv = { path = "../titlv-rs" }
```

## Read frames

Wrap the radar byte stream in a `BufReader`, construct a `FrameStreamReader`,
and read frames as `StandardTlv`. The reader skips bytes before a frame's magic
word and returns `None` at end of stream.

```no_run
use std::{fs::File, io::BufReader};

use titlv::prelude::*;

fn main() -> Result<()> {
    let file = File::open("radar.bin")?;
    let mut reader = FrameStreamReader::new(BufReader::new(file));

    while let Some(frame) = reader.read_frame::<StandardTlv>()? {
        println!(
            "frame {}: {} supported TLVs",
            frame.header.frame_number,
            frame.payload.len(),
        );

        for tlv in frame.payload {
            match tlv {
                StandardTlv::PointCloud(cloud) => {
                    for point in cloud.points {
                        println!("x={}, y={}, z={}, doppler={}", point.x, point.y, point.z, point.doppler);
                    }
                }
                StandardTlv::ExtendedPointCloud(cloud) => {
                    println!("{} extended points", cloud.points.len());
                }
            }
        }
    }

    Ok(())
}
```

`FrameStreamReader` preserves its position between calls. Each returned frame
has a [`FrameHeader`](https://docs.rs/titlv/latest/titlv/types/struct.FrameHeader.html)
and a payload containing decoded TLVs. Unsupported TLVs are skipped when using
`StandardTlv`.

## Frame format

> **Note:** All packet lengths are in bytes.
> The primitives are encoded in little-endian format.

A frame starts with the magic word, followed by its header and body.

```mermaid
---
title: "TI radar frame"
---
packet
0-7: "Magic word ([u8; 8])"
8-39: "Frame header (32 bytes)"
40-71: "Frame body ([u8], variable length)"
```

The body begins at byte 40 and has a variable length.

```mermaid
---
title: "Frame header"
---
packet
0-3: "Version (u32, 4 bytes)"
4-7: "Total packet length (u32, 4 bytes)"
8-11: "Platform (u32, 4 bytes)"
12-15: "Frame number (u32, 4 bytes)"
16-19: "CPU timestamp (u32, 4 bytes)"
20-23: "Number of objects (u32, 4 bytes)"
24-27: "Number of TLVs (u32, 4 bytes)"
28-31: "Sub-frame number (u32, 4 bytes)"
```

All header fields are little-endian 32-bit values.

```mermaid
---
title: "TLV"
---
packet
0-3: "Type (u32, 4 bytes)"
4-7: "Length (u32, 4 bytes)"
8-39: "TLV value ([u8], variable length)"
```

Each TLV begins with an 8-byte header followed by its variable-length value.

## Custom TLVs

Derive `TlvReader` for a payload with fields in wire order. Derive `Tlv` and
provide the TI TLV type ID to validate the packet header before decoding.

```rust
use titlv::prelude::*;

#[derive(TlvReader)]
struct Temperature {
    degrees_celsius: f32,
}

#[derive(Tlv)]
#[tlv(type = 42)]
struct TemperatureTlv {
    degrees_celsius: f32,
}
```

Use the traits directly when the TLV type is known:

```rust,ignore
let temperature = TemperatureTlv::from_packet(packet)?;
```

## Errors

The crate returns `Result<T, TlvError>`. I/O failures, malformed packet data,
and unexpected TLV type IDs are reported as `TlvError` values.
