use std::{marker::PhantomData, ops::Deref};

use bevy::{DefaultPlugins, app::{App, FixedUpdate, Startup}, asset::AssetServer, camera::Camera2d, ecs::{children, component::Component, name::Name, related, schedule::IntoScheduleConfigs, system::{Commands, Query, Res}}, input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton}, math::Vec3, sprite::Sprite, time::Time, transform::components::Transform};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};
use frunk::{Poly, hlist, hlist_pat};
use nalgebra::{RealField, SVector, zero};
use num_traits::One;
use physics_basic::{body::ShapeSphere, rotation::{AngularMomentum, AngularVel, Rotation, RotationMatrix, angular_momentum_from_omega, so_vec_to_mat, sphere_inertia}, stats::*};
use stardust_simworld::{grid::{GridPlugins, at_grid::AtGridCell, grid::GridData}, grid_gas::{GridGasPlugins, edge_type::GridGasEdgeWall, resource::{GridGasDatas, GridGasResource, grid_gas_datas}}, multi_body::{self, attach::{AttachName, AttachTo, AttachToWith, AttachedEntities, MapAttachToWith}, propagate_change, propagate_position}, physics, schedule::schedule_sim, simulate_speed::{SimulateSpeedPlugin, simulate_speed::SimulateSpeed}, subsystem::{self, pipe::{dyn_key::DynKey, tree_router::{TreeRouter, TreeRouterPropagateLeafToRootSystemSet, TreeRouterPropagateRootToLeafSystemSet, TreeRouterVecComp}}, thrust::{self, ThrustDef}}, transform::{TransformPlugins, tramsform::WldLengthToScreenLength}};
use statistic_physics::{formulas::mass_momentum_2_kenetic, stats::Internal};
use wacky_bag::structures::n_dim_array::{n_dim_chunk::get_chunk_dim_elem_count, n_dim_chunk_array::NDimChunkArray};
use wacky_bag_bevy::{component::CAM, stat_component::{cache_set::{self, CacheSet}, stat::Stat}, unit_ui::wld_cameras::WorldCameras, utils::{plugin_add_systems::plugin_add_systems, relationship_root::RelationshipRoot, stat_for_hlist::{MapToChange, MapToStat}}};

use stardust_simworld_test::{num::Num, utils::{consts::{DIM, WLD_LEN_TO_SCREEN}, draw}};
use wacky_bag_hlist::variadics_tuple::ToVariadicsTuple;

fn num<Num:RealField>(v:f32)->Num{Num::from_f32(v).unwrap()}
fn cache_set_plugin(app:&mut App){
	app.add_systems(schedule_sim(), cache_set::set_cache_set_system_cfg::<Stat<RotationMatrix<Num,DIM>>,()>());
}
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
		.add_plugins(cache_set_plugin)
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
		.add_plugins(multi_body::plugin::<Num,DIM>)
		.add_plugins(subsystem::pipe::tree_router::plugin)
		.add_plugins(thrust::plugin::<Num,DIM>)
		.add_plugins(thrust::control_pipe::plugin::<Num>)
		;
	app
		
		.add_systems(FixedUpdate, draw::draw_gas_grid)
		.add_systems(Startup,setup)
		.add_systems(schedule_sim(), (
			ship_control.after(TreeRouterPropagateLeafToRootSystemSet).before(TreeRouterPropagateRootToLeafSystemSet)
		,))
	;

	println!("{:?}",app);
	app.run();

}
#[derive(Debug,Component,Default)]
pub struct ShipController{
	log_timer:usize,
}

pub fn ship_control(mut q:Query<(&mut ShipController, &mut TreeRouterVecComp)>,keyboard_input: Res<ButtonInput<KeyCode>>){
	let input_kw=keyboard_input.pressed(KeyCode::KeyW);
	let input_ka=keyboard_input.pressed(KeyCode::KeyA);
	let input_ks=keyboard_input.pressed(KeyCode::KeyS);
	let input_kd=keyboard_input.pressed(KeyCode::KeyD);
	let input_x:f64=(input_kw as i32-input_ks as i32) as f64;
	let input_y:f64=(input_ka as i32-input_kd as i32) as f64;
	q.par_iter_mut().for_each(
		|(mut controller,interface)|{
		let mut interface=interface.lock().unwrap();
		controller.log_timer+=1;
		if controller.log_timer>=32 {
			controller.log_timer=0;
			println!("{:?}",interface.deref());
		}
		let lt_idx=interface.key( DynKey::Str("thrust_left".into()).append(DynKey::Str(thrust::control_pipe::THRUST_PIPE_KEY.into())) );
		let rt_idx=interface.key( DynKey::Str("thrust_right".into()).append(DynKey::Str(thrust::control_pipe::THRUST_PIPE_KEY.into())) );
		let Some(lt)=interface.get_mut(lt_idx).and_then(|d|d.downcast_mut::<thrust::control_pipe::ThrustPipe<Num>>()) else {
			bevy::log::error!("missing thrust_left");
			return;
		};
		lt.2.power=lt.0.max_power*(input_x-input_y);
		
		let Some(rt)=interface.get_mut(rt_idx).and_then(|d|d.downcast_mut::<thrust::control_pipe::ThrustPipe<Num>>()) else {
			bevy::log::error!("missing thrust_right");
			return;
		};

		rt.2.power=rt.0.max_power*(input_x+input_y);
		
		// {
		// 	let km=interface.keys().key_map();

		// 	// println!("{:?}",km);
		// }
	});
}
#[test]
pub fn test_awd(){
	let ag_inertia=sphere_inertia::<_,DIM>(num(1.0),num(0.5));
	let hlist_pat![agm]=angular_momentum_from_omega(hlist![&ag_inertia,&AngularVel([0.25].into())]);
	println!("{ag_inertia:.5?}");
	println!("{agm:.5?}");

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
	let img_thrust=asset_server.load("textures/test/enemy_A.png");

	let ag_inertia=sphere_inertia::<_,DIM>(num(1.0),num(0.5));
	let hlist_pat![agm]=angular_momentum_from_omega(hlist![&ag_inertia,&AngularVel([0.25].into())]);
	println!("{ag_inertia:.5?}");
	println!("{agm:.5?}");

	let _a_ship=commands.spawn((
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
		Stat(RotationMatrix::<Num,DIM>::default()),
		CacheSet::<Stat<RotationMatrix<Num,DIM>>>::default(),
		Transform::default(),
		Sprite::from_image(img.clone()),
		AtGridCell([0,0]),
		Name::new("A ship"),
		subsystem::pipe::tree_router::bundle(),
		ShipController::default(),
		related!(
			AttachedEntities[
				(
					Name::new("A ship Thrust Left"),
					thrust::bundle(ThrustDef::new(0.5, 0.25)),

					hlist![
						Pos(SVector::<Num,DIM>::from([num(2.0),num(0.0)])),
						// Pos::<Num,DIM>::default(),
						Vel::<Num,DIM>::default(),
						Rotation::<Num,DIM>::default(),
						AngularVel::<Num,DIM>::default(),
						RotationMatrix::<Num,DIM>::default(),
						TimePass::<Num>(num(1.0/16.0))
					]
					.map(Poly(MapToStat)).to_variadics_tuple(),
					
					CacheSet::<Stat<RotationMatrix<Num,DIM>>>::default(),

					hlist![
						Name::new("thrust_left"),
						Pos(SVector::<Num,DIM>::from([num(-0.25),num(0.25)]))
					]
					.map(Poly(MapAttachToWith)).to_variadics_tuple(),
					
					hlist![
						Momentum::<Num,DIM>::default(),AngularMomentum::<Num,DIM>::default()
					]
					.map(Poly(MapToChange)).to_variadics_tuple(),
					

					RelationshipRoot::<AttachTo>::default(),
					subsystem::pipe::tree_router::bundle(),
					
					Transform::from_scale(Vec3::ONE*0.5),
					Sprite::from_image(img_thrust.clone()),
					AtGridCell([0,0]),
				),
				(
					Name::new("A ship Thrust Right"),
					thrust::bundle(ThrustDef::new(0.5, 0.25)),

					hlist![
						Pos(SVector::<Num,DIM>::from([num(3.0),num(0.0)])),
						Vel::<Num,DIM>::default(),
						Rotation::<Num,DIM>::default(),
						AngularVel::<Num,DIM>::default(),
						RotationMatrix::<Num,DIM>::default(),
						TimePass::<Num>(num(1.0/16.0))
					]
					.map(Poly(MapToStat)).to_variadics_tuple(),

					hlist![
						Name::new("thrust_right"),
						Pos(SVector::<Num,DIM>::from([num(-0.25),num(-0.25)]))
					]
					.map(Poly(MapAttachToWith)).to_variadics_tuple(),
					
					CacheSet::<Stat<RotationMatrix<Num,DIM>>>::default(),
					
					hlist![
						Momentum::<Num,DIM>::default(),AngularMomentum::<Num,DIM>::default()
					]
					.map(Poly(MapToChange)).to_variadics_tuple(),
					
					
					

					RelationshipRoot::<AttachTo>::default(),
					subsystem::pipe::tree_router::bundle(),
					
					Transform::from_scale(Vec3::ONE*0.5),
					Sprite::from_image(img_thrust.clone()),
					AtGridCell([0,0]),
				)
			]
		),
	));
	// let left_thrust=a_ship.
}