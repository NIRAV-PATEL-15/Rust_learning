use rand::Rng;
use std::io;
#[derive(Debug)]
// cell struct
struct Cell {
    height_of_cell: u16,
    width_of_cell: u16,
    value: String,
    cell_no: u16,
}

impl Cell {
    // creating new instance of cell
    fn new(value: String, height: u16, cell_no: u16) -> Cell {
        Cell {
            value,
            height_of_cell: height,
            width_of_cell: 50,
            cell_no,
        }
    }
}
#[derive(Debug)]

struct Row {
    cells: Vec<Cell>,
    height_of_row: u16,
    width_of_row: u16,
    no_of_cells: u16,
    row_no: u16,
}
impl Row {
    // creating new instance of cell
    fn new(no_of_cells: u16, row_no: u16) -> Row {
        Row {
            cells: vec![],
            height_of_row: 0,
            width_of_row: 0,
            no_of_cells,
            row_no,
        }
    }
    // calculating height and width of row based on cells
    fn calc_height_and_width(&mut self) {
        let max_height = self
            .cells
            .iter()
            .map(|cell| cell.height_of_cell)
            .max()
            .expect("failed to get max height");
        self.height_of_row = max_height;
        let max_width = self.cells.iter().map(|cell| cell.width_of_cell).sum();
        self.width_of_row = max_width;
    }
}
// Struct representing the table
#[derive(Debug)]

struct Table {
    rows: Vec<Row>,
    height_of_table: u16,
    width_of_table: u16,
    num_rows: u16,
    num_cells: u16,
}

impl Table {
    // creating new instance of table
    fn new(no_of_rows: u16, cell_in_row: u16) -> Table {
        Table {
            rows: vec![],
            height_of_table: 0,
            width_of_table: 0,
            num_rows: no_of_rows,
            num_cells: cell_in_row * no_of_rows,
        }
    }

    // calculating height and width of table based on rows
    fn calc_height_and_width(&mut self) {
        let table_height: u16 = self.rows.iter().map(|row| row.height_of_row).sum();
        let table_width: u16 = self
            .rows
            .iter()
            .map(|row| row.width_of_row)
            .max()
            .expect("failed to get width");
        self.height_of_table = table_height;
        self.width_of_table = table_width;
    }
}
fn main() {
    let heights = vec![30, 50, 20, 80, 40];
    let mut input = String::new();
    println!("Enter no of rows and no of cells in a row separated by space");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");
    let numbers: Vec<u16> = input
        .split_whitespace()
        .map(|num| num.parse().expect("Please enter valid numbers"))
        .collect();

    let mut table = Table::new(numbers[0], numbers[1]);

    for i in 0..table.num_rows {
        let mut row = Row::new(numbers[1], i + 1);
        for j in 0..row.no_of_cells {
            let idx = rand::thread_rng().gen_range(0..heights.len());
            let data = rand::thread_rng().gen_range(0..=100);
            let cell = Cell::new(data.to_string(), heights[idx], j + 1);
            row.cells.push(cell);
        }
        row.calc_height_and_width();
        table.rows.push(row);
        table.calc_height_and_width();
    }

    for row in table.rows.iter() {
        println!("\n-------------------");
        for cell in row.cells.iter() {
            print!("| {} |", cell.value);
        }
        println!(
            "<--Height of row : {} and Width of row: {}",
            row.height_of_row, row.width_of_row
        );
        println!("\n-------------------");
    }
    println!(
        "There are {} rows and {} cells in the table",
        table.num_rows, table.num_cells
    );
    println!(
        "Height of table : {} and Width of table: {}",
        table.height_of_table, table.width_of_table
    );
    println!("{:#?}", table);
}
