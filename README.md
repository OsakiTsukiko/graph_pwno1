# Practical Work No.1

- abstract data type directed graph
- function for reading graph from file
- - in file
- - out file

- map for inbound edges
- map for outbound edges
- map for edge cost
  
### Operations
- get the number of vertices
- check if edge exists
- iterate the vertices
- in degree / out degree of vertex
- outbound edges for vertex (target vertex)
- inbound edges for vertex (source vertex)
- get/set egde cost
- add remove vertex and edges
- graph should be copyable???

### External
- read graph from file
- write graph to file
- create random graph with specified nr of vertices and edges

## OTHER..
- The object returned by the parse functions shall not allow modifying the graph through its public functions. So, don't return sets by reference. Return iterators. 
- Generally, make sure the graph cannot be brought in an inconsistent state by applying public functions on various accessible objects. 
- Save in a new file (ex: graph1k_modif.txt) the graph obtained after the modifications (add edges, delete edges, delete vertices,...). 
- Generate randomly 2 directed graphs with costs and save them in 2 files: 
- - 5.1. random_graph1.txt - a graph with  7 vertices and  20 edges 
- - 5.2. random_graph2.txt - a graph with  6 vertices and  40 edges ???? BRUV