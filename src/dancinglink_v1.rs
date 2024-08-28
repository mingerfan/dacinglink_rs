use std::fmt::Display;

use crate::utils;

// Code Reference: http://magic.vicp.io/oi-wiki/search/dlx/
// Code Reference: https://blog.csdn.net/nameofcsdn/article/details/132225150

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct DL {
    r: usize, // row size
    c: usize, // col size
    idx: usize,
    first: Vec<usize>,
    size: Vec<isize>,
    row: Vec<usize>,
    col: Vec<usize>,
    L: Vec<usize>,
    R: Vec<usize>,
    U: Vec<usize>,
    D: Vec<usize>,
    res: Option<Vec<usize>>, // usize: the max deep when dancing
}

const DEFAULT_ROW: usize = 10;
const DEFAULT_COL: usize = 10;
const MAX_DEEP: usize = 100;

impl DL {
    #[allow(non_snake_case)]
    pub fn new(row_size: usize, col_size: usize) -> Self {
        let row_size = if row_size < 1 { DEFAULT_ROW } else { row_size };
        let col_size = if col_size < 1 { DEFAULT_COL } else { col_size };
        let idx_max = row_size * col_size + 1;
        // Actually, we do not use idx 0, so idx in first and size starts with 1
        let first = vec![0; row_size + 1];
        let size = vec![0; col_size + 1];
        let row = vec![0; idx_max];
        let col = vec![0; idx_max];
        let mut L = vec![0; idx_max];
        let mut R = vec![0; idx_max];
        let mut U = vec![0; idx_max];
        let mut D = vec![0; idx_max];
        let res = None;

        // We build a new virtual row, but we don't add them to row or col vectors
        // Note: Extra idx 0 element
        for i in 0..=col_size {
            // In virtual row, we link each element to itself vertically
            D[i] = i;
            U[i] = i;
            // In the horizontal direction, the elements in the virtual row link to each other
            // ->0->1->...->col_size->
            // <-0->1<-...<-col_size<-
            L[i] = if i != 0 { i - 1 } else { col_size };
            R[i] = if i != col_size { i + 1 } else { 0 };
        }

        // We maintain a global idx and it is in ascending order
        // when we are constructing this Cross-Linked List
        let idx = col_size;

        DL {
            r: row_size,
            c: col_size,
            idx,
            first,
            size,
            row,
            col,
            L,
            R,
            U,
            D,
            res,
        }
    }

    // row and col idx starts with 1
    pub fn insert(&mut self, row: usize, col: usize) {
        assert!(
            row <= self.r && col <= self.c,
            "Insert: row or col is out of index"
        );
        // Because of an extra 0 idx, when we insert a elem, we should ++idx first
        self.idx += 1;
        self.row[self.idx] = row;
        self.col[self.idx] = col;
        self.size[col] += 1;
        // Idx directly links to col element in virtual row
        // Like head insert in linked list
        self.U[self.idx] = col;
        self.D[self.idx] = self.D[col];
        self.U[self.D[col]] = self.idx;
        self.D[col] = self.idx;
        // Condition 1
        // There is no element in row r, we directly insert it into this row
        // and let the point first point to this element
        if self.first[row] == 0 {
            self.first[row] = self.idx;
            self.L[self.idx] = self.idx;
            self.R[self.idx] = self.idx;
        } else {
            // Once the first[row] is not 0, it can't be change when inserting
            // So, we insert a new element after element the first[row] points to
            self.L[self.idx] = self.first[row];
            self.R[self.idx] = self.R[self.first[row]];
            self.L[self.R[self.first[row]]] = self.idx;
            self.R[self.first[row]] = self.idx;
        }
    }

    fn get_2d_vec(&self) -> Vec<Vec<usize>> {
        let mut res_vec = vec![vec![0; self.c + 1]; self.r + 1];
        for (r, row) in res_vec.iter_mut().enumerate().take(self.r + 1).skip(1) {
            let first_idx = self.first[r];
            let mut idx = first_idx;
            while idx != 0 && self.L[idx] != 0 {
                row[self.col[idx]] = 1;
                idx = self.R[idx];
                if idx == first_idx {
                    break;
                }
            }
        }
        res_vec
    }
}

// It is maybe only correct before any removal operation.
impl Display for DL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_ = utils::format_2d_string(&self.get_2d_vec());
        write!(f, "{str_}")
    }
}

impl DL {
    // In this function, we will remove the column and the corresponding rows to which items in this column are linked.
    fn remove(&mut self, col: usize) {
        let mut vertical_idx = self.D[col];
        let mut horizontal_idx;
        self.L[self.R[col]] = self.L[col];
        self.R[self.L[col]] = self.R[col];
        while vertical_idx != col {
            horizontal_idx = self.R[vertical_idx];
            while horizontal_idx != vertical_idx {
                self.U[self.D[horizontal_idx]] = self.U[horizontal_idx];
                self.D[self.U[horizontal_idx]] = self.D[horizontal_idx];
                self.size[self.col[horizontal_idx]] -= 1;
                horizontal_idx = self.R[horizontal_idx];
            }
            vertical_idx = self.D[vertical_idx];
        }
    }

    // In this function, we will restore the column and the corresponding rows that were previously removed.
    fn recover(&mut self, col: usize) {
        let mut vertical_idx = self.U[col];
        let mut horizontal_idx;
        while vertical_idx != col {
            horizontal_idx = self.L[vertical_idx];
            while horizontal_idx != vertical_idx {
                self.U[self.D[horizontal_idx]] = horizontal_idx;
                self.D[self.U[horizontal_idx]] = horizontal_idx;
                self.size[self.col[horizontal_idx]] += 1;
                horizontal_idx = self.L[horizontal_idx];
            }
            vertical_idx = self.U[vertical_idx];
        }
        self.L[self.R[col]] = col;
        self.R[self.L[col]] = col;
    }

    #[tracing::instrument]
    fn dance_internal(&mut self, deep: usize) -> bool {
        tracing::info!("Into dancing internal");
        // if empty, return false
        if self.R[0] == 0 {
            // In external function, we should ensure self.res is not None
            self.res.as_mut().map(|res_vec| res_vec.truncate(deep));
            return true;
        }
        // Choose the column with least elements
        let mut min = self.R[0];
        let mut horizontal_idx = self.R[0];
        while horizontal_idx != 0 {
            let cur = self.size[horizontal_idx];
            if cur < self.size[min] {
                min = horizontal_idx;
            }
            horizontal_idx = self.R[horizontal_idx]
        }
        tracing::info!("Choose column: {}", min);
        // Attemp to remove the selected column
        self.remove(min);

        let mut vertical_idx = self.D[min];
        let mut horizontal_idx;
        while vertical_idx != min {
            self.res.as_mut().unwrap()[deep] = self.row[vertical_idx];
            horizontal_idx = self.R[vertical_idx];
            while horizontal_idx != vertical_idx {
                self.remove(self.col[horizontal_idx]);
                horizontal_idx = self.R[horizontal_idx];
            }
            if self.dance_internal(deep + 1) {
                return true;
            }
            let mut h_idx_recover = self.L[vertical_idx];
            while h_idx_recover != vertical_idx {
                self.recover(self.col[h_idx_recover]);
                h_idx_recover = self.L[h_idx_recover];
            }
            vertical_idx = self.D[vertical_idx]
        }
        self.recover(min);

        false
    }

    pub fn dance(&mut self) -> Result<Vec<usize>, String> {
        tracing::info!("Into dancing");
        self.res = Some(vec![0; MAX_DEEP]);
        let res = self.dance_internal(0);
        if !res {
            Err("This is a useless info to make clippy happy".to_string())
        } else {
            self.res
                .clone()
                .ok_or("This is a useless info to make clippy happy".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use core::panic;
    use std::{
        sync::mpsc::{self, TryRecvError},
        thread,
        time::{Duration, Instant},
    };

    use proptest::prelude::*;

    use crate::println_cod;

    use super::*;


    #[test]
    fn print_dl() {
        let mut dl = DL::new(30, 30);
        let test_vec = vec![(1, 2), (2, 5), (3, 8), (30, 30), (25, 1), (30, 1)];
        println!("{dl}");
        for (row, col) in test_vec {
            dl.insert(row, col);
        }
        println!("{dl}");
    }

    fn test_base(r: usize, c: usize, case: Vec<Vec<usize>>, cod: bool) -> bool {
        let mut dl = DL::new(r, c);
        for (r_in, c_vec) in case.iter().enumerate() {
            for (c_in, item) in c_vec.iter().enumerate() {
                if *item == 1 {
                    dl.insert(r_in + 1, c_in + 1)
                }
            }
        }
        println_cod!(cod, "array:\n {dl}");
        println_cod!(cod, "array size: {:?}", dl.size);
        let res = dl.dance();
        println_cod!(cod, "dancing res: {:?}", res);
        if let Ok(sol) = res {
            let sol = sol
                .iter()
                .enumerate()
                .filter(|(_, &x)| x != 0)
                .map(|(idx, _)| case[sol[idx] - 1].clone())
                .collect();
            println_cod!(cod, "solution: {:?}", sol);
            if utils::check_dl_res(sol, cod) {
                return true;
            }
        }
        false
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn test_dl_pass((r, s) in (5..=30usize).prop_flat_map(|r|  (Just(r), 3..=r)), c in 5..=30usize) {
            let ret = utils::generate_sparse_matrix_with_solution(r, c, s);
            // if !test_base(r, c, case, false) {
            //     panic!("Test Failed")
            // }
            let case = ret.0.clone();
            let case_clone = case.clone();

            let case_sol = ret.1;

            // Set the timeout duration
            let timeout = Duration::from_secs(10);

            // Create a channel for thread communication
            let (tx, rx) = mpsc::channel();

            // Spawn a new thread to run the test logic
            let handle = thread::spawn(move || {
                let res = test_base(r, c, case_clone, false);
                let _ = tx.send(());
                res
            });

            // Monitor the test thread execution time in the main thread
            let start_time = Instant::now();
            loop {
                match rx.try_recv() {
                    Ok(_) => {
                        // Test thread completed
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        // Continue waiting
                        if start_time.elapsed() > timeout {
                            // Timeout occurred
                            handle.thread().unpark(); // Attempt to wake up the thread to be forcefully terminated
                            println!("mat:\n {}", utils::format_2d_string(&case));
                            println!("sol: {:?}", utils::change_sol_base_idx(&case_sol));
                            utils::save_failed_case(&utils::Matrix(case.clone()), &case_sol, "failed_cases.txt");
                            panic!("Test timed out");
                        }
                    }
                    Err(TryRecvError::Disconnected) => {
                        panic!("Test thread panicked or disconnected");
                    }
                }
                thread::sleep(Duration::from_millis(10)); // Wait a short duration before checking again
            }

            let result = handle.join().expect("Test thread panicked");
            if !result {
                println!("mat:\n {}", utils::format_2d_string(&case));
                println!("sol: {:?}", utils::change_sol_base_idx(&case_sol));
                utils::save_failed_case(&utils::Matrix(case.clone()), &case_sol, "failed_cases.txt");
            }
            prop_assert!(result);
        }
    }

    #[test]
    fn test_dl_special() {
        // let params = [
        //     (5, 16, 2),
        //     (30, 30, 5),
        //     (14, 5, 13),
        // ];
        // for (r, c, s) in params {
        //     let case = utils::generate_sparse_matrix_with_solution(r, c, s);
        //     println!("{}", utils::format_2d_string(&case));
        //     if !test_base(r, c, case, true) {
        //         panic!("Test Failed")
        //     }
        // }
        tracing_subscriber::fmt::init();

        if let Ok(cases) = utils::load_failed_cases("failed_cases.txt") {
            let mut test_res = vec![true; cases.len()];
            for (idx, (mat, sol)) in cases.into_iter().enumerate() {
                if !test_base(mat.0.len(), mat.0[0].len(), mat.0.clone(), true) {
                    println!("Official solution is {:?}", utils::change_sol_base_idx(&sol));
                    let sol_mat: Vec<_> = sol.iter().map(|i| mat.0[*i].clone()).collect();
                    print!("{}", utils::format_2d_string(&sol_mat));
                    let res = utils::check_dl_res(sol_mat, true);
                    assert!(res, "check official solution failed");
                    test_res[idx] = false
                }
            }
            println!("result list: {:?}", test_res);
            assert!(test_res.iter().fold(true, |acc, x| acc & x), "Test Failed");
        } else {
            println!("Failed to Load failed cases!");
        }
    }
}
