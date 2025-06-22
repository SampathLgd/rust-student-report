
# 📘 Rust Student Report Card System

This is a **Rust-based CLI application** that allows you to manage student report data with the ability to:

- ➕ Add, ✏️ Update, 🔍 View, and ❌ Delete students  
- 🧠 Automatically calculate **average, percentage, and grade**  
- 📝 Generate a **styled PDF report card** for all students


## 🛠 Features

- 📚 Stores:
  - Student name
  - Total marks
  - Maximum marks
  - Number of subjects
- 🧮 Calculates:
  - Average = Total Marks / Subjects
  - Percentage = (Total Marks / Max Marks) × 100
  - Grade:
    - A: ≥ 90%
    - B: 75–89%
    - C: 60–74%
    - D: < 60%
- 📄 Exports a beautiful **PDF report card** using [`printpdf`](https://crates.io/crates/printpdf)


## 🚀 Getting Started

### 🔧 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (use `rustup`)
- Git (optional, for cloning)

### 🧪 Run Locally

```bash
git clone https://github.com/SampathLgd/rust-student-report.git
cd rust-student-report
cargo run --release
````


## 📂 Output

* 📄 PDF is saved to: `output/report_card.pdf`

### 🧪 Console Menu

```text
====== Student Report System ======
1. Add Student
2. View All Students
3. Update Student
4. Delete Student
5. Generate PDF Report
6. Exit
```


## 🖨️ Sample PDF Preview

You can find the generated PDF in the `output/` folder → `report_card.pdf`


Made with ❤️ by [Sampath Lagudu]

