# BitGraph
A graph data structured utilizes special encoding/deconding methods to represent weight and vertex information as bits. The purpose of this rust library is to create a more spacious representation of a graph. In theory, this graph will utilize less space depending on the type of _EdgeScale_ that is being used. A lower edgescale means less occupied space. The smallest _enum_ `EdgeScale` is `SAME` and the largest is `U32`. Depending on the bit depth of the machine utilizing this data structure (usually 32 or 64 bit), will also play a part in how much space is being utilized. One way of understanding how `BitGraph` works is by picturing an array where the indices connect to eachother with a restriced amount of weight.      
# Example Declarations
````rust
let bg1: BitGraph<NoData> = BitGraph::new(EdgeScale::SAME);
let bg2: BitGraph<String> = BitGraph::new(EdgeScale::U8);
let bg3: BitGraph<i8> = BitGraph::new(EdgeScale::U4);
```` 
## Types of BitGraphs
There are currently 6 types of BitGraphs (all types have _unsigned_ weights):
* `SAME`: max weight = 0 = 2^__0__ - 1
* `BINARY`: max weight = 1 = 2^__1__ - 1
* `U4`: max weight = 7 = 2^__(4 - 1)__ - 1
* `U8`: max weight = 127 = 2^__(8 - 1)__ - 1
* `U16`: max weight = 32,767 = 2^__(16 - 1)__ - 1 
* `U32`: max weight = 2,147,483,647 = 2^__(32 - 1)__ - 1

## The 'Vertex' struct
vertices (or nodes) within a `BitGraph`
````rust
#[derive(Debug, Clone)]
struct Vertex<T> {
  data: T,
  vertnum: usize,
  edgevert: Vec<usize>
}
````
## What is a vertnum?
A vertnum is a single number that represents the indexing number of the current `Vertex<T>`. This is important for several types of functionality. See more in __src/bg.rs__
## What are edgeverts?
An `edgevert` is where the current `Vertex<T>` at _index_ `vertnum` will contains a vector of numbers with the weight and vertex number destination are encoded. This information is crucial for containing important information on outgoing edges from the current `Vertex<T>` at _index_ `vertnum`.

## The 'BitGraph' struct
````rust
#[derive(Debug, Clone)]
pub struct BitGraph<T> {
  vertices: Vec<Vertex<T>>,
  vert_bit_indexing: usize,
  max_weight: usize,
  partition: usize, 
  bits: usize, 
}
````
* `vertices`: A vector of type `Vertex<T>`
* `vert_bit_indexing`: The machines bits divided by `partition`. Used for indexing the vertex bit position in any given `Vertex<T>`'s `edgevert` vector. This is primarily used to keep the bit position of the vertex within a its range of bits. For instance, in binary, `SAME` = 1, `BINARY` = 10, `U4` = 1000, `U8` = 10000000, etc. Every _1_ is where the `vert_bit_indexing` (or `vbi` for short) lies within every `edgevert`. 
* `max_weight`: The maximum weight of any vertex within the constructed `BitGraph<T>`. This number is useful for checking if a given weight is within the specified range (which is completely dependent on the chosen `EdgeScale`).
* `partition`: The amount of bits occupied for the `vbi` and the weight for a single representation of a connection from the `vertnum` vertex to the destination `vertnum` (i.e., using the array analogy from the intro of thisREADME, the connection of the array indices encoded in a few numbers, A.K.A, `edgevert`).

# Pros and Cons
  __Pros__:
  	#1 Space Efficiency => All vertex and weight info is stored into single numbers
	#2 Fast (dis)connections => Uses bitwise operators to encode vertex/weight info
	#3 BitGraph + HashMap => Making index identifying easier (explained later)
