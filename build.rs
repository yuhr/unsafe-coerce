use cargo_metadata::MetadataCommand;
use rustc_version::{version_meta, Channel};

fn main() {
	match version_meta().unwrap().channel {
		Channel::Nightly => {
			let metadata = MetadataCommand::new().exec().unwrap();
			let package = metadata.root_package().unwrap();
			for subfeature in package.features["nightly"].iter() {
				println!("cargo:rustc-cfg=feature=\"{subfeature}\"");
			}
		}
		_ => {}
	}
}