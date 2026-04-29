use bevy::app::App;
use frunk::{HList, hlist};
use nalgebra::{SVector, zero};
use num_traits::One;
use physics_basic::stats::{Energy, Mass, Momentum};
use stardust_simworld::{grid::{GridPlugins, grid::GridData}, grid_gas::{GridGasPlugins, edge_type::GridGasEdgeWall, resource::{GridGasDatas, GridGasResource, grid_gas_datas}}};
use statistic_physics::stats::Volume;
use wacky_bag::structures::n_dim_array::{n_dim_chunk::get_chunk_dim_elem_count, n_dim_chunk_array::NDimChunkArray};

use crate::{num::Num, utils::consts::DIM};



pub fn example1(){
	let mut app=App::new();
	let lens=[16;DIM];
	let zero=Num::default();
	let one=Num::one();
	let f_1_2=Num::from_num(0.5);
	let default_grid_gas_data:GridGasDatas<Num,DIM>=grid_gas_datas(hlist!(Mass(one),Momentum([zero();DIM].into()),Energy(f_1_2),Volume(one)));
	app.add_plugins((
		GridPlugins{
			grid_data:GridData{
				ranges:lens.clone(),
				grid_edge_len:one,
				grid_volume:one,
			}
		},
	));
	let dwa:GridGasPlugins::<_,_,_,_>=
	GridGasPlugins{
				p: std::marker::PhantomData,
				resource: GridGasResource(
					NDimChunkArray::from_fn(lens.clone(), get_chunk_dim_elem_count::<GridGasDatas<Num,DIM>,DIM>().0, |a|{

					})
				),
				edge_type: GridGasEdgeWall::default(),
			};
	app.add_plugins(
			dwa
		)
		;
}