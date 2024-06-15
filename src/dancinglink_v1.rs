use std::slice::SliceIndex;

#[allow(non_snake_case)]
struct DL {
    r: usize, // row size
    c: usize, // col size
    idx: usize,
    first: Vec<usize>,
    size: Vec<usize>,
    row: Vec<usize>,
    col: Vec<usize>,
    L: Vec<usize>,
    R: Vec<usize>,
    U: Vec<usize>,
    D: Vec<usize>,
}

const DEFAULT_ROW: usize = 10;
const DEFAULT_COL: usize = 10;

impl DL {
    #[allow(non_snake_case)]
    fn new(row_size: usize, col_size: usize) -> Self {
        let row_size = if row_size < 1 { DEFAULT_ROW } else { row_size };
        let col_size = if col_size < 1 { DEFAULT_COL } else { col_size };
        let idx_max = row_size * col_size + 1;
        // Actually, we do not use idx 0, so idx in first and size starts with 1
        let first = vec![0; row_size + 1];
        let size = vec![0; row_size + 1];
        let row = vec![0; idx_max];
        let col = vec![0; idx_max];
        let mut L = vec![0; idx_max];
        let mut R = vec![0, idx_max];
        let mut U = vec![0, idx_max];
        let mut D = vec![0, idx_max];

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
        }
    }

    // row and col idx starts with 1
    fn insert(&mut self, row: usize, col: usize) {
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
            self.R[self.idx] = self.L[self.first[row]];
            self.L[self.first[row]] = self.idx;
            self.R[self.first[row]] = self.idx;
        }
    }
}
