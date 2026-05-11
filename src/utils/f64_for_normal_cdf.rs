use std::str::FromStr;

use simba::scalar::{FixedI32F32, FixedI64};
use wacky_bag::math::normal_cdf::{NORMAL_CDF_CONSTS_DATA_F64, NORMAL_CDF_CONSTS_DATA_STR, NormalCdfConsts, NormalCdfConstsData};

pub struct NormalCdfConstsForF64;

impl NormalCdfConsts<NormalCdfConstsForF64> for f64 {
	fn datas() -> NormalCdfConstsData<Self> {
		// NORMAL_CDF_CONSTS_DATA_STR.map(|v|f64::from_str(v).unwrap())
		NORMAL_CDF_CONSTS_DATA_F64
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