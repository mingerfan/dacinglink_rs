use std::collections::HashSet;
use std::fmt::Display;

use rand::Rng;

// Custom function to format a 2D Vec and return a string
pub fn format_2d_string<T: Display>(vec_2d: &Vec<Vec<T>>) -> String {
    // Calculate the maximum width of each column
    let col_widths = calculate_col_widths(vec_2d);
    let x_axis_vec: Vec<_> = (0..vec_2d[0].len()).collect();
    println!("x len: {}", x_axis_vec.len());
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

// Generate a sparse matrix and ensure there is a solution
fn generate_sparse_matrix_with_solution(
    rows: usize,
    cols: usize,
    solution_rows: usize,
) -> Vec<Vec<usize>> {
    assert!(solution_rows <= rows);
    let mut matrix = vec![vec![0; cols]; rows];
    let mut covered_cols = vec![false; cols];

    // Randomly select some rows as part of the solution
    let mut selected_rows: HashSet<usize> = HashSet::new();
    while selected_rows.len() < solution_rows {
        selected_rows.insert(rand::random::<usize>() % rows);
    }

    let mut rng = rand::thread_rng();

    for &row in &selected_rows {
        for (col, covered_col) in covered_cols.iter_mut().enumerate() {
            if rng.gen_bool((col + 1) as f64 / cols as f64) && !*covered_col {
                matrix[row][col] = 1;
                *covered_col = true;
            }
        }
    }

    matrix
}
