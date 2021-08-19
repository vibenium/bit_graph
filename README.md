# BitGraph
A graph data structured utilizes special encoding/deconding methods to represent weight and vertex information as bits. The purpose of this rust library is to create a more spacious representation of a graph. 
## Types of BitGraphs
There are currently 6 types of BitGraphs (all min weights are 0, except for _SAME_):
* __SAME__: No weights
* __BINARY__: max weight = 1
* __U4__: max weight = 7 
* __U8__: max weight = 127 
* __U16__: max weight = 32,767  
* __U32__: max weight = 2,147,483,647 
