use std::fmt::Write;

pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    let mut col_widths = vec![0; headers.len()];

    // Calculate column widths
    for (i, h) in headers.iter().enumerate() {
        col_widths[i] = h.len();
    }

    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if cell.len() > col_widths[i] {
                col_widths[i] = cell.len();
            }
        }
    }

    // Print header
    let mut header_line = String::new();
    for (i, h) in headers.iter().enumerate() {
        let _ = write!(header_line, "{:<width$}  ", h, width = col_widths[i]);
    }
    println!("{}", header_line.trim_end());

    // Print separator
    let mut sep = String::new();
    for width in &col_widths {
        let _ = write!(sep, "{:-<width$}  ", "", width = *width);
    }
    println!("{}", sep.trim_end());

    // Print rows
    for row in rows {
        let mut line = String::new();
        for (i, cell) in row.iter().enumerate() {
            let _ = write!(line, "{:<width$}  ", cell, width = col_widths[i]);
        }
        println!("{}", line.trim_end());
    }
}
