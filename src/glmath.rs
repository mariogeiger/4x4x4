use std::f32;
use std::ops;

#[derive(Copy, Clone)]
pub struct Mat4(pub [[f32; 4]; 4]);

#[derive(Copy, Clone)]
pub struct Mat3(pub [[f32; 3]; 3]);

#[allow(dead_code)]
impl Mat3 {
    /*
    00 10 20
    01 11 21
    02 12 22
    */
    pub fn identity() -> Mat3 {
        Mat3([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }
    pub fn inverse(&self) -> Option<Mat3> {
        let d = self.det();
        if d == 0.0f32 {
            None
        } else {
            let invdet = 1.0 / d;
            let mut result = Mat3::identity();
            result.0[0][0] = (self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2]) * invdet;
            result.0[1][0] = -(self.0[0][1] * self.0[2][2] - self.0[0][2] * self.0[2][1]) * invdet;
            result.0[2][0] = (self.0[0][1] * self.0[1][2] - self.0[0][2] * self.0[1][1]) * invdet;
            result.0[0][1] = -(self.0[1][0] * self.0[2][2] - self.0[1][2] * self.0[2][0]) * invdet;
            result.0[1][1] = (self.0[0][0] * self.0[2][2] - self.0[0][2] * self.0[2][0]) * invdet;
            result.0[2][1] = -(self.0[0][0] * self.0[1][2] - self.0[1][0] * self.0[0][2]) * invdet;
            result.0[0][2] = (self.0[1][0] * self.0[2][1] - self.0[2][0] * self.0[1][1]) * invdet;
            result.0[1][2] = -(self.0[0][0] * self.0[2][1] - self.0[2][0] * self.0[0][1]) * invdet;
            result.0[2][2] = (self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1]) * invdet;
            Some(result)
        }
    }
    pub fn det(&self) -> f32 {
        self.0[0][0] * (self.0[1][1] * self.0[2][2] - self.0[1][2] * self.0[2][1])
            - self.0[0][1] * (self.0[1][0] * self.0[2][2] - self.0[1][2] * self.0[2][0])
            + self.0[0][2] * (self.0[1][0] * self.0[2][1] - self.0[2][0] * self.0[1][1])
    }
}

#[allow(dead_code)]
impl Mat4 {
    /*
    00 10 20 30
    01 11 21 31
    02 12 22 32
    03 13 23 33
    */
    pub fn identity() -> Mat4 {
        Mat4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn scale(s: f32) -> Mat4 {
        Mat4([
            [s, 0.0, 0.0, 0.0],
            [0.0, s, 0.0, 0.0],
            [0.0, 0.0, s, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, z, 1.0],
        ])
    }
    pub fn rotation(a: f32, mut x: f32, mut y: f32, mut z: f32) -> Mat4 {
        // http://fr.wikipedia.org/wiki/Matrice_de_rotation

        let c = a.cos();
        let s = a.sin();
        let ic = 1.0f32 - c;

        let mut len = x * x + y * y + z * z;
        if len != 1.0 && len != 0.0 {
            len = len.sqrt();
            x /= len;
            y /= len;
            z /= len;
        }

        Mat4([
            [x * x * ic + c, x * y * ic - z * s, x * z * ic + y * s, 0.0],
            [y * x * ic + z * s, y * y * ic + c, y * z * ic - x * s, 0.0],
            [x * z * ic - y * s, y * z * ic + x * s, z * z * ic + c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn perspective(aspect_ratio: f32, fov: f32, znear: f32, zfar: f32) -> Mat4 {
        let f = 1.0 / (fov / 2.0).tan();

        Mat4([
            [f / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, -(zfar + znear) / (zfar - znear), -1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ])
    }
    pub fn to_mat3(&self) -> Mat3 {
        Mat3([
            [self.0[0][0], self.0[0][1], self.0[0][2]],
            [self.0[1][0], self.0[1][1], self.0[1][2]],
            [self.0[2][0], self.0[2][1], self.0[2][2]],
        ])
    }
    pub fn normal_matrix(&self) -> Option<Mat3> {
        self.to_mat3().inverse()
    }
}

impl ops::Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, _rhs: Mat4) -> Mat4 {
        let mut x = [[0.0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    x[j][i] += self.0[k][i] * _rhs.0[j][k];
                }
            }
        }
        Mat4(x)
    }
}
