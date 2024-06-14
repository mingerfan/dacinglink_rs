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
        let col_size = if col_size < 1 { DEFAULT_COL } else { col_size + 1};
        let first = Vec::new();
        let size = vec![0; row_size];
        let row = Vec::new();
        let col = Vec::new();
        let mut L = vec![0; col_size];
        let mut R = vec![0, col_size];
        let mut U = vec![0, row_size];
        let mut D = vec![0, row_size];

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

    fn insert(row: usize, col: usize) {
        // Because of an extra 0 idx, when we insert a elem, we should ++idx first
        // Condition 
    }

}
