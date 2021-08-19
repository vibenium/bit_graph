# BitGraph
A graph data structured utilizes special encoding/deconding methods to represent weight and vertex information as bits. The purpose of this rust library is to create a more spacious representation of a graph. In theory, this graph will utilize less space depending on the type of 'EdgeScale' that is being used.   
# Example Declarations
````rust
let my_bg1: BitGraph<NoData> = BitGraph::new(EdgeScale::SAME);
let my_bg2: BitGraph<String> = BitGraph::new(EdgeScale::U8);
let my_bg3: BitGraph<i8> = BitGraph::new(EdgeScale::U8);
let my_bg4: BitGraph<Option<i32>> = BitGraph::new_with_capacity(EdgeScale::SAME, 20);
let my_bg5: BitGraph<Vec::<i32>> = BitGraph::new_with_capacity(EdgeScale::U32, 100);
```` 
## Types of BitGraphs
There are currently 6 types of BitGraphs (all types have _unsigned_ weights):
* __SAME__: max weight = 0
* __BINARY__: max weight = 1
* __U4__: max weight = 7 
* __U8__: max weight = 127 
* __U16__: max weight = 32,767  
* __U32__: max weight = 2,147,483,647
 
