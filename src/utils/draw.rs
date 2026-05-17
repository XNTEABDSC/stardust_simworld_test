
use bevy::{color::palettes::css::{GREEN, WHITE}, prelude::*};
use frunk::{HList, Poly, hlist_pat};
use physics_basic::stats::{Mass, Vel};
use simba::scalar::SupersetOf;
use stardust_simworld::{grid_gas::resource::GridGasResource, transform::tramsform::WldLengthToScreenLength};
use statistic_physics::stats::{VelVar1Dir};
use wacky_bag::{structures::n_dim_array::t_n_dim_array::TNDimArrayForEach, utils::output_func::HMappableFrom};
use wacky_bag_bevy::utils::stat_for_hlist::MapFromStatRef;

use crate::{num::Num, utils::consts::{DIM}};



pub fn draw_gas_grid(res_gas_grid:Res<GridGasResource<Num,{DIM}>>,mut gizmos:Gizmos, len_trans:Res<WldLengthToScreenLength>){
	res_gas_grid.0.for_each(&mut |cell,idx|{
		let (x,y)=(idx[0],idx[1]);
		let wld_len_to_screen=len_trans.wld_len_2_screen_len;
		let vecgridmid=Vec2{
			x: (x as f32 * wld_len_to_screen)+wld_len_to_screen/2.0,
			y: (y as f32 * wld_len_to_screen)+wld_len_to_screen/2.0,
		};
		let hlist_pat![mass,v_mean,v_var]:HList!(&Mass<Num>,&Vel<Num,DIM>,&VelVar1Dir<Num>)=HMappableFrom::output_map(cell.to_ref().sculpt().0, Poly(MapFromStatRef));
		gizmos.rect_2d(
			Isometry2d::from_translation(vecgridmid), 
			Vec2 { x: wld_len_to_screen, y: wld_len_to_screen }, 
			WHITE);
		let num_to_screen = |v:Num|->f32{
			let f:f32=v.to_subset().unwrap();
			f*wld_len_to_screen
		};
		let vec_grid_mid_offset=Vec2{
			x:num_to_screen(v_mean.0[0]),
			y:num_to_screen(v_mean.0[1]),
		};
		gizmos.arrow_2d(vecgridmid, vecgridmid+vec_grid_mid_offset*4.0, WHITE);

		let v_var_len=num_to_screen(v_var.0);
		gizmos.circle_2d(Isometry2d::from_translation(vecgridmid), v_var_len, WHITE);

		gizmos.cross_2d(Isometry2d::from_translation(vecgridmid), num_to_screen(mass.0)/4.0, GREEN);
		// gizmos.
	});
}
