use std::fmt::Display;

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
        let formatted_value = format!("{:width$} ", val, width = if col_width < 2 { 2 } else { col_width });
        result.push_str(&formatted_value);
    }
    result.push('\n');
    for (row_idx, row) in vec_2d.iter().enumerate() {
        result.push_str(&format!("{row_idx}\t"));
        for (col, value) in row.iter().enumerate() {
            // Use the format! macro to add each element to the string, aligned by column width
            let col_width = std::cmp::max(col_widths[col], x_tick_label_widths[col]);
            let formatted_value = format!("{:width$} ", value, width = if col_width < 2 { 2 } else { col_width });
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
