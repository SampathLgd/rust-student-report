use printpdf::*;
use std::fs::File;
use std::io::{self, BufWriter};

#[derive(Clone)]
struct Student {
    name: String,
    total_marks: f64,
    max_marks: f64,
    subjects: u32,
}

impl Student {
    fn average(&self) -> f64 {
        self.total_marks / self.subjects as f64
    }

    fn percentage(&self) -> f64 {
        (self.total_marks / self.max_marks) * 100.0
    }

    fn grade(&self) -> &'static str {
        let avg = self.average();
        if avg >= 90.0 {
            "A"
        } else if avg >= 75.0 {
            "B"
        } else if avg >= 60.0 {
            "C"
        } else {
            "D"
        }
    }
}

fn input(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn generate_pdf_report(students: &[Student]) {
    let (doc, page1, layer1) = PdfDocument::new("Student Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);

    let title_font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let heading_font = doc.add_builtin_font(BuiltinFont::HelveticaOblique).unwrap();
    let regular_font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();

    let mut y = Mm(270.0);
    layer.use_text("STUDENT REPORT CARD", 26.0, Mm(50.0), y, &title_font);
    y -= Mm(20.0);

    for student in students {
        let box_height = Mm(50.0);
        let box_top = y;
        let box_bottom = y - box_height;
        let box_left = Mm(20.0);
        let box_right = Mm(190.0);

        let border_points = vec![
            (Point::new(box_left, box_top), false),
            (Point::new(box_right, box_top), false),
            (Point::new(box_right, box_bottom), false),
            (Point::new(box_left, box_bottom), false),
            (Point::new(box_left, box_top), false),
        ];

        let border = Line {
            points: border_points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };
        layer.add_shape(border);

        let mut line_y = y - Mm(6.0);
        let label_x = Mm(25.0);
        let value_x = Mm(80.0);

        layer.use_text("Name:", 13.0, label_x, line_y, &heading_font);
        layer.use_text(&student.name, 13.0, value_x, line_y, &regular_font);
        line_y -= Mm(7.0);

        layer.use_text("Total Marks:", 13.0, label_x, line_y, &heading_font);
        layer.use_text(&format!("{}", student.total_marks), 13.0, value_x, line_y, &regular_font);
        line_y -= Mm(7.0);

        layer.use_text("Maximum Marks:", 13.0, label_x, line_y, &heading_font);
        layer.use_text(&format!("{}", student.max_marks), 13.0, value_x, line_y, &regular_font);
        line_y -= Mm(7.0);

        layer.use_text("Subjects:", 13.0, label_x, line_y, &heading_font);
        layer.use_text(&format!("{}", student.subjects), 13.0, value_x, line_y, &regular_font);
        line_y -= Mm(7.0);

        layer.use_text("Average:", 13.0, label_x, line_y, &heading_font);
        layer.use_text(&format!("{:.2}", student.average()), 13.0, value_x, line_y, &regular_font);
        line_y -= Mm(7.0);

        layer.use_text("Percentage:", 13.0, label_x, line_y, &heading_font);
        layer.use_text(&format!("{:.2}%", student.percentage()), 13.0, value_x, line_y, &regular_font);
        line_y -= Mm(7.0);

        layer.use_text("Grade:", 13.0, label_x, line_y, &heading_font);
        layer.use_text(student.grade(), 13.0, value_x, line_y, &regular_font);

        y -= box_height + Mm(12.0);
    }

    doc.save(&mut BufWriter::new(File::create("output/report_card.pdf").unwrap())).unwrap();
    println!("\n✅ Beautiful PDF report saved as 'output/report_card.pdf'\n");
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        println!("\n====== Student Report System ======");
        println!("1. Add Student");
        println!("2. View All Students");
        println!("3. Update Student");
        println!("4. Delete Student");
        println!("5. Generate PDF Report");
        println!("6. Exit");

        let choice = input("Enter your choice:");

        match choice.as_str() {
            "1" => {
                let name = input("Enter name:");
                let total: f64 = input("Enter total marks:").parse().unwrap_or(0.0);
                let max: f64 = input("Enter maximum marks:").parse().unwrap_or(100.0);
                let subjects: u32 = input("Enter number of subjects:").parse().unwrap_or(1);
                students.push(Student { name, total_marks: total, max_marks: max, subjects });
                println!("✅ Student added successfully!\n");
            }
            "2" => {
                println!("\n📋 Student Reports:\n");
                for (i, s) in students.iter().enumerate() {
                    println!(
                        "{}. {} | Total: {} | Max: {} | Subjects: {} | Avg: {:.2} | %: {:.2} | Grade: {}",
                        i + 1,
                        s.name,
                        s.total_marks,
                        s.max_marks,
                        s.subjects,
                        s.average(),
                        s.percentage(),
                        s.grade()
                    );
                }
                println!();
            }
            "3" => {
                let index: usize = input("Enter student index to update:").parse().unwrap_or(0);
                if let Some(s) = students.get_mut(index - 1) {
                    s.name = input("Enter new name:");
                    s.total_marks = input("Enter new total marks:").parse().unwrap_or(0.0);
                    s.max_marks = input("Enter new maximum marks:").parse().unwrap_or(100.0);
                    s.subjects = input("Enter new number of subjects:").parse().unwrap_or(1);
                    println!("✅ Student updated successfully!\n");
                } else {
                    println!("❌ Invalid index\n");
                }
            }
            "4" => {
                let index: usize = input("Enter student index to delete:").parse().unwrap_or(0);
                if index > 0 && index <= students.len() {
                    students.remove(index - 1);
                    println!("✅ Student deleted successfully!\n");
                } else {
                    println!("❌ Invalid index\n");
                }
            }
            "5" => generate_pdf_report(&students),
            "6" => break,
            _ => println!("Invalid option"),
        }
    }
}
