struct Subject {
	class_section: String,
	course_code: String
}

struct Student {
	name: String,
	s_num: String,
	subjects: Vec<Subject>
}

fn main() {
	println!("Initial commit!");
}