use dotmgr::F;

use std::{env, fs, path::Path};

fn setup() {
	fs::create_dir_all(Path::new("testdir/source")).unwrap();
	fs::create_dir_all(Path::new("testdir/target")).unwrap();
	env::set_current_dir("testdir").unwrap();
}

fn teardown() {
	env::set_current_dir("..").unwrap();
	fs::remove_dir_all(Path::new("testdir")).unwrap();
}

#[test]
fn source_file_target_empty() {
	setup();

	fs::write("source/f", "woof").unwrap();
	assert_eq!(1, 1);

	teardown();
}
