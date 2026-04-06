use crate::num::Num;
use num_traits::{One, cast::FromPrimitive};

pub const DIM:usize=2;
pub const GRID_LEN:(usize,usize)=(16,16);
pub const WLD_LEN_TO_SCREEN:f32=64.0;
pub const GRID_EDGE_LEN:Num=simba::scalar::FixedI64::<fixed::types::extra::U32>(fixed::types::I32F32::ONE);
pub const GRID_VOLUME:Num=simba::scalar::FixedI64::<fixed::types::extra::U32>(fixed::types::I32F32::ONE);