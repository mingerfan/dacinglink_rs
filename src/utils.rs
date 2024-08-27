
#[cfg(test)]
use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
};

use serde::{Deserialize, Serialize};

// Custom function to format a 2D Vec and return a string
pub fn format_2d_string<T: std::fmt::Display>(vec_2d: &Vec<Vec<T>>) -> String {
    // Calculate the maximum width of each column
    let col_widths = calculate_col_widths(vec_2d);
    let x_axis_vec: Vec<_> = (0..vec_2d[0].len()).collect();
    // println!("x len: {}", x_axis_vec.len());
    let x_tick_label_widths = calculate_col_widths(&vec![x_axis_vec.clone()]);

    let mut result = String::new();

    result.push_str("r/c\t");
    for (col, val) in x_axis_vec.iter().enumerate() {
        let col_width = std::cmp::max(col_widths[col], x_tick_label_widths[col]);
        let formatted_value = format!(
            "{:width$} ",
            val,
            width = if col_width < 2 { 2 } else { col_width }
        );
        result.push_str(&formatted_value);
    }
    result.push('\n');
    for (row_idx, row) in vec_2d.iter().enumerate() {
        result.push_str(&format!("{row_idx}\t"));
        for (col, value) in row.iter().enumerate() {
            // Use the format! macro to add each element to the string, aligned by column width
            let col_width = std::cmp::max(col_widths[col], x_tick_label_widths[col]);
            let formatted_value = format!(
                "{:width$} ",
                value,
                width = if col_width < 2 { 2 } else { col_width }
            );
            result.push_str(&formatted_value);
        }
        result.push('\n'); // Add a newline at the end of each row
    }

    result
}

// Calculate the maximum width of each column
fn calculate_col_widths<T: ToString>(vec_2d: &Vec<Vec<T>>) -> Vec<usize> {
    let mut col_widths = vec![0; vec_2d[0].len()];
    for row in vec_2d {
        for (col, value) in row.iter().enumerate() {
            let width = value.to_string().len();
            if width > col_widths[col] {
                col_widths[col] = width;
            }
        }
    }
    col_widths
}

#[cfg(test)]
// Generate a sparse matrix and ensure there is a solution
pub fn generate_sparse_matrix_with_solution(
    rows: usize,
    cols: usize,
    solution_rows: usize,
) -> (Vec<Vec<usize>>, Vec<usize>) {
    use std::collections::HashSet;

    assert!(solution_rows <= rows);
    let mut matrix = vec![vec![0; cols]; rows];
    let mut covered_cols = vec![false; cols];

    // Randomly select some rows as part of the solution
    let mut selected_rows: HashSet<usize> = HashSet::new();
    while selected_rows.len() < solution_rows {
        selected_rows.insert(rand::random::<usize>() % rows);
    }

    for &row in &selected_rows {
        for (col, covered_col) in covered_cols.iter_mut().enumerate() {
            if rand::random::<bool>() && !*covered_col {
                matrix[row][col] = 1;
                *covered_col = true;
            }
        }
    }

    // Ensure all columns are covered by at least one '1'
    for (col, covered_col) in covered_cols.iter_mut().enumerate() {
        if !*covered_col {
            // This is a random number
            let row = *selected_rows.iter().next().unwrap();
            matrix[row][col] = 1;
        }
    }

    let mut rng = rand::thread_rng();

    for (_, m_cols) in matrix
        .iter_mut()
        .enumerate()
        .filter(|(row, _)| !selected_rows.contains(row))
    {
        for m_item in m_cols.iter_mut() {
            if rand::Rng::gen_bool(&mut rng, 0.1) {
                *m_item = 1;
            }
        }
    }

    let test = selected_rows
        .iter()
        .map(|idx| matrix[*idx].clone())
        .collect();
    assert!(check_dl_res(test, false), "Failed to gen valid matrix");

    let mut sol: Vec<_> = selected_rows.into_iter().collect();
    sol.sort();

    (matrix, sol)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Matrix(pub Vec<Vec<usize>>);

#[cfg(test)]
// Function to save a failed test case to a file
pub fn save_failed_case(matrix: &Matrix, sol: &[usize], file_path: &str) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open file");
    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &(matrix, sol)).expect("Failed to write matrix");
    writer.write_all(b"\n").expect("Failed to write newline");
}

#[cfg(test)]
pub fn change_sol_base_idx(sol: &[usize]) -> Vec<usize> {
    sol.iter().map(|idx| idx + 1).collect()
}

#[cfg(test)]
type MatWithSol = (Matrix, Vec<usize>);

#[cfg(test)]
// Function to load all failed test cases from a file
pub fn load_failed_cases(file_path: &str) -> Result<Vec<MatWithSol>, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);

    let res = reader
        .lines()
        .filter_map(|line| line.ok().and_then(|s| serde_json::from_str(&s).ok()))
        .collect();
    Ok(res)
}

#[macro_export]
macro_rules! println_cod {
    ($sel: expr, $($arg: tt)*) => {
        if $sel {
            println!($($arg)*)
        }
    };
}

#[cfg(test)]
pub fn check_dl_res(solution: Vec<Vec<usize>>, cod: bool) -> bool {
    assert!(!solution.is_empty());
    let mut sum = vec![0usize; solution[0].len()];
    for vector in solution {
        sum = sum.iter().zip(vector).map(|(a, b)| a + b).collect();
    }
    println_cod!(cod, "column sum: {:?}", sum);
    for i in sum {
        if i != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn test() {
        let res = generate_sparse_matrix_with_solution(20, 20, 8).0;
        let res = format_2d_string(&res);
        println!("{res}")
    }

    #[test]
    fn test_case_store_load() {
        let mat = Matrix(vec![vec![1, 2, 3], vec![1, 2, 3]]);
        let sol = vec![1, 1, 1];
        save_failed_case(&mat, &sol, "cases_test.temptxt");
        save_failed_case(&mat, &sol, "cases_test.temptxt");
        save_failed_case(&mat, &sol, "cases_test.temptxt");
        let res = std::panic::catch_unwind(|| {
            for i in load_failed_cases("cases_test.temptxt").unwrap() {
                println!("{:?}", i);
                let (mat_read, sol_read) = i;
                assert_eq!(mat_read, mat);
                assert_eq!(sol_read, sol);
            }
        });
        let _ = fs::remove_file("cases_test.temptxt");
        assert!(res.is_ok())
    }
}
