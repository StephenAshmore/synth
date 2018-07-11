# Plans

### Main
Main will contain the initial spawning of the graph thread and cli thread. Joining them when execution ends.

### CLI
CLI will contain the code that draws the synth information onto the terminal, gets input from the user, validates commands, and so forth.
- Call commands in the graph to validate
- Call functions in the graph to add components to the graph data structure

### Graph
Graph contains the code that iterates through the graph, executing each component along its path.
- Pass an input buffer to each component and receive an output buffer. 
- Contain code for adding components to the graph
- Contain code for validating that components exist (for validation to the CLI code that validates user input)
- When there is a split in the graph where input goes to two places, we will need to either employ breadth-first traversal of the graph or use threads. BF traversal should be superior for my needs.

### Components
All individual components (oscillators, etc) will live in the components folder. They will all follow a similar trait, so that they can be employed in the graph without determining their type.