use std::marker::PhantomData;

use bevy::{DefaultPlugins, app::{App, FixedUpdate, Startup}, asset::AssetServer, camera::Camera2d, ecs::system::{Commands, Res}, input::mouse::MouseButton, sprite::Sprite, time::Time, transform::components::Transform};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};
use frunk::{hlist, hlist_pat};
use nalgebra::{RealField, SVector, zero};
use num_traits::One;
use physics_basic::{body::ShapeSphere, rotation::{AngularVel, Rotation, angular_momentum_from_omega, sphere_inertia, so_vec_to_mat}, stats::*};
use stardust_simworld::{grid::{GridPlugins, at_grid::AtGridCell, grid::GridData}, grid_gas::{GridGasPlugins, edge_type::GridGasEdgeWall, resource::{GridGasDatas, GridGasResource, grid_gas_datas}}, physics, simulate_speed::{SimulateSpeedPlugin, simulate_speed::SimulateSpeed}, transform::{TransformPlugins, tramsform::WldLengthToScreenLength}};
use statistic_physics::{formulas::mass_momentum_2_kenetic, stats::Internal};
use wacky_bag::structures::n_dim_array::{n_dim_chunk::get_chunk_dim_elem_count, n_dim_chunk_array::NDimChunkArray};
use wacky_bag_bevy::unit_ui::wld_cameras::WorldCameras;

use stardust_simworld_test::{num::Num, utils::{consts::{DIM, WLD_LEN_TO_SCREEN}, draw}};

fn num<Num:RealField>(v:f32)->Num{Num::from_f32(v).unwrap()}

fn main(){
	let fps: f64=16.0;
	let fixed_update=16.0;

	let mut app=App::new();
	let lens=[16,16];
	let one=Num::one();
	let f_1_2=0.5;

	
	let g_momentum=Momentum([zero(),zero()].into());
	let g_mass=Mass(one);
	let _default_grid_gas_data:GridGasDatas<Num,DIM>=
	grid_gas_datas(hlist!(g_mass,g_momentum,Energy(mass_momentum_2_kenetic(hlist![&g_mass, &g_momentum]).0),Volume(one)));

	let g_momentum_s=Momentum([f_1_2*f_1_2*f_1_2,f_1_2*f_1_2*f_1_2].into());
	let g_mass_s=Mass(f_1_2);
	let default_grid_gas_data_s:GridGasDatas<Num,DIM>=
	grid_gas_datas(hlist!(g_mass_s,g_momentum_s,Energy(mass_momentum_2_kenetic(hlist![&g_mass_s, &g_momentum_s]).0+f_1_2*f_1_2*f_1_2*f_1_2),Volume(one)));
	
	app
		.add_plugins(DefaultPlugins)
        .add_plugins(PanCamPlugin::default())

        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
		.insert_resource(Time::from_hz(fixed_update))
		.add_plugins(SimulateSpeedPlugin{
			global_speed:SimulateSpeed::from_fps(fps)
		})
		.add_plugins(TransformPlugins::<Num,DIM>{
			wld_length_to_screen_length: WldLengthToScreenLength::from_wld_len_2_screen_len(WLD_LEN_TO_SCREEN),
			_p: Default::default(),
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
					if _a==[7,2] {
						default_grid_gas_data_s.clone()
					}else {
						default_grid_gas_data_s.clone()
					}
				})
			),

		})
		.add_plugins(physics::Plugins::<Num,DIM>::default())
		.add_plugins(GridGasEdgeWall::<Num,DIM,_>::default())
		
		;
	// println!("??{}",app.is_plugin_added::<GridGasPlugins::<Num,DIM,NormalCdfConstsForSimbaFixedI32F32>>());
	// println!("??{:?}",app.get_added_plugins());
	app
		
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
		// .run()
	;

	println!("{:?}",app);
	app.run();
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


pub fn setup(mut commands:Commands, asset_server: Res<AssetServer>){

	let main_camera=commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0,0.0),
        PanCam{
            grab_buttons: vec![MouseButton::Middle],
            ..Default::default()
        },
    )).id();
	
	// let a:TypeRegistry
    commands.insert_resource(WorldCameras(vec![main_camera]));

	let img=asset_server.load("textures/test/ship_C.png");

	let ag_inertia=sphere_inertia::<_,DIM>(num(1.0),num(0.5));
	let hlist_pat![agm]=angular_momentum_from_omega(hlist![&ag_inertia,&AngularVel(so_vec_to_mat(&[0.25].into()))]);
	println!("{ag_inertia:.5?}");
	commands.spawn((
		physics::bundle::phy_body_statistic_bundle(hlist![
			TimePass(num(1.0/16.0)),
			ShapeSphere::from_radius(num(0.5)),
			Mass(num(1.0)),
			Pos(SVector::<Num,DIM>::from([num(2.0),num(2.0)])),
			Momentum(SVector::<Num,DIM>::from([num(0.0),num(0.0)])),
			ag_inertia,
			agm,
			Rotation::default(),
			Internal(num(0.1))
		]),
		Transform::default(),
		Sprite::from_image(img),
		AtGridCell([0,0]),
	));


}