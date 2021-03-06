extern crate users;
use users::{Users, OSUsers};

fn main() {
	let mut cache = OSUsers::empty_cache();
	
	let current_uid = cache.get_current_uid();
	println!("Your UID is {}", current_uid);
	
	let you = cache.get_user_by_uid(current_uid).expect("No entry for current user!");
	println!("Your username is {}", you.name);
	
	let primary_group = cache.get_group_by_gid(you.primary_group).expect("No entry for your primary group!");
	println!("Your primary group has ID {} and name {}", primary_group.gid, primary_group.name);

	if primary_group.members.is_empty() {
		println!("There are no other members of that group.");
	}
	else {
		for username in primary_group.members.iter() {
			println!("User {} is also a member of that group.", username);
		}
	}
}
