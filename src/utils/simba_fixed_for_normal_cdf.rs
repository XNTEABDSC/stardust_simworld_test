use std::str::FromStr;

use simba::scalar::{FixedI32F32, FixedI64};
use wacky_bag::math::normal_cdf::{NormalCdfConsts, NormalCdfConstsData};


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