// pub fn lttb(data: &[f32], threshold: usize) -> Vec<f32> {
//     assert_eq!(threshold <= 2 || threshold >= data.len(), false, "threshold not well defined");
//
//     let mut sampled = Vec::with_capacity(threshold);
//
//     // Bucket size. Leave room for start and end data points.
//     let every = ((data.len() - 2) as f32) / ((threshold - 2) as f32);
//
//     // Initially a is the first point in the triangle.
//     let mut a = 0;
//
//     // Always add the first point.
//     sampled.push(data[a]);
//
//     for i in 0..threshold - 2 {
//         // Calculate point average for next bucket (containing c).
//         let mut avg_x = 0f32;
//         let mut avg_y = 0f32;
//
//         let avg_range_start = (((i + 1) as f32) * every) as usize + 1;
//
//         let mut end = (((i + 2) as f32) * every) as usize + 1;
//         if end >= data.len() {
//             end = data.len();
//         }
//         let avg_range_end = end;
//
//         let avg_range_length = (avg_range_end - avg_range_start) as f32;
//
//         for i in 0..(avg_range_end - avg_range_start) {
//             let idx = avg_range_start + i;
//
//             avg_x += idx as f32;
//             avg_y += data[idx];
//         }
//         avg_x /= avg_range_length;
//         avg_y /= avg_range_length;
//
//         // Get the range for this bucket.
//         let range_offs = ((i as f32) * every) as usize + 1;
//         let range_to = (((i + 1) as f32) * every) as usize + 1;
//
//         // Point a.
//         let point_a_x = a as f32;
//         let point_a_y = data[a];
//
//         let mut max_area = -1f32;
//         let mut next_a = range_offs;
//         for i in 0..(range_to - range_offs) {
//             let idx = (range_offs + i) as usize;
//             // Calculate triangle area over three buckets.
//             let area = ((point_a_x - avg_x) * (data[idx] - point_a_y)
//                 - (point_a_x - idx as f32) * (avg_y - point_a_y))
//                 .abs()
//                 * 0.5;
//             if area > max_area {
//                 max_area = area;
//                 next_a = idx; // Next a is this b.
//             }
//         }
//
//         sampled.push(data[next_a]); // Pick this point from the bucket.
//         a = next_a; // This a is the next a (chosen b).
//     }
//
//     // Always add the last point.
//     sampled.push(data[data.len() - 1]);
//
//     sampled
// }


/**
Convert bytes to physical value

## Structure of each package from D module
```
0| 1  0 E1 E1 E1 E1 E1 E1
1| 0 E1 E1 E1 E1 E1 E1 E1
2| 0 E1 E1 E1 E1 E1 E2 E2
3| 0 E2 E2 E2 E2 E2 E2 E2
4| 0 E2 E2 E2 E2 E2 E2 E2
5| 0 E2 E2 E3 E3 E3 E3 E3
6| 0 E3 E3 E3 E3 E3 E3 E3
7| 0 E3 E3 E3 E3 E3 E3  0
```
 **/
pub fn bytes_to_physical_normal(raw_data: &[u8], package_nums: usize) -> Vec<f32> {
    const LSB: f32 = 0.0051116943359375;
    const OFFSET: f32 = 670.0;

    let size = package_nums * 128;

    let mut physical = vec![0.0f32; size * 3];

    for (i, chunk) in raw_data.chunks(8).enumerate() {
        let value = u64::from_be_bytes(chunk.try_into().unwrap());

        let e1_val = (((value >> 56) & 0x3F) << 12
            | ((value >> 48) & 0x7F) << 5
            | ((value >> 42) & 0x1F)) as f32
            * LSB
            - OFFSET;
        let e2_val = (((value >> 40) & 0x3) << 16
            | ((value >> 32) & 0x7F) << 9
            | ((value >> 24) & 0x7F) << 2
            | ((value >> 21) & 0x3)) as f32
            * LSB
            - OFFSET;
        let e3_val = (((value >> 16) & 0x1F) << 13
            | ((value >> 8) & 0x7F) << 6
            | ((value >> 1) & 0x3F)) as f32
            * LSB
            - OFFSET;

        physical[i] = e1_val;
        physical[i + size] = e2_val;
        physical[i + size * 2] = e3_val;
    }

    physical
        .chunks(size)
        .map(|lead|
        lead
            .chunks(128)
            .map(|chunk| chunk.iter().sum::<f32>() / 128f32)
        )
        .flatten()
        .collect::<Vec<f32>>()
}


pub fn bytes_to_physical_hall(raw_data: &[u8], package_nums: usize) -> Vec<f32> {
    const LSB: f32 = 0.0051116943359375;

    let size = package_nums * 128;

    let mut hall = vec![0.0f32; size * 3];

    for (i, chunk) in raw_data.chunks(20).enumerate() {
        let package = &chunk[14..20];

        let gx = (((package[0] & 0b00000001) as u16) << 11
            | ((package[1] & 0b01111111) as u16) << 4
            | (package[2] & 0b01111000) as u16) as f32 * LSB;


        let gy = (((package[2] & 0b00000111) as u16) << 9
            | ((package[3] & 0b01111111) as u16) << 2
            | (package[4] & 0b01100000) as u16) as f32 * LSB;

        let gz = (((package[4] & 0b00011111) as u16) << 7
            | ((package[5] & 0b01111111) as u16) << 5) as f32 * LSB;

        hall[i] = gx;
        hall[i + size] = gy;
        hall[i + size * 2] = gz;
    }

    hall
        .chunks(size)
        .map(|lead|
        lead
            .chunks(128)
            .map(|chunk| chunk.iter().sum::<f32>() / 128f32)
        )
        .flatten()
        .collect::<Vec<f32>>()
}
