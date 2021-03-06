use bevy::prelude::*;
use bevy::window::*;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: "Dot32 intro".to_string(),
			width: 800.,
			height: 600.,
			present_mode: PresentMode::Fifo, // Vesync enabled, replace Fifo with Mailbox for no vsync
			..Default::default()
		})
		.insert_resource(ClearColor(Color::rgb(0.7, 0.55, 0.41)))
		.add_plugins(DefaultPlugins)
		.add_plugin(dot32_intro::Intro)
		.run();
}