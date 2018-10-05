use std::io;
use std::io::Write;

struct Subject {
	class_section: String,
	course_code: String
}

struct Student {
	name: String,
	s_num: String,
	subjects: Vec<Subject>
}

fn print_menu() {
	println!("\n======MENU======");
	println!("[1] Add student");
	println!("[2] View all students");
	println!("[3] Add subject to student");
	println!("[4] Delete student");
	println!("[5] Exit");
	print!("\nChoice: ");

}

fn get_choice() -> String {
	let mut temp = String::new();
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut temp).expect("Error");
	temp.pop();
	temp.to_string();
	temp
}

fn main() {
	print_menu();
	let mut choice = get_choice().parse::<u32>().unwrap();
	println!("{}\n", choice);
}