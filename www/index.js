import { Sudoku } from "sudoku-gen";
import { Grid } from "./grid"

let s = Sudoku.new_empty(9);

let grid = new Grid(s, canvas);
grid.plot();

canvas.addEventListener("click", (e) => {
    grid.onClick(e);
    grid.plot();
})
document.addEventListener("keydown", (e) => {

    if (!grid.selected) return;
    let number = parseInt(e.key)
    if (Number.isInteger(number)) {
        let i, j;
        [i, j] = grid.selected;
        grid.setNumber(i, j, number)
    }
    if (e.key === "ArrowRight") {
        grid.selectRight()
    }
    else if (e.key === "ArrowLeft") {
        grid.selectLeft()
    }
    else if (e.key === "ArrowDown") {
        grid.selectDown()
    }
    else if (e.key === "ArrowUp") {
        grid.selectUp()
    }
    grid.plot();
});

solve.addEventListener("click", (e) => {
    s.solve(0);
    grid.plot();
})