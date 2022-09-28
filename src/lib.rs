#![doc = include_str!("../README.md")]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![no_std]

pub trait FromBytes {
    const N: usize;

    fn from_be_bytes(bytes: [u8; Self::N]) -> Self;
    fn from_le_bytes(bytes: [u8; Self::N]) -> Self;
    fn from_ne_bytes(bytes: [u8; Self::N]) -> Self;
}

pub trait ToBytes {
    const N: usize;

    fn to_be_bytes(self) -> [u8; Self::N];
    fn to_le_bytes(self) -> [u8; Self::N];
    fn to_ne_bytes(self) -> [u8; Self::N];
}

#[macro_export]
macro_rules! delegate_from_bytes {
    ($($ty:ident)+) => ($(
        impl $crate::FromBytes for $ty {
            const N: usize = ::core::mem::size_of::<$ty>();

            fn from_be_bytes(bytes: [u8; ::core::mem::size_of::<$ty>()]) -> Self {
                Self::from_be_bytes(bytes)
            }

            fn from_le_bytes(bytes: [u8; ::core::mem::size_of::<$ty>()]) -> Self {
                Self::from_le_bytes(bytes)
            }

            fn from_ne_bytes(bytes: [u8; ::core::mem::size_of::<$ty>()]) -> Self {
                Self::from_ne_bytes(bytes)
            }
        }
    )+)
}

#[macro_export]
macro_rules! delegate_to_bytes {
    ($($ty:ident)+) => ($(
        impl $crate::ToBytes for $ty {
            const N: usize = ::core::mem::size_of::<$ty>();

            fn to_be_bytes(self) -> [u8; ::core::mem::size_of::<$ty>()] {
                self.to_be_bytes()
            }

            fn to_le_bytes(self) -> [u8; ::core::mem::size_of::<$ty>()] {
                self.to_le_bytes()
            }

            fn to_ne_bytes(self) -> [u8; ::core::mem::size_of::<$ty>()] {
                self.to_ne_bytes()
            }
        }
    )+)
}

delegate_from_bytes! { u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64 }
delegate_to_bytes! { u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64 }
