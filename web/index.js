import { Universe, Cell } from "game-of-life-abridged";
import { memory } from "game-of-life-abridged/game_of_life_abridged_bg";

const CELL_SIZE = 8; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#AFE1AF";

// Render every x milliseconds
const renderSpeed = 10;

// Construct the universe, and get its width and height.
const universe = Universe.init();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  
  setTimeout(function(){ 
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
  }, renderSpeed);  

};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
  };

const getIndex = (row, column) => {
    return row * width + column;
  };
  
const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  
    ctx.beginPath();
  
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);
  
        ctx.fillStyle = cells[idx] === Cell.Dead
          ? DEAD_COLOR
          : ALIVE_COLOR;
  
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  
    ctx.stroke();
  };

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);