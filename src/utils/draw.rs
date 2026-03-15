
use bevy::{color::palettes::css::WHITE, prelude::*};
use frunk::{HList, Poly, hlist_pat};
use physics_basic::stats::Vel;
use stardust_simworld::grid_gas::resource::GridGasResource;
use statistic_physics::stats::{VelVar1Dir};
use wacky_bag::{structures::n_dim_array::t_n_dim_array::TNDimArrayForEach, utils::output_func::HMappableFrom};
use wacky_bag_bevy::utils::stat_for_hlist::MapFromStatRef;

use crate::utils::consts::{DIM, WLD_LEN_TO_SCREEN};



pub fn draw_gas_grid(res_gas_grid:Res<GridGasResource<{DIM}>>,mut gizmos:Gizmos){
	res_gas_grid.0.for_each(&mut |cell,idx|{
		let (x,y)=(idx[0],idx[1]);
		
		let vecgridmid=Vec2{
			x: (x as f32 * WLD_LEN_TO_SCREEN)+WLD_LEN_TO_SCREEN/2.0,
			y: (y as f32 * WLD_LEN_TO_SCREEN)+WLD_LEN_TO_SCREEN/2.0,
		};
		let hlist_pat![v_mean,v_var]:HList!(&Vel<DIM>,&VelVar1Dir)=HMappableFrom::output_map(cell.to_ref().sculpt().0, Poly(MapFromStatRef));
		gizmos.rect_2d(
			Isometry2d::from_translation(vecgridmid), 
			Vec2 { x: WLD_LEN_TO_SCREEN, y: WLD_LEN_TO_SCREEN }, 
			WHITE);
		let vec_grid_mid_offset=Vec2{
			x:v_mean.0[(0,0)].to_num::<f32>()*WLD_LEN_TO_SCREEN,
			y:v_mean.0[(0,1)].to_num::<f32>()*WLD_LEN_TO_SCREEN,
		};
		gizmos.arrow_2d(vecgridmid, vecgridmid+vec_grid_mid_offset*4.0, WHITE);

		let v_var_len=v_var.0.to_num::<f32>()*WLD_LEN_TO_SCREEN;
		gizmos.circle_2d(Isometry2d::from_translation(vecgridmid), v_var_len, WHITE);
	});
}

// pub fn draw_gas_grid_(res_gas_grid:Res<gas_grid::GasGrid>,mut gizmos: Gizmos,){
//     for (x,ggy) in &res_gas_grid.grid {
//         for (y,gc) in ggy{
//             let vecgridmid=Vec2{
//                 x: (x as f32 * WLD_LEN_TO_SCREEN)+WLD_LEN_TO_SCREEN/2.0,
//                 y: (y as f32 * WLD_LEN_TO_SCREEN)+WLD_LEN_TO_SCREEN/2.0,
//             };
            
//             gizmos.rect_2d(
//                 Isometry2d::from_translation(vecgridmid), 
//                 Vec2 { x: WLD_LEN_TO_SCREEN, y: WLD_LEN_TO_SCREEN }, 
//                 WHITE);
//             let vec_grid_mid_offset=Vec2{
//                 x:gc.matters.v_mean().0.to_num::<f32>()*WLD_LEN_TO_SCREEN,
//                 y:gc.matters.v_mean().1.to_num::<f32>()*WLD_LEN_TO_SCREEN,
//             };
//             gizmos.arrow_2d(vecgridmid, vecgridmid+vec_grid_mid_offset*4.0, WHITE);

//             let v_var_sqrt=gc.matters.v_var();
//             let v_var_sqrt_len=v_var_sqrt.to_num::<f32>()*WLD_LEN_TO_SCREEN;
//             gizmos.circle_2d(Isometry2d::from_translation(vecgridmid), v_var_sqrt_len, WHITE);
            
//         }
//     }
// }