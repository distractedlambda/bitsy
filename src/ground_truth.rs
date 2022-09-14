use crate::batch::Batch;

fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(c: f64) -> f64 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn split_argb(argb: u32) -> ([u8; 4]) {
    let a = (argb >> 24) as u8;
    let r = (argb >> 16) as u8;
    let g = (argb >> 8) as u8;
    let b = argb as u8;
    [a, r, g, b]
}

fn u8_to_norm(v: u8) -> f64 {
    (v as f64) / 255.0
}

fn norm_to_u8(v: f64) -> u8 {
    (v * 255.0).clamp(0.0, 255.0) as u8
}

pub fn compute_ground_truth<const N: usize>(
    result: &mut Batch<u32, N>,
    blend_src: &Batch<u32, N>,
    blend_dst: &Batch<u32, N>,
) {
    for i in 0..N {
        let [src_a, src_r, src_g, src_b] = split_argb(blend_src[i]);
        let [dst_a, dst_r, dst_g, dst_b] = split_argb(blend_dst[i]);

        let src_a_norm = u8_to_norm(src_a);
        let src_r_norm = u8_to_norm(src_r);
        let src_g_norm = u8_to_norm(src_g);
        let src_b_norm = u8_to_norm(src_b);

        let dst_a_norm = u8_to_norm(dst_a);
        let dst_r_norm = u8_to_norm(dst_r);
        let dst_g_norm = u8_to_norm(dst_g);
        let dst_b_norm = u8_to_norm(dst_b);

        let one_m_src_a_norm = 1.0 - src_a_norm;
        let res_a_norm = src_a_norm + dst_a_norm * one_m_src_a_norm;

        let src_r_linear = srgb_to_linear(src_r_norm);
        let src_g_linear = srgb_to_linear(src_g_norm);
        let src_b_linear = srgb_to_linear(src_b_norm);

        let dst_r_linear = srgb_to_linear(dst_r_norm);
        let dst_g_linear = srgb_to_linear(dst_g_norm);
        let dst_b_linear = srgb_to_linear(dst_b_norm);

        let src_weight = src_a_norm / res_a_norm;
        let dst_weight = dst_a_norm * one_m_src_a_norm / res_a_norm;
        let res_r_linear = src_r_linear * src_weight + dst_r_linear * dst_weight;
        let res_g_linear = src_g_linear * src_weight + dst_g_linear * dst_weight;
        let res_b_linear = src_b_linear * src_weight + dst_b_linear * dst_weight;

        let res_r_norm = linear_to_srgb(res_r_linear);
        let res_g_norm = linear_to_srgb(res_g_linear);
        let res_b_norm = linear_to_srgb(res_b_linear);

        let res_a_u8 = norm_to_u8(res_a_norm);
        let res_r_u8 = norm_to_u8(res_r_norm);
        let res_g_u8 = norm_to_u8(res_g_norm);
        let res_b_u8 = norm_to_u8(res_b_norm);

        let res_argb = ((res_a_u8 as u32) << 24)
            | ((res_r_u8 as u32) << 16)
            | ((res_g_u8 as u32) << 8)
            | (res_b_u8 as u32);

        result[i] = res_argb;
    }
}

pub fn total_loss<const N: usize>(truth: &Batch<u32, N>, prediction: &Batch<u32, N>) -> u64 {
    let mut total = 0;

    for i in 0..N {
        let truth_bytes = truth[i].to_ne_bytes();
        let prediction_bytes = prediction[i].to_ne_bytes();
        for j in 0..4 {
            total += truth_bytes[j].abs_diff(prediction_bytes[j]) as u64;
        }
    }

    total
}
