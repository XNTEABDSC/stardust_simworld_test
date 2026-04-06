use std::str::FromStr;

use simba::scalar::{FixedI32F32, FixedI64};
use wacky_bag::math::normal_cdf::{NORMAL_CDF_CONSTS_DATA_STR, NormalCdfConsts, NormalCdfConstsData};

pub struct NormalCdfConstsForSimbaFixedI32F32;

impl NormalCdfConsts<NormalCdfConstsForSimbaFixedI32F32> for FixedI32F32 {
	fn datas() -> NormalCdfConstsData<Self> {
		NORMAL_CDF_CONSTS_DATA_STR.map(|v|FixedI64(FromStr::from_str(v).unwrap()))
	}
}


#[cfg(test)]
mod test{
    use simba::scalar::FixedI32F32;
    use wacky_bag::math::normal_cdf::normal_cdf;

	#[test]
	fn test1(){
		let a=normal_cdf(FixedI32F32::from_num(0.5));
		println!("{}",a);
	}
}