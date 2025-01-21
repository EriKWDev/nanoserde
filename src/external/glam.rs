#[cfg(feature = "binary")]
use crate::{DeBin, DeBinErr, SerBin};

#[cfg(feature = "binary")]
macro_rules! impl_glam_vec_bin {
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

#[cfg(feature = "binary")]
macro_rules! impl_glam_matrix_bin {
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

#[cfg(feature = "ron")]
use crate::{DeRon, DeRonErr, DeRonState, SerRon, SerRonState};
#[cfg(feature = "ron")]
use core::str::Chars;

#[cfg(feature = "ron")]
macro_rules! impl_glam_vec_ron {
    ($ty:ty) => {
        impl SerRon for $ty {
            fn ser_ron(&self, indent_level: usize, state: &mut SerRonState) {
                let arr = self.to_array();
                arr.ser_ron(indent_level, state)
            }
        }
        impl DeRon for $ty {
            fn de_ron(state: &mut DeRonState, input: &mut Chars) -> Result<Self, DeRonErr> {
                match DeRon::de_ron(state, input) {
                    Ok(arr) => Ok(Self::from_array(arr)),
                    Err(err) => Err(err),
                }
            }
        }
    };
}

#[cfg(feature = "ron")]
macro_rules! impl_glam_matrix_ron {
    ($ty:ty) => {
        impl SerRon for $ty {
            fn ser_ron(&self, indent_level: usize, state: &mut SerRonState) {
                let arr = self.to_cols_array();
                arr.ser_ron(indent_level, state)
            }
        }
        impl DeRon for $ty {
            fn de_ron(state: &mut DeRonState, input: &mut Chars) -> Result<Self, DeRonErr> {
                match DeRon::de_ron(state, input) {
                    Ok(arr) => Ok(Self::from_cols_array(&arr)),
                    Err(err) => Err(err),
                }
            }
        }
    };
}

macro_rules! impl_glam_vec {
    ($ty:ty) => {
        #[cfg(feature = "binary")]
        impl_glam_vec_bin!($ty);
        #[cfg(feature = "ron")]
        impl_glam_vec_ron!($ty);
    };
}

macro_rules! impl_glam_quat {
    ($ty:ty) => {
        #[cfg(feature = "binary")]
        impl_glam_vec_bin!($ty);
        #[cfg(feature = "ron")]
        impl_glam_vec_ron!($ty);
    };
}

macro_rules! impl_glam_matrix {
    ($ty:ty) => {
        #[cfg(feature = "binary")]
        impl_glam_matrix_bin!($ty);
        #[cfg(feature = "ron")]
        impl_glam_matrix_ron!($ty);
    };
}

impl_glam_matrix!(glam::Mat2);
impl_glam_matrix!(glam::Mat3);
impl_glam_matrix!(glam::Mat3A);
impl_glam_matrix!(glam::Mat4);

impl_glam_matrix!(glam::DMat2);
impl_glam_matrix!(glam::DMat3);
impl_glam_matrix!(glam::DMat4);

impl_glam_matrix!(glam::Affine2);
impl_glam_matrix!(glam::Affine3A);

impl_glam_matrix!(glam::DAffine2);
impl_glam_matrix!(glam::DAffine3);

impl_glam_vec!(glam::Vec2);
impl_glam_vec!(glam::Vec3);
impl_glam_vec!(glam::Vec3A);
impl_glam_vec!(glam::Vec4);

impl_glam_vec!(glam::DVec2);
impl_glam_vec!(glam::DVec3);
impl_glam_vec!(glam::DVec4);

impl_glam_vec!(glam::IVec2);
impl_glam_vec!(glam::IVec3);
impl_glam_vec!(glam::IVec4);

impl_glam_vec!(glam::I8Vec2);
impl_glam_vec!(glam::I8Vec3);
impl_glam_vec!(glam::I8Vec4);

impl_glam_vec!(glam::I16Vec2);
impl_glam_vec!(glam::I16Vec3);
impl_glam_vec!(glam::I16Vec4);

impl_glam_vec!(glam::I64Vec2);
impl_glam_vec!(glam::I64Vec3);
impl_glam_vec!(glam::I64Vec4);

impl_glam_vec!(glam::UVec2);
impl_glam_vec!(glam::UVec3);
impl_glam_vec!(glam::UVec4);

impl_glam_vec!(glam::U8Vec2);
impl_glam_vec!(glam::U8Vec3);
impl_glam_vec!(glam::U8Vec4);

impl_glam_vec!(glam::U16Vec2);
impl_glam_vec!(glam::U16Vec3);
impl_glam_vec!(glam::U16Vec4);

impl_glam_vec!(glam::U64Vec2);
impl_glam_vec!(glam::U64Vec3);
impl_glam_vec!(glam::U64Vec4);

impl_glam_quat!(glam::Quat);
impl_glam_quat!(glam::DQuat);
