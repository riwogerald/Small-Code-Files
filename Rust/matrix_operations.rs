use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    fn from_vec(data: Vec<Vec<f64>>) -> Result<Self, String> {
        if data.is_empty() {
            return Err("Matrix cannot be empty".to_string());
        }

        let rows = data.len();
        let cols = data[0].len();

        // Check that all rows have the same length
        for row in &data {
            if row.len() != cols {
                return Err("All rows must have the same length".to_string());
            }
        }

        Ok(Matrix { data, rows, cols })
    }

    fn identity(size: usize) -> Self {
        let mut matrix = Matrix::new(size, size);
        for i in 0..size {
            matrix.data[i][i] = 1.0;
        }
        matrix
    }

    fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row < self.rows && col < self.cols {
            Some(self.data[row][col])
        } else {
            None
        }
    }

    fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), String> {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }

    fn determinant(&self) -> Result<f64, String> {
        if self.rows != self.cols {
            return Err("Determinant only defined for square matrices".to_string());
        }

        match self.rows {
            1 => Ok(self.data[0][0]),
            2 => Ok(self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]),
            _ => {
                let mut det = 0.0;
                for j in 0..self.cols {
                    let minor = self.minor(0, j)?;
                    let cofactor = if j % 2 == 0 { 1.0 } else { -1.0 };
                    det += cofactor * self.data[0][j] * minor.determinant()?;
                }
                Ok(det)
            }
        }
    }

    fn minor(&self, exclude_row: usize, exclude_col: usize) -> Result<Matrix, String> {
        if self.rows <= 1 || self.cols <= 1 {
            return Err("Cannot create minor of 1x1 matrix".to_string());
        }

        let mut minor_data = Vec::new();
        for i in 0..self.rows {
            if i == exclude_row {
                continue;
            }
            let mut row = Vec::new();
            for j in 0..self.cols {
                if j == exclude_col {
                    continue;
                }
                row.push(self.data[i][j]);
            }
            minor_data.push(row);
        }

        Matrix::from_vec(minor_data)
    }

    fn trace(&self) -> Result<f64, String> {
        if self.rows != self.cols {
            return Err("Trace only defined for square matrices".to_string());
        }

        let mut trace = 0.0;
        for i in 0..self.rows {
            trace += self.data[i][i];
        }
        Ok(trace)
    }

    fn is_symmetric(&self) -> bool {
        if self.rows != self.cols {
            return false;
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                if (self.data[i][j] - self.data[j][i]).abs() > 1e-10 {
                    return false;
                }
            }
        }
        true
    }

    fn scalar_multiply(&self, scalar: f64) -> Matrix {
        let mut result = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] *= scalar;
            }
        }
        result
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            write!(f, "[")?;
            for (j, &val) in row.iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:8.2}", val)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl Add for Matrix {
    type Output = Result<Matrix, String>;

    fn add(self, other: Matrix) -> Self::Output {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("Matrices must have the same dimensions for addition".to_string());
        }

        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Ok(result)
    }
}

impl Sub for Matrix {
    type Output = Result<Matrix, String>;

    fn sub(self, other: Matrix) -> Self::Output {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("Matrices must have the same dimensions for subtraction".to_string());
        }

        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        Ok(result)
    }
}

impl Mul for Matrix {
    type Output = Result<Matrix, String>;

    fn mul(self, other: Matrix) -> Self::Output {
        if self.cols != other.rows {
            return Err("Number of columns in first matrix must equal number of rows in second matrix".to_string());
        }

        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        Ok(result)
    }
}

fn input_matrix() -> Result<Matrix, String> {
    println!("Enter matrix dimensions:");
    print!("Rows: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let rows: usize = input.trim().parse()
        .map_err(|_| "Invalid number of rows")?;

    print!("Columns: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let cols: usize = input.trim().parse()
        .map_err(|_| "Invalid number of columns")?;

    let mut data = Vec::new();
    
    println!("Enter matrix elements row by row (space-separated):");
    for i in 0..rows {
        print!("Row {}: ", i + 1);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let row: Result<Vec<f64>, _> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect();
        
        match row {
            Ok(row_data) => {
                if row_data.len() != cols {
                    return Err(format!("Row {} must have {} elements", i + 1, cols));
                }
                data.push(row_data);
            }
            Err(_) => return Err(format!("Invalid numbers in row {}", i + 1)),
        }
    }

    Matrix::from_vec(data)
}

fn demo_matrices() {
    println!("\n=== Matrix Demo ===");
    
    // Create sample matrices
    let matrix_a = Matrix::from_vec(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ]).unwrap();

    let matrix_b = Matrix::from_vec(vec![
        vec![9.0, 8.0, 7.0],
        vec![6.0, 5.0, 4.0],
        vec![3.0, 2.0, 1.0],
    ]).unwrap();

    println!("Matrix A:");
    print!("{}", matrix_a);
    
    println!("Matrix B:");
    print!("{}", matrix_b);

    // Addition
    match matrix_a.clone() + matrix_b.clone() {
        Ok(sum) => {
            println!("A + B:");
            print!("{}", sum);
        }
        Err(e) => println!("Addition error: {}", e),
    }

    // Multiplication
    match matrix_a.clone() * matrix_b.clone() {
        Ok(product) => {
            println!("A × B:");
            print!("{}", product);
        }
        Err(e) => println!("Multiplication error: {}", e),
    }

    // Transpose
    println!("Transpose of A:");
    print!("{}", matrix_a.transpose());

    // Determinant
    match matrix_a.determinant() {
        Ok(det) => println!("Determinant of A: {:.2}", det),
        Err(e) => println!("Determinant error: {}", e),
    }

    // Identity matrix
    let identity = Matrix::identity(3);
    println!("3×3 Identity Matrix:");
    print!("{}", identity);
}

fn main() {
    println!("🔢 Matrix Operations Calculator");
    println!("==============================");

    let mut matrices: Vec<Matrix> = Vec::new();

    loop {
        println!("\nOptions:");
        println!("1. Create new matrix");
        println!("2. Display matrices");
        println!("3. Add two matrices");
        println!("4. Subtract two matrices");
        println!("5. Multiply two matrices");
        println!("6. Transpose matrix");
        println!("7. Calculate determinant");
        println!("8. Matrix properties");
        println!("9. Run demo");
        println!("10. Exit");
        print!("Choose an option (1-10): ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => {
                match input_matrix() {
                    Ok(matrix) => {
                        matrices.push(matrix.clone());
                        println!("✓ Matrix created (index {}):", matrices.len() - 1);
                        print!("{}", matrix);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                if matrices.is_empty() {
                    println!("No matrices created yet.");
                } else {
                    for (i, matrix) in matrices.iter().enumerate() {
                        println!("Matrix {} ({}×{}):", i, matrix.rows, matrix.cols);
                        print!("{}", matrix);
                    }
                }
            }
            "9" => demo_matrices(),
            "10" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice! Please enter 1-10.");
            }
        }
    }
}