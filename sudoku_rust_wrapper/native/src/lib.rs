#[macro_use]
extern crate neon;
#[macro_use]
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize)]
struct User {
    name: String,
    age: u16,
}

const N : usize = 9;
#[derive(Serialize)]
struct Board {
    grid: [[usize; N]; N]
}

const UNASSIGNED : usize= 0;

fn find_unassigned_location(grid:[[usize; N]; N],
							rowindex: &mut usize, colindex: &mut usize) -> bool
{
    for (_i, row) in grid.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            // print!("{}", col);
            if *col == UNASSIGNED {
                *rowindex = _i ;
                *colindex = _j ;
                return true
            }
        }
    }
    false
}

fn used_in_row(grid:[[usize; N]; N],rowindex:  usize, num:  usize) -> bool
{
    for col in 0..N {
        if grid[rowindex][col] == num{
            return true
        }
    }
    false
}

fn used_in_col(grid:[[usize; N]; N], colindex: usize, num: usize) -> bool
{
    for row in 0..N {
        if grid[row][colindex] == num{
            return true
        }
    }
    false
}

fn used_in_box(grid:[[usize; N]; N], box_start_row: usize,  box_start_col: usize,  num: usize) -> bool
{
	for row in 0..3{
        for col in 0..3 {
            if grid[row + box_start_row][col + box_start_col] == num{
                return true
            }
        }
    }
	 false
}


fn is_safe(grid:[[usize; N]; N], row:usize, col:usize, num:usize) -> bool
{
	return !used_in_row(grid, row, num) &&
		   !used_in_col(grid, col, num) &&
		   !used_in_box(grid, row - row % 3 , col - col % 3, num) &&
			grid[row][col] == UNASSIGNED
}

fn print_grid(grid:[[usize;N];N])
{
    for (_i, row) in grid.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {

            print!("{}  ", col);
        }
        println!();
        println!();
    }
    println!();

}

fn solve_sudoku(grid:&mut [[ usize; N]; N]) -> bool
{
	let mut col : usize = 0 ;
    let mut row : usize = 0 ;


	if !find_unassigned_location(*grid, & mut row, & mut col)
	   {return true} // success!


	for num in 1..(N+1){

		if is_safe(*grid, row, col, num)
		{
			grid[row][col] = num;

			if solve_sudoku( grid){
                return true

            }

			grid[row][col] = UNASSIGNED;
		}
	}
	return false
}


use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn congradulations(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("congradulations"))
}

fn sudokuSolve(mut cx: FunctionContext) -> JsResult<JsValue> {

    let arg0 = cx.argument::<JsValue>(0)?;

    let mut arg0_value: [[usize; N]; N] = neon_serde::from_value(&mut cx, arg0)?;

    solve_sudoku(&mut arg0_value);

    // let value : [[usize; N]; N] = [[8,0,0,0,0,0,0,0,0],
    //       [0,0,3,6,0,0,0,0,0],
    //       [0,7,0,0,9,0,2,0,0],
    //       [0,5,0,0,0,7,0,0,0],
    //       [0,0,0,0,4,5,7,0,0],
    //       [0,0,0,1,0,0,0,3,0],
    //       [0,0,1,0,0,0,0,6,8],
    //       [0,0,8,5,0,0,0,1,0],
    //       [0,9,0,0,0,0,4,0,0]];

    let js_value = neon_serde::to_value(&mut cx, &arg0_value)?;
    Ok(js_value)
}

register_module!(mut cx, {
    cx.export_function("hello", hello);
    cx.export_function("congradulations", congradulations);
    cx.export_function("sudokuSolve", sudokuSolve)
});
