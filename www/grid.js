import { Sudoku } from "sudoku-gen";
import { memory } from "sudoku-gen/sudoku_bg.wasm"

export class Grid {

    plot() {
        this.canvas.width = 50 * this.size;
        this.canvas.height = 50 * this.size;
        let width = canvas.width;
        let height = canvas.width;
        let step = Math.round(width / this.size);
        let big_step = Math.round(width / Math.sqrt(this.size));
        var ctx = this.canvas.getContext('2d');
        for (let x = step; x < width; x += step) {
            ctx.beginPath();
            ctx.moveTo(x, 0);
            ctx.lineTo(x, height);
            if (x % big_step == 0) {
                ctx.strokeStyle = "black";
                ctx.lineWidth = 1;
            }
            else {
                ctx.strokeStyle = "#cccccc";
                ctx.lineWidth = 1;
            }
            ctx.stroke();
        }
        for (let y = step; y < height; y += step) {
            ctx.beginPath();
            ctx.moveTo(0, y);
            ctx.lineTo(width, y);
            if (y % big_step == 0) {
                ctx.strokeStyle = "black";
                ctx.lineWidth = 1;
            }
            else {
                ctx.strokeStyle = "#cccccc";
                ctx.lineWidth = 1;
            }
            ctx.stroke();
        }
        if (this.selected != null) {
            this.plotSelected(this.selected);
        }
        this.plotGridContent();

    }
    plotGridContent() {
        var ctx = this.canvas.getContext('2d');
        ctx.font = "25px sans serif";
        ctx.textAlign = "start"
        ctx.textBaseline = "hanging"

        for (let i = 0; i < this.size; i++) {
            for (let j = 0; j < this.size; j++) {
                let index = i * this.size + j
                let value = this.grid[index];
                if (value != 0) {
                    let m = ctx.measureText(value);
                    let pos_x = i * 50 + (50 - m.width) / 2
                    let pos_y = j * 50 + (50 - m.width) / 2
                    ctx.fillText(this.grid[index], pos_x, pos_y);
                }
            }
        }
    }
    plotSelected() {
        var ctx = this.canvas.getContext('2d');
        if (this.selected != null) {
            ctx.fillStyle = "white";
            ctx.fillRect(this.selected[0] * 50 + 1, this.selected[1] * 50 + 1, 48, 48);
        }
        ctx.fillStyle = "#cce6ff";
        ctx.fillRect(this.selected[0] * 50 + 1, this.selected[1] * 50 + 1, 48, 48);
        ctx.fillStyle = "black";
    }
    setSelected(row, col) {
        this.selected = [row, col]
    }
    selectRight() {
        let i, j;
        [i, j] = this.selected;
        if (i + 1 >= this.size) {
            j++;
            i = 0;
        }
        else if (i >= this.size - 1 && j >= this.size - 1) {
            i = this.size - 1;
            j = this.size - 1;
        }
        else {
            i++;
        }
        this.selected = [i, j];
    }
    selectLeft() {
        let i, j;
        [i, j] = this.selected;
        if (i - 1 < 0) {
            j--;
            i = this.size - 1;
        }
        else if (i == 0 && j == 0) {
            i = 0;
            j = 0;
        }
        else {
            i--;
        }
        this.selected = [i, j];
    }
    selectDown() {
        let i, j;
        [i, j] = this.selected;
        if (j >= this.size - 1) {
            j = 0;
            i++;
        }
        else {
            j++;
        }
        this.selected = [i, j];
    }
    selectUp() {
        let i, j;
        [i, j] = this.selected;
        if (j - 1 < 0) {
            j = this.size - 1;
            i--;
        }
        else {
            j--;
        }
        this.selected = [i, j];
    }
    setNumber(row, col, number) {
        let i = row * this.size + col;
        this.sudoku.set(row, col, number);
    }
    onClick(e) {
        var rect = this.canvas.getBoundingClientRect();
        let step = Math.floor(rect.width / this.size);
        let i = Math.floor((e.clientX - rect.left) / step)
        let j = Math.floor((e.clientY - rect.top) / step);
        this.setSelected(i, j)
    }
    constructor(sudoku, canvas) {
        this.sudoku = sudoku;
        this.canvas = canvas;
        this.size = this.sudoku.get_size();
        let grid_ptr = this.sudoku.get_grid_ptr();
        this.grid = new Uint8Array(memory.buffer, grid_ptr, 81);
        this.selected = null;
    }

}