use rand::Rng;
use std::{io,fs};
use serde::{Deserialize, Serialize};
use serde_json::Value;
// cell struct
#[derive(Debug,Serialize)]
struct Cell {
    height_of_cell: u16,
    width_of_cell: u16,
    value: String,
}

impl Cell {
    // creating new instance of cell
    fn new(value: String, height: u16, width: u16) -> Cell {
        Cell {
            value,
            height_of_cell: height,
            width_of_cell: width,
        }
    }
}
#[derive(Debug,Serialize)]

struct Row {
    cells: Vec<Cell>,
    height_of_row: u16,
    width_of_row: u16,
}
impl Row {
    // creating new instance of cell
    fn new(cells: Vec<Cell>) -> Row {
        Row {
            cells,
            height_of_row: 0,
            width_of_row: 0,
        }
    }
    // calculating height and width of row based on cells
    fn calc_height_and_width(&mut self) {
        let mut max_height: u16 = 0;
        let mut total_width: u16 = 0;
        for i in 0..self.cells.len() {
            if max_height < self.cells[i].height_of_cell {
                max_height = self.cells[i].height_of_cell;
            }
            total_width += self.cells[i].width_of_cell;
        }
        self.height_of_row = max_height;
        self.width_of_row = total_width;
    }
}

// Struct representing the table
#[derive(Debug,Serialize)]
struct Table {
    rows: Vec<Row>,
    height_of_table: u16,
    width_of_table: u16,
}

impl Table {
    // creating new instance of table
    fn new(rows: Vec<Row>) -> Table {
        Table {
            rows,
            height_of_table: 0,
            width_of_table: 0,
        }
    }

    // calculating height and width of table based on rows
    fn calc_height_and_width(&mut self) {
        let mut table_height: u16 = 0;
        let mut table_width: u16 = 0;
        for i in 0..self.rows.len() {
            table_height += self.rows[i].height_of_row;
            table_width += self.rows[i].width_of_row;
        }
        self.height_of_table = table_height;
        self.width_of_table = table_width;
    }
//function to change value
    fn change_value(&mut self,row:usize,cell:usize,val:String){
        self.rows[row].cells[cell].value = val.to_string();
        // recalculating height and width of row
        self.rows[row].calc_height_and_width();
        // recalculating height and width of table
        self.calc_height_and_width();
    }
}
fn main() {
    let no_of_rows = 5;
    let no_of_cells_in_row = 5;
    // initializing vec<Row> to store rows
    let mut table_vec: Vec<Row> = vec![];
    for _ in 0..no_of_rows {
        // initializing vec<Cell> to store cells
        let mut row_vec: Vec<Cell> = vec![];
        for _ in 0..no_of_cells_in_row {
            //storing random value in cells 
            let value = rand::thread_rng().gen_range(0..=100);
            let height = rand::thread_rng().gen_range(1..9) * 10;
            let width = 30;
            //creating instance of cell
            let cell = Cell::new(value.to_string(), height, width);
            row_vec.push(cell);
        }
        // creating instance of row
        let mut row: Row = Row::new(row_vec);
        row.calc_height_and_width();
        table_vec.push(row);
    }
    //creating instance of table
    let mut table = Table::new(table_vec);
    table.calc_height_and_width();
    
//changing value of a particular cell
table.change_value(3, 4, "hello".to_string());

//Creating a new json file to store the data
let file_str = serde_json::to_string_pretty(&table);
match file_str{
    Ok(file)=>{
        fs::write("./data.json", &file).expect("failed to write file");
    }

    Err(_)=>{
        println!("Failed to convet file");
    }
}

}
