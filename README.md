# BitGraph
A graph data structured utilizes special encoding/deconding methods to represent weight and vertex information as bits. The purpose of this rust library is to create a more spacious representation of a graph. In theory, this graph will utilize less space depending on the type of _EdgeScale_ that is being used. A lower edgescale means less occupied space. The smallest _enum_ `EdgeScale` is `SAME` and the largest is `U32`    
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
* `SAME`: max weight = 0 = 2^__0__ - 1
* `BINARY`: max weight = 1 = 2^__1__ - 1
* `U4`: max weight = 7 = 2^__4__ - 1
* `U8`: max weight = 127 = 2^__8__ - 1
* `U16`: max weight = 32,767 = 2^__16__ - 1 
* `U32`: max weight = 2,147,483,647 = 2^__32__ - 1
 
