// use bevy::{ecs::{schedule::Schedule, world::{CommandQueue, World}}, tasks::{ComputeTaskPool, TaskPool}};
use bevy::{ecs::world::CommandQueue, prelude::*, tasks::{ComputeTaskPool, TaskPool}};

#[derive(Debug,Component,Reflect)]
struct NameComponent(pub String);

fn main(){
	ComputeTaskPool::get_or_init(TaskPool::default);
	let mut world = World::default();
	// let mut schedule = Schedule::default();
	let mut command_queue = CommandQueue::default();
    let mut commands = Commands::new(&mut command_queue, &world);
	
	let root = commands.spawn(NameComponent("root".into())).id();
	let parent = commands.spawn(NameComponent("parent".into())).id();
	let child = commands.spawn(NameComponent("child".into())).id();
	
	command_queue.apply(&mut world);
	
	let mut command_queue2 = CommandQueue::default();
    let mut commands2 = Commands::new(&mut command_queue2, &world);
	commands2.entity(parent).insert(ChildOf(root));
	commands2.entity(child).insert(ChildOf(parent));
	println!("{:?}",world.get::<Children>(parent));

	command_queue2.apply(&mut world);
	println!("{:?}",world.get::<Children>(parent));
}