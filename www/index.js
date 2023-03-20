import {Sudoku} from "sudoku-gen";
import {memory} from "sudoku-gen/sudoku_bg.wasm"

let s = Sudoku.new_for_test();
const g_ptr = s.get_grid_ptr();
let grid = new Uint8Array(memory.buffer, g_ptr, 81);
console.log(grid);
s.solve(0);
console.log(grid);