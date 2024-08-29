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

macro_rules! println_cod {
    ($sel: expr, $($arg: tt)*) => {
        if $sel {
            println!($($arg)*)
        }
    };
}

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
