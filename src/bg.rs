/*
    BitGraph Description: BitGraph is a graph (directed) data structure that uses usize 
    values to store edge wieghts and their corresponding vertex.   
*/
#![allow(non_snake_case)]

pub mod bit_graph {

    #[derive(Debug)]
    struct Vertex<T> {  
        data: T,
        vertnum: usize, // the 'source' vertex number
        edgevert: Vec<usize> // the 'destination' vertex number(s) and their weight(s)
    } 

    impl<T> Vertex<T> {

        // used when more edgeverts are needed to initialize potential connections
        pub fn push_new_ev(&mut self) { self.edgevert.push(0); } // 0x000.....

        pub fn get_ev_size(&self) -> usize { self.edgevert.len() }
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
    #[derive(PartialEq, Debug)]
    pub enum EdgeScale {
        SAME, BINARY, U4, U8, U16, U32
    }

    // Used only when the user does not want to store any data inside a Vertex
    // pub struct NoData;

    #[derive(Debug)]
    pub struct BitGraph<T> {
        vertices: Vec<Vertex<T>>,
        vert_bit_indexing: usize,
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
            let b = std::mem::size_of::<usize>() * 8;
            auxf::verify_partition_size(&scale, &b);
            match scale { 
                SAME   
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 1,
                            partition: 1, 
                            bits: b  
                        }, 
                BINARY 
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 2,
                            partition: 2, 
                            bits: b  
                        },  
                U4     
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 4,
                            partition: 4, 
                            bits: b  
                        },
                U8     
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 8,
                            partition: 8, 
                            bits: b  
                        },
                U16    
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 16,
                            partition: 16, 
                            bits: b 
                        },
                U32    
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::new(),
                            vert_bit_indexing: b / 32, 
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
                SAME   
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity), 
                            vert_bit_indexing: b / 1,
                            partition: 1, 
                            bits: b  
                        },
                BINARY 
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity), 
                            vert_bit_indexing: b / 2,
                            partition: 2, 
                            bits: b  
                        },
                U4     
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity),
                            vert_bit_indexing: b / 4,
                            partition: 4, 
                            bits: b  
                        },
                U8     
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity), 
                            vert_bit_indexing: b / 8,
                            partition: 8, 
                            bits: b  
                        },
                U16    
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity), 
                            vert_bit_indexing: b / 16,
                            partition: 16, 
                            bits: b 
                        },
                U32     
                    =>  BitGraph 
                        { 
                            vertices: Vec::<Vertex<T>>::with_capacity(capacity), 
                            vert_bit_indexing: b / 32,
                            partition: 32, 
                            bits: b 
                        },
            }
        }

        pub fn size(&self) -> usize { self.vertices.len() }

        pub fn ev_len_at(&self, idx: usize) -> usize { self.vertices[idx].get_ev_size() }

        // simple add without a returned value
        pub fn add(&mut self, new_data: T) {

            // also used for index of new vertex
            let length: usize = self.vertices.len(); 

            // The amount of vertices within a single usize
            let vbi: usize = self.vert_bit_indexing; 
            
            // initializing new edgevert, THIS IS THE PROBLEM, I THINK
            let mut ev: Vec<usize> = Vec::with_capacity(length / vbi + 1);
            for _ in 0..ev.capacity() { // filling pre-allocated array 
                ev.push(0); // 0x000...
            }

            // initializing new Vertex with new edgevert and new_data
            let v: Vertex<T> = Vertex 
                {
                    data: new_data,
                    vertnum: length,
                    edgevert: ev
                }; 

            self.vertices.push(v);
            /*
                Checking if the edgeverts need to be incremented. This is needed because
                edgverts can only hold as many numbers equal to the amount of bits in a 
                usize number.
            */
            if length != 0 && length % vbi == 0 { // MAJOR FLAW HERE
                for i in 0..length { self.vertices[i].push_new_ev(); }
            }
        }

        // adds a vertex with a returned value (the vertnum of the newly added Vertex)
        pub fn addv(&mut self, data: T) -> usize {
            self.add(data); // uses the add method above
            return self.vertices.len() - 1;
        }


    }


}
