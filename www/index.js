
import { MyList } from "my-wasm";
import { memory } from "my-wasm/my_wasm_bg.wasm";

let list = MyList.new([9,8,7,4,5,6,3,2,1]);
list.sort();
let grid_ptr = list.get_grid();
let grid = new Uint8Array(memory.buffer, grid_ptr, list.get_size());
console.log(grid);
