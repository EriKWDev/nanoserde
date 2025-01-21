#[cfg(feature = "binary")]
pub mod bin {
    use crate::{DeBin, DeBinErr, SerBin};

    macro_rules! impl_glam_array {
        ($ty:ty) => {
            impl SerBin for $ty {
                fn ser_bin(&self, s: &mut Vec<u8>) {
                    let arr = self.to_array();
                    arr.ser_bin(s)
                }
            }
            impl DeBin for $ty {
                fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
                    match DeBin::de_bin(o, d) {
                        Ok(arr) => Ok(Self::from_array(arr)),
                        Err(err) => Err(err),
                    }
                }
            }
        };
    }
    impl_glam_array!(glam::Vec2);
    impl_glam_array!(glam::Vec3);
    impl_glam_array!(glam::Vec3A);
    impl_glam_array!(glam::Vec4);

    impl_glam_array!(glam::DVec2);
    impl_glam_array!(glam::DVec3);
    impl_glam_array!(glam::DVec4);

    impl_glam_array!(glam::IVec2);
    impl_glam_array!(glam::IVec3);
    impl_glam_array!(glam::IVec4);

    impl_glam_array!(glam::I8Vec2);
    impl_glam_array!(glam::I8Vec3);
    impl_glam_array!(glam::I8Vec4);

    impl_glam_array!(glam::I16Vec2);
    impl_glam_array!(glam::I16Vec3);
    impl_glam_array!(glam::I16Vec4);

    impl_glam_array!(glam::I64Vec2);
    impl_glam_array!(glam::I64Vec3);
    impl_glam_array!(glam::I64Vec4);

    impl_glam_array!(glam::UVec2);
    impl_glam_array!(glam::UVec3);
    impl_glam_array!(glam::UVec4);

    impl_glam_array!(glam::U8Vec2);
    impl_glam_array!(glam::U8Vec3);
    impl_glam_array!(glam::U8Vec4);

    impl_glam_array!(glam::U16Vec2);
    impl_glam_array!(glam::U16Vec3);
    impl_glam_array!(glam::U16Vec4);

    impl_glam_array!(glam::U64Vec2);
    impl_glam_array!(glam::U64Vec3);
    impl_glam_array!(glam::U64Vec4);

    impl_glam_array!(glam::Quat);
    impl_glam_array!(glam::DQuat);

    macro_rules! impl_glam_matrix_array {
        ($ty:ty) => {
            impl SerBin for $ty {
                fn ser_bin(&self, s: &mut Vec<u8>) {
                    let arr = self.to_cols_array();
                    arr.ser_bin(s)
                }
            }
            impl DeBin for $ty {
                fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
                    match DeBin::de_bin(o, d) {
                        Ok(arr) => Ok(Self::from_cols_array(&arr)),
                        Err(err) => Err(err),
                    }
                }
            }
        };
    }

    impl_glam_matrix_array!(glam::Mat2);
    impl_glam_matrix_array!(glam::Mat3);
    impl_glam_matrix_array!(glam::Mat3A);
    impl_glam_matrix_array!(glam::Mat4);

    impl_glam_matrix_array!(glam::DMat2);
    impl_glam_matrix_array!(glam::DMat3);
    impl_glam_matrix_array!(glam::DMat4);

    impl_glam_matrix_array!(glam::Affine2);
    impl_glam_matrix_array!(glam::Affine3A);

    impl_glam_matrix_array!(glam::DAffine2);
    impl_glam_matrix_array!(glam::DAffine3);
}
