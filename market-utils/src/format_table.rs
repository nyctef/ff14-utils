use std::fmt::Display;

// TODO:
// - make numbers right-aligned
// - make passing &str etc more ergonomic
// maybe have a TableElement enum rather than a single generic impl Display type?
// not sure how to easily pass simple values though
// eg we want to make `table.add_row(["foo", "bar", "baz"]);` simple
//
// https://doc.rust-lang.org/std/any/index.html maybe solves right-aligning numbers
// without any additional type wrapping, but probably doesn't solve the other problems
#[derive(Debug, Clone)]
pub struct Table<T: Sized + Display, const COUNT: usize> {
    rows: Vec<TableRow<T, COUNT>>,
}

#[derive(Debug, Clone)]
enum TableRow<T: Sized + Display, const COUNT: usize> {
    Separator,
    Data([T; COUNT]),
}

impl<T: Sized + Display, const COUNT: usize> Table<T, COUNT> {
    pub fn new() -> Self {
        Table { rows: Vec::new() }
    }

    pub fn add_separator(&mut self) {
        self.rows.push(TableRow::Separator);
    }

    pub fn add_row(&mut self, row: [T; COUNT]) {
        self.rows.push(TableRow::Data(row));
    }

    pub fn format(&self) -> String {
        let mut output = String::new();

        let col_widths: [usize; COUNT] = self
            .rows
            .iter()
            .filter_map(|row| {
                if let TableRow::Data(data) = row {
                    Some(
                        data.iter()
                            .map(|d| d.to_string().len())
                            .collect::<Vec<usize>>()
                            .try_into()
                            .unwrap(),
                    )
                } else {
                    None
                }
            })
            .fold([0; COUNT], |acc, widths: [usize; COUNT]| {
                acc.iter()
                    .zip(widths.iter())
                    .map(|(a, b)| *a.max(b))
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap()
            });

        for row in &self.rows {
            match row {
                TableRow::Separator => {
                    let mut total_width: usize = col_widths.iter().sum::<usize>();
                    total_width += (COUNT - 1) * 3; // for " | " separators
                    output.push_str(&"-".repeat(total_width));
                    output.push('\n');
                }
                TableRow::Data(data) => {
                    output.push_str(
                        &data
                            .iter()
                            .enumerate()
                            .map(|(i, d)| format!("{:width$}", d, width = col_widths[i]))
                            .collect::<Vec<_>>()
                            .join(" | "),
                    );
                    output.push('\n');
                }
            }
        }
        output
    }

    pub fn print(&self) {
        // TODO: implement format trait directly?
        println!("{}", self.format());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table() {
        let mut table: Table<&str, 3> = Table::new();
        table.add_row(["Col1 title", "Col2 title", "Col3 title"]);
        table.add_separator();
        table.add_row(["Row1 Col1 foo", "Row1 Col2", "Row1 Col3"]);
        table.add_row(["Row2 Col1", "Row2 Col2 bar", "Row2 Col3"]);
        let result = table.format();
        assert_eq!(
            result.trim(),
            r#"
Col1 title    | Col2 title    | Col3 title
------------------------------------------
Row1 Col1 foo | Row1 Col2     | Row1 Col3 
Row2 Col1     | Row2 Col2 bar | Row2 Col3 
"#
            .trim()
        );
    }
}
