use std::io;

struct Student {
    name: String,
    score: f32,
}

impl Student {
    fn has_passed(&self) -> bool {
        self.score >= 50.0
    }

    fn display_result(&self) {
        if self.has_passed() {
            println!("{} has passed with a score of {:.1}!", self.name, self.score);
        } else {
            println!("{} has failed with a score of {:.1}.", self.name, self.score);
        }
    }
}

fn main() {
    let mut name = String::new();
    let mut score_input = String::new();

    println!("Enter student's name:");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("Enter student's score:");
    io::stdin().read_line(&mut score_input).expect("Failed to read score");

    let score: f32 = match score_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid score input!");
            return;
        }
    };

    let student = Student {
        name: name.trim().to_string(),
        score,
    };

    student.display_result();
}
