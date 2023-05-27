use nalgebra::{Matrix3, Matrix4, Point3, Vector3};

pub fn rotate_x(angle: f32) -> Matrix4<f32> {
    let mut rot_x = Matrix4::zeros();

    rot_x[(0, 0)] = 1.0;
    rot_x[(3, 3)] = 1.0;

    rot_x[(1, 1)] = angle.cos();
    rot_x[(1, 2)] = -angle.sin();
    rot_x[(2, 1)] = angle.sin();
    rot_x[(2, 2)] = angle.cos();

    rot_x
}

pub fn rotate_y(angle: f32) -> Matrix4<f32> {
    let mut rot_y = Matrix4::zeros();

    rot_y[(1, 1)] = 1.0;
    rot_y[(3, 3)] = 1.0;

    rot_y[(0, 0)] = angle.cos();
    rot_y[(0, 2)] = angle.sin();
    rot_y[(2, 0)] = -angle.sin();
    rot_y[(2, 2)] = angle.cos();

    rot_y
}

pub fn rotate_z(angle: f32) -> Matrix4<f32> {
    let mut rot_z = Matrix4::zeros();

    rot_z[(2, 2)] = 1.0;
    rot_z[(3, 3)] = 1.0;

    rot_z[(0, 0)] = angle.cos();
    rot_z[(0, 1)] = -angle.sin();
    rot_z[(1, 0)] = angle.sin();
    rot_z[(1, 1)] = angle.cos();

    rot_z
}

pub fn rotate_axis(axis: Vector3<f32>, angle: f32) -> Matrix4<f32> {
    if axis.x == 0.0 && axis.y == 0.0 && axis.z == 0.0 {
        return Matrix4::identity();
    }

    let cross_matrix = axis.normalize().cross_matrix();
    let rotation_matrix = Matrix3::identity()
        + cross_matrix * angle.sin()
        + cross_matrix * cross_matrix * (1.0 - angle.cos());

    rotation_matrix.to_homogeneous()
}

pub fn translate(vector: Vector3<f32>) -> Matrix4<f32> {
    let mut translation = Matrix4::identity();

    translation[(0, 3)] = vector[0];
    translation[(1, 3)] = vector[1];
    translation[(2, 3)] = vector[2];

    translation
}

pub fn scale(sx: f32, sy: f32, sz: f32) -> Matrix4<f32> {
    let mut scaling = Matrix4::zeros();

    scaling[(0, 0)] = sx;
    scaling[(1, 1)] = sy;
    scaling[(2, 2)] = sz;
    scaling[(3, 3)] = 1.0;

    scaling
}

pub fn uniform_scale(sxyz: f32) -> Matrix4<f32> {
    scale(sxyz, sxyz, sxyz)
}

pub fn projection(fov: f32, aspect_ratio: f32, near_plane: f32, far_plane: f32) -> Matrix4<f32> {
    let mut projection_matrix = Matrix4::zeros();

    let ctg_fov_over_2 = 1.0 / (fov * 0.5).tan();
    let view_distance = far_plane - near_plane;

    projection_matrix[(0, 0)] = ctg_fov_over_2 / aspect_ratio;
    projection_matrix[(1, 1)] = ctg_fov_over_2;
    projection_matrix[(2, 2)] = -(far_plane + near_plane) / view_distance;
    projection_matrix[(2, 3)] = -2.0 * far_plane * near_plane / view_distance;
    projection_matrix[(3, 2)] = -1.0;

    projection_matrix
}

pub fn inverse_projection(
    fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
) -> Matrix4<f32> {
    let mut projection_matrix = Matrix4::zeros();

    let tan_fov_over_2 = (fov * 0.5).tan();
    let view_distance = far_plane - near_plane;

    projection_matrix[(0, 0)] = tan_fov_over_2 * aspect_ratio;
    projection_matrix[(1, 1)] = tan_fov_over_2;
    projection_matrix[(2, 3)] = -1.0;
    projection_matrix[(3, 2)] = view_distance / (-2.0 * far_plane * near_plane);
    projection_matrix[(3, 3)] =
        -(far_plane + near_plane) / view_distance * projection_matrix[(3, 2)];

    projection_matrix
}

pub fn look_at(observation: Point3<f32>, camera: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let to_camera = (camera - observation).normalize();
    let right = up.cross(&to_camera).normalize();
    let head = to_camera.cross(&right);

    Matrix4::from_columns(&[
        right.to_homogeneous(),
        head.to_homogeneous(),
        to_camera.to_homogeneous(),
        camera.to_homogeneous(),
    ])
    .try_inverse()
    .unwrap()
}
