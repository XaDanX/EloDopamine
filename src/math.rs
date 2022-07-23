use crate::vector::{Vec2, Vec3};

pub fn multiply_matrices(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];
    result.clear();

    for i in 0..4 {
        for j in 0..4 {
            let mut c: f32 = 0.0;
            for k in 0..4 {
                c += a.get((i*4) + k).unwrap() * b.get((k*4) + j).unwrap();
            }
            result.push(c);
        }
    }
    return result;
}

pub fn w2s(pos: &Vec3, matrix: &[f32]) -> Option<Vec2> {
    let width: f32 = 1920.0;
    let height: f32 = 1080.0;
    let clip_coords_x = pos.x * matrix[0] + pos.y * matrix[4] + pos.z * matrix[8] + matrix[12];
    let clip_coords_y = pos.x * matrix[1] + pos.y * matrix[5] + pos.z * matrix[9] + matrix[13];
    let clip_coords_w = pos.x * matrix[3] + pos.y * matrix[7] + pos.z * matrix[11] + matrix[15];
    if clip_coords_w < 0.1 {
        return None
    }
    let m_x = clip_coords_x / clip_coords_w;
    let m_y = clip_coords_y / clip_coords_w;

    let out_x = (width / 2.0 * m_x) + (m_x + width / 2.0);
    let out_y = -(height / 2.0 * m_y) + (m_y + height / 2.0);
    return Some(Vec2::new(out_x, out_y));
}