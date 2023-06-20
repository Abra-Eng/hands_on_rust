// this code assumes a single student for sipmlicity
fn main() {

    let mut student = Student::enroll_student(String::from("Abra"), 21, 1);
    // using the double colons with (Struct_name::fn_name) allows us to call the associated function
    // directly on the Student struct,  without needing an instance of the struct beforehand.
    //  This is useful when creating a new instance of the struct.

    student.display_info();

    student.update_info("Elisa".to_string(), 32, 2);

    student.display_info();
}

struct Student {
    name: String,
    age: u32,
    roll_number: u32
}

impl Student {
    // enrollment
    fn enroll_student(name: String, age: u32, roll_number: u32) -> Self { // we need to return itself 
        Student {
            name,
            age,
            roll_number,
        }
        
    }
    // updating info
    fn update_info(&mut self, name: String, age: u32, roll_number: u32) {
        self.name = name;
        self.age = age;
        self.roll_number = roll_number;

    }
    // displaying info
    fn display_info(&self) { // we just needs to print values
        
        println!("Name: {}, Age: {}, roll number: {}", self.name, self.age, self.roll_number);
    }
}
