use std::fmt::Display;

// Custom function to format a 2D Vec and return a string
pub fn format_2d_string<T: Display>(vec_2d: &Vec<Vec<T>>) -> String {
    // Calculate the maximum width of each column
    let col_widths = calculate_col_widths(vec_2d);
    let mut result = String::new();

    for row in vec_2d {
        for (col, value) in row.iter().enumerate() {
            // Use the format! macro to add each element to the string, aligned by column width
            let formatted_value = format!("{:width$} ", value, width = col_widths[col]);
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
