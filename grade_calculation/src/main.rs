// Simple Grade Calculation 

use std::collections::HashMap;
struct Student {
    name: String,
    grades: Vec<u32>,
}

impl Student {
    fn add_students(name: &str, grades: Vec<u32>) -> Self {
        Student {
            name: name.to_string(),
            grades
        }
    }

    fn calculate_average(&self) -> f64 {
        // with .iter() we iterate through grades and with .sum() sum up
        let sum: u32 = self.grades.iter().sum();  
        // turn usize into u32 
        let count = self.grades.len() as u32; 

        if count > 0 {
            f64::from(sum) / f64::from(count) // turns u32 values to f64
        } else {
            0.0
        }         
    }
}

fn main() {
    // creating a hashmap that store students' name and grades
    let mut students: HashMap<String, Student> = HashMap::new(); 

    // creating students
    let student1 = Student::add_students("Abra", vec![23, 45, 67]);
    let student2 = Student::add_students("Yutpa", vec![65, 45, 80]);

    // inserting students into HashMap
    students.insert(format!("{}", student1.name), student1); // with Format! we set the student name as a key
    students.insert(format!("{}", student2.name), student2);

    // Calculating average 
    for (key, student) in  &students{ // value = student
        let average = student.calculate_average();
        println!("Name: {} \nGrades: {:?} \tAverage: {:.2}", student.name, student.grades, average);
    } 

}
