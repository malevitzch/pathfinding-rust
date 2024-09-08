#pathfinding-rust
To compile the project you need to have rustc, cargo and gcc installed.  
To run the project simply run the "cargo run" command. The project uses the BFS algorithm to numerically compute the probability of existence of a path from the top left corner of a square grid to bottom right corner, and generates a color-graded 2D graph based on the size of the grid and the probability of a node being a wall. 
The result is stored in "Output_Graph.png", the X axis is map size from 1 to 20 and Y axis is the probability from 0 to 90%.
