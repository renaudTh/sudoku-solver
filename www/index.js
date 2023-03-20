import {Sudoku} from "sudoku-gen";
import {memory} from "sudoku-gen/sudoku_bg.wasm"

class Grid {

    plot(){
        this.canvas.width = 50*this.size;
        this.canvas.height = 50*this.size;
        let width = canvas.width;
        let height = canvas.width;
        let step = Math.round(width/this.size);
        let big_step = Math.round(width/Math.sqrt(this.size));
        console.log(step, big_step);
        var ctx = this.canvas.getContext('2d');
        for(let x = step; x < width; x+=step){
            ctx.beginPath();
            ctx.moveTo(x,0);
            ctx.lineTo(x, height);
            if(x % big_step == 0){
                ctx.lineWidth = 3;
            }
            else{
                ctx.lineWidth = 1;
            }
            ctx.stroke();
        }
        for(let y = step; y < height; y+=step){
            ctx.beginPath();
            ctx.moveTo(0,y);
            ctx.lineTo(width, y);
            if(y % big_step == 0){
                ctx.lineWidth = 3;
            }
            else{
                ctx.lineWidth = 1;
            }
            ctx.stroke();
        }
    }
    click(e){
        var ctx = e.target.getContext('2d');
        var rect = e.target.getBoundingClientRect();
        let step = Math.floor(rect.width/grid.size);

        if(this.selected != null){
            ctx.fillStyle = "white";
            ctx.fillRect(this.selected[0]*50+1,this.selected[1]*50+1,48,48);
        }


        let x = (e.clientX - rect.left); 
        let i = Math.floor(x/step)
        let y = (e.clientY - rect.top); 
        let j = Math.floor(y/step);
        this.selected = [i,j]
        ctx.fillStyle = "blue";
        ctx.fillRect(this.selected[0]*50+1,this.selected[1]*50+1,48,48);
        console.log(this.selected);
    }
    constructor(size, canvas){
        this.canvas = canvas;

        this.size = size;
        this.grid = new Array(size*size).fill(0);
        this.selected = null;
        this.plot();
        this.canvas.addEventListener("click", this.click);
    }

}

let grid = new Grid(9, canvas);
