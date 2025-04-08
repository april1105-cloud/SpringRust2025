// In class Assignment

// Define the Student struct
struct Student {
    major: String,
}

// First-order function: assigns a major to a student
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

// Higher-order function: updates all students in a collection using the provided behavior
fn update_majors(
    mut collection: Vec<Student>,
    majors: Vec<String>,
    behavior: fn(&mut Student, String),
) -> Vec<Student> {
    for (student, major) in collection.iter_mut().zip(majors.into_iter()) {
        behavior(student, major);
    }
    collection
}

fn main() {
    let students = vec![
        Student { major: "".to_string() },
        Student { major: "".to_string() },
        Student { major: "".to_string() },
    ];

    let majors = vec![
        "Computer Science".to_string(),
        "Biology".to_string(),
        "Art History".to_string(),
    ];

    let updated_students = update_majors(students, majors, assign_major);

    for (i, student) in updated_students.iter().enumerate() {
        println!("Student {} major: {}", i + 1, student.major);
    }
}
