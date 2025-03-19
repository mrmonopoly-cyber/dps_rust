use core::mem::size_of;
use crate::common::DataGenericType::*;

pub trait DpsType: PartialOrd + Sized {
    fn get_type_category(&self) -> crate::common::DataGenericType {
        crate::common::DataGenericType::Unsigned
    }

    fn update(&mut self, data: &[u8]) -> Result<(), &str>;
}

macro_rules! impl_dps_data_type {
    ($t:ty, $te:expr) => {
        impl DpsType for $t {
            fn get_type_category(&self) -> crate::common::DataGenericType {
                $te
            }
            fn update(&mut self, data: &[u8]) -> Result<(), &str> {
                if data.len() ==  size_of::<$t>(){
                    let mut new_v = self.to_be_bytes();
                    new_v.copy_from_slice(data);
                    *self = <$t>::from_le_bytes(new_v);
                    Ok(())
                } else {
                    Err("array too big")
                }
            }
        }
    };
}

impl_dps_data_type!(u8, Unsigned);
impl_dps_data_type!(u16, Unsigned);
impl_dps_data_type!(u32, Unsigned);

impl_dps_data_type!(i8, Signed);
impl_dps_data_type!(i16, Signed);
impl_dps_data_type!(i32, Signed);

impl_dps_data_type!(f32, Floated);
