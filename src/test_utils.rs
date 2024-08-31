#![cfg(test)]

use std::{
    error::Error, fs::OpenOptions, hash::Hash, io::{BufRead, BufReader, BufWriter, Write}, vec
};

use proptest::prelude::Strategy;
use rand::{seq::SliceRandom, Rng};

use crate::println_cod;
use crate::utils::{format_2d_string, Matrix};

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

pub fn check_multicover(solution: Vec<Vec<usize>>, cod: bool) -> bool {
    assert!(!solution.is_empty());
    let mut sum = vec![0usize; solution[0].len()];
    for vector in solution {
        vector.iter().enumerate().for_each(|(idx, x)| sum[idx] += x)
        // vector.iter().zip(sum.as_mut_slice()).for_each(|(v, sum)| *sum += v );
    }
    println_cod!(cod, "column sum: {:?}", sum);
    for i in sum {
        if i == 0 {
            return false;
        }
    }
    true
}

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

// Generate a sparse matrix and ensure there is a solution
pub fn generate_muticover_matrix(
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
    for &i in selected_rows.iter().take(rng.gen_range(1..=solution_rows)) {
        let mut numbers: Vec<_> = (0..cols).collect();
        numbers.shuffle(&mut rng);
        for j in numbers {
            if matrix[i][j] != 1 {
                matrix[i][j] = 1;
                break;
            }
        }
    }

    let test = selected_rows
        .iter()
        .map(|idx| matrix[*idx].clone())
        .collect();
    assert!(check_multicover(test, false), "Failed to gen valid matrix");

    let mut sol: Vec<_> = selected_rows.into_iter().collect();
    sol.sort();

    (matrix, sol)
}

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

pub fn change_sol_base_idx(sol: &[usize]) -> Vec<usize> {
    sol.iter().map(|idx| idx + 1).collect()
}

type MatWithSol = (Matrix, Vec<usize>);

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
mod test {
    use std::fs;

    use rand::Rng;

    use crate::utils;

    use super::*;

    #[test]
    fn test() {
        let res = generate_sparse_matrix_with_solution(20, 20, 8).0;
        let res = format_2d_string(&res);
        println!("{res}")
    }

    #[test]
    fn test_check_multicover() {
        let vec_test = vec![vec![1, 0, 1], vec![1, 2, 0], vec![1, 3, 5]];
        check_multicover(vec_test, true);
    }

    #[test]
    fn test_gen_multicover() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let row = rng.gen_range(1..=20);
            let col = rng.gen_range(1..=20);
            let sol = rng.gen_range(1..=row);
            let ret = generate_muticover_matrix(row, col, sol);
            println!("{}", utils::format_2d_string(&ret.0));
            check_multicover(ret.0, true);
        }
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
