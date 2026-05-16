use std::marker::PhantomData;

use bevy::{DefaultPlugins, app::{App, FixedUpdate, Startup}, camera::Camera2d, ecs::system::Commands, input::mouse::MouseButton, time::Time, transform::components::Transform};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};
use frunk::{HList, hlist};
use nalgebra::{SVector, zero};
use num_traits::{One, Zero};
use physics_basic::stats::{Energy, Mass, Momentum, Volume};
use stardust_simworld::{grid::{GridPlugins, grid::GridData}, grid_gas::{GridGasPlugins, edge_type::GridGasEdgeWall, resource::{GridGasDatas, GridGasResource, grid_gas_datas}}, simulate_speed::{SimulateSpeedPlugin, simulate_speed::SimulateSpeed}};
use wacky_bag::structures::n_dim_array::{n_dim_chunk::get_chunk_dim_elem_count, n_dim_chunk_array::NDimChunkArray};
use wacky_bag_bevy::unit_ui::wld_cameras::WorldCameras;

use stardust_simworld_test::{num::Num, utils::{consts::DIM, draw, simba_fixed_for_normal_cdf::NormalCdfConstsForSimbaFixedI32F32}};

#[test]
fn test1(){
	
	let one=Num::one();
	let f_1_2=Num::from_num(0.5);
	let default_grid_gas_data:GridGasDatas<Num,DIM>=grid_gas_datas(hlist!(Mass(one),Momentum([f_1_2*f_1_2*f_1_2*f_1_2,zero()].into()),Energy(zero()),Volume(one)));

	println!("a");
}

fn main(){
	let mut app=App::new();
	let lens=[4;DIM];
	let one=Num::one();
	let f_1_2=0.5;

	let default_grid_gas_data:GridGasDatas<Num,DIM>=grid_gas_datas(hlist!(Mass(one),Momentum([f_1_2*f_1_2*f_1_2*f_1_2,zero()].into()),Energy(f_1_2*f_1_2*f_1_2*f_1_2),Volume(one)));
	
	// let default_grid_gas_data:GridGasDatas<Num,DIM>=grid_gas_datas(hlist!(Mass(one),Momentum([zero();DIM].into()),Energy(zero()),Volume(one)));
	
	app
		.add_plugins(DefaultPlugins)
        .add_plugins(PanCamPlugin::default())

        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
		.insert_resource(Time::from_hz(32.))
		.add_plugins(SimulateSpeedPlugin{
				global_speed:SimulateSpeed::from_fps(16.0)
			})
		.add_plugins(GridPlugins{
			grid_data:GridData{
				ranges:lens.clone(),
				grid_edge_len:one,
				grid_volume:one,
			}
		})
		.add_plugins(GridGasPlugins{
			p: PhantomData::default(),
			resource: GridGasResource(
				NDimChunkArray::from_fn(lens.clone(), get_chunk_dim_elem_count::<GridGasDatas<Num,DIM>,DIM>(16*1024).0, |_a|{
					default_grid_gas_data.clone()
				})
			),

		})
		;
		
	// println!("??{}",app.is_plugin_added::<GridGasPlugins::<Num,DIM,NormalCdfConstsForSimbaFixedI32F32>>());
	// println!("??{:?}",app.get_added_plugins());
	app
		.add_plugins(GridGasEdgeWall::<Num,DIM,_>::default())
		// .add_plugins(GridGasPluginsWithEdge{
		// 	p: PhantomData::default(),
		// 	resource: GridGasResource(
		// 		NDimChunkArray::from_fn(lens.clone(), get_chunk_dim_elem_count::<GridGasDatas<Num,DIM>,DIM>(16*1024).0, |_a|{
		// 			default_grid_gas_data.clone()
		// 		})
		// 	),
		// 	edge_type: GridGasEdgeWall::<Num,DIM,_>::default(),
		// })
		.add_systems(FixedUpdate, draw::draw_gas_grid)
		.add_systems(Startup,setup)
		.run()
	;
	// let dwa:GridGasPlugins::<Num,2,NormalCdfConstsForSimbaFixedI32F32, GridGasEdgeWall<Num,DIM,NormalCdfConstsForSimbaFixedI32F32>>=
	// GridGasPlugins{
	// 	p: PhantomData::default(),
	// 	resource: GridGasResource(
	// 		NDimChunkArray::from_fn(lens.clone(), get_chunk_dim_elem_count::<GridGasDatas<Num,DIM>,DIM>(16*1024).0, |a|{
	// 			default_grid_gas_data.clone()
	// 		})
	// 	),
	// 	edge_type: GridGasEdgeWall::<Num,DIM,NormalCdfConstsForSimbaFixedI32F32>::default(),
	// };
	// app.add_plugins(
	// 	dwa
	// );

}


pub fn setup(mut commands:Commands){
	let main_camera=commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0,0.0),
        PanCam{
            grab_buttons: vec![MouseButton::Middle],
            ..Default::default()
        },
    )).id();

    commands.insert_resource(WorldCameras(vec![main_camera]));
}