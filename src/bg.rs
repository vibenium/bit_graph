/*
    BitGraph Description: BitGraph is a graph (directed) data structure that uses usize 
    values to store edge wieghts and their corresponding vertex.   
*/
#![allow(non_snake_case)]

pub mod bit_graph {

    #[derive(Debug, Clone)]
    struct Vertex<T> {  
        data: T,
        vertnum: usize, // the 'source' vertex number
        edgevert: Vec<usize> // the 'destination' vertex number(s) and their weight(s)
    } 

    impl<T> Vertex<T> {

        // used when more edgeverts are needed to initialize potential connections
        pub fn push_new_ev(&mut self) { self.edgevert.push(0); } // 0x000.....

        pub fn get_ev_size(&self) -> usize { self.edgevert.len() }

        /*
            connect_to: This function establishes a connection between a source and destination vertex.
            It is called from the bitgraphs connect() and try_connect() functions. Establishing the 
            source connection is the simplest part of the process, and does not need to directly 
            interact with a Vertex<T> type. It is therefore, not a necessary step here. The comments
            below will have detailed descriptions about the inner workings of the function...

            bitnum: The destination vertex.
            vbi: vert_bit_indexing from a given BitGraph<T>. Needed for indexing the correct edgevert. 
            
            In connect() impl of BitGraph<T>...
            self.vertices[source].connect_to(dest, weight, self.vert_bit_indexing);
        
            self.edgevert[bitnum / vbi] ... EXPLAINED: The amount of edgeverts is proportional to the amount
            of bits on a maching, vertices in a given graph, and which EdgeScale has been choosen at the
            initialization of the graph. When there are not enough bits in a number to store a vertex and
            its weights, another edgevert will be needed. self.edgevert[bitnum / vbi] ensures that any bitnum
            (destination vertex) will find the right edgevert based on the amount of bits divided by the 
            partition size (also known as vert_bit_indexing in the BitGraph<T> struct) to establish a 
            connection for any vertnum from 0 to infinity.

            ... |= (1 << (es - 1)) << (bitnum % vbi) EXPLAINED: 
        */
        
        // NEED TO ADD 'es'
        pub fn connect_to(&mut self, bitnum: usize, weight: usize, vbi: usize) {
            self.edgevert[bitnum / vbi] |= (1 << (es - 1)) << (bitnum % vbi); 
            // NEED TO APPLY WEIGHT BELOW...
        }
    }

    /*
        EdgeScale: This enum dictates how many bits are within each 
        edgevert (minus 1 for the vertex position). If SAME is chosen, 
        then there are no weighted edges between vertices. SAME is 
        useful for just establishing connections between vertices 
        without needing to specify a certain weight. Once an 
        EdgeScale has been chosen, it cannot be changed.
        
        U4 ->
    */
    #[derive(PartialEq, Debug, Clone)]
    pub enum EdgeScale {
        SAME, BINARY, U4, U8, U16, U32
    }

    // Used only when the user does not want to store any data inside a Vertex
    // pub struct NoData;

    #[derive(Debug, Clone)]
    pub struct BitGraph<T> {
        vertices: Vec<Vertex<T>>,
        vert_bit_indexing: usize,
        max_weight: usize, // The max weight an edge can have 
        partition: usize, // how the bits are divided up
        bits: usize, // the number of bits on any given machine
    }

    // private auxiliary functions
    mod auxf {
        use crate::EdgeScale;

        // checks if the amount of bits available is enough for a given EdgeScale
        pub fn verify_partition_size(scale: &EdgeScale, bits: &usize) {
            if *scale == EdgeScale::U8 && *bits < 8 ||
                *scale == EdgeScale::U16 && *bits < 16 ||
                *scale == EdgeScale::U32 && *bits < 32 
            { panic!("Not enough bits for the given EdgeScale"); }
        }
        
    }
    
    impl<T> BitGraph<T> { 

        // Creates new BitGraph with no vertices
        pub fn new(scale: EdgeScale) -> BitGraph<T> {
            let b = std::mem::size_of::<usize>() * 8; // the amount of bits in any given machine
            auxf::verify_partition_size(&scale, &b); // checking for an overflow
            match scale { 
                EdgeScale::SAME   
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 1,
                            max_weight: 0,
                            partition: 1, 
                            bits: b  
                        }, 
                EdgeScale::BINARY 
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 2,
                            max_weight: 1, // 01
                            partition: 2, 
                            bits: b  
                        },  
                EdgeScale::U4     
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 4,
                            max_weight: 7, // 0111
                            partition: 4, 
                            bits: b  
                        },
                EdgeScale::U8     
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 8,
                            max_weight: 127, // 0111 1111
                            partition: 8, 
                            bits: b  
                        },
                EdgeScale::U16    
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 16,
                            max_weight: 32_767, // 0111 1111 1111 1111
                            partition: 16, 
                            bits: b 
                        },
                EdgeScale::U32    
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 32,
                            max_weight: 2_147_483_647, // 0111 1111 1111 1111 1111 1111 1111 1111
                            partition: 32, 
                            bits: b 
                        },
            }
        }
        // Same as new(), but with reserved space. It uses the same principal as Vec::with_capacity(_),
        // so the user is responsible for filling in the empty spaces (since they may not need/want to 
        // fill the entire space).
        pub fn new_with_capacity(scale: EdgeScale, capacity: usize) -> BitGraph<T> {
            let b = std::mem::size_of::<usize>() * 8;
            auxf::verify_partition_size(&scale, &b);
            match scale { 
                EdgeScale::SAME   
                    =>  BitGraph 
                        {  
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity), 
                            vert_bit_indexing: b / 1,
                            max_weight: 0,
                            partition: 1, 
                            bits: b  
                        },
                EdgeScale::BINARY 
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity), 
                            vert_bit_indexing: b / 2,
                            max_weight: 1, // 01
                            partition: 2, 
                            bits: b  
                        },
                EdgeScale::U4     
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity),
                            vert_bit_indexing: b / 4,
                            max_weight: 7, // 0111
                            partition: 4, 
                            bits: b  
                        },
                EdgeScale::U8     
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity), 
                            vert_bit_indexing: b / 8,
                            max_weight: 127, // 0111 1111
                            partition: 8, 
                            bits: b  
                        },
                EdgeScale::U16    
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity), 
                            vert_bit_indexing: b / 16,
                            max_weight: 32_767, // 0111 1111 1111 1111
                            partition: 16, 
                            bits: b 
                        },
                EdgeScale::U32     
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity), 
                            vert_bit_indexing: b / 32,
                            max_weight: 2_147_483_647, // 0111 1111 1111 1111 1111 1111 1111 1111
                            partition: 32, 
                            bits: b 
                        },
            }
        }

        pub fn get_partition_size(&self) -> usize { self.partition }

        // Returns the amount of vertices within a given BitGraph
        pub fn size(&self) -> usize { self.vertices.len() }

        // Returns the size of the edgevert at idx
        pub fn ev_len_at(&self, idx: usize) -> usize { self.vertices[idx].get_ev_size() }

        // simple add without a returned value
        pub fn add(&mut self, new_data: T) {

            // all new vertnums are assigned from 0, 1, 2, ..., to the last element added
            let new_vertnum: usize = self.vertices.len(); 
            // The amount of vertices within a single usize
            let vbi: usize = self.vert_bit_indexing; 

            /*
             *  Checking if the edgeverts need to be incremented. This is needed because
             *  edgverts can only hold as many numbers equal to the amount of bits in a 
             *  usize number.
            */
            if new_vertnum % vbi == 0 { 
                for i in 0..new_vertnum { self.vertices[i].push_new_ev(); }
            }

            // initializing new edgevert, THIS IS THE PROBLEM, I THINK
            let mut ev: Vec<usize> = Vec::with_capacity(new_vertnum / vbi + 1);
            for _ in 0..ev.capacity() { // filling pre-allocated array 
                ev.push(0); // 0x000...
            }

            // initializing new Vertex with new edgevert and new_data
            let v: Vertex<T> = Vertex 
                {
                    data: new_data,
                    vertnum: new_vertnum,
                    edgevert: ev
                }; 

            self.vertices.push(v);

        }

        // adds a vertex with a returned value (the vertnum of the newly added Vertex)
        pub fn addv(&mut self, data: T) -> usize {
            self.add(data); // uses the add method above
            return self.vertices.len() - 1;
        }

        // The connect_to() function found in Vertex<T> impl
        // pub fn connect_to(&mut self, bitnum: usize, weight: usize, bits_partition: usize, partition_size: usize) {
        pub fn connect(&mut self, source: usize, dest: usize, weight: usize) {
            if source < self.vertices.len() && dest < self.vertices.len() {
                if weight <= self.max_weight { 
                    // performing connection by calling a Vertex<T> function...
                    // self.vert_bit_indexing is needed for proper edgevert indexing for any given vertex
                    self.vertices[source].connect_to(dest, weight, self.vert_bit_indexing);
                } else {
                    panic!("wieght exceeds max wieght")
                }
            } else {
                panic!("out of bounds or connecting non-existent edges");
            }
        
        }

    }


}
