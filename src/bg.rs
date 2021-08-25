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
        // size of the Vector of type usize, edgevert
        pub fn get_ev_size(&self) -> usize { self.edgevert.len() }
        // retrieving the edgevert itself (maybe only use for debugging purposes)
        pub fn get_ev_num(&self, idx: usize) -> usize { self.edgevert[idx] } 

        /*
            connect_to: This function establishes a connection between a source and destination vertex.
            It is called from the bitgraphs connect() and try_connect() functions. Establishing the 
            source connection is the simplest part of the process, and does not need to directly 
            interact with a Vertex<T> type. It is therefore, not a necessary step here. The comments
            below will have detailed descriptions about the inner workings of the function...

            bitnum: The destination vertex.
            vbi: vert_bit_indexing from a given BitGraph<T>. Needed for indexing the correct edgevert. 
            
            In connect() impl of BitGraph<T>...
            self.vertices[source].connect_to(dest, weight, self.vert_bit_indexing, self.partition);
        
            self.edgevert[bitnum / vbi] ... EXPLAINED: The amount of edgeverts is proportional to the amount
            of bits on a maching, vertices in a given graph, and which EdgeScale has been choosen at the
            initialization of the graph. When there are not enough bits in a number to store a vertex and
            its weights, another edgevert will be needed. self.edgevert[bitnum / vbi] ensures that any bitnum
            (destination vertex) will find the right edgevert based on the amount of bits divided by the 
            partition size (also known as vert_bit_indexing in the BitGraph<T> struct) to establish a 
            connection for any vertnum from 0 to infinity.

        */        
        pub fn connect_to(&mut self, bitnum: usize, weight: usize, vbi: usize, partition_size: usize) {
            // How far the bit_pos will left shift scaled by 'partition_size'
            let bit_pos_scalar: usize = bitnum % vbi;
            // Initializing bit position for the bitnum
            let vert_bit_pos: usize = match partition_size {
                1 => 1 << bit_pos_scalar,
                _ => 1 << ((bit_pos_scalar + 1) * partition_size) - 1,
            };
            // Initializing bits for the 'weight' region
            let vert_weight_pos: usize = match partition_size {
                1 => 0,
                _ => weight << bit_pos_scalar * partition_size,
            };
            // Finalizing edgevert
            self.edgevert[bitnum / vbi] |= vert_bit_pos | vert_weight_pos;
        }

        pub fn disconnect_from(&mut self, bitnum: usize, vbi: usize, partition_size: usize) {
            // defining bit region based on the 'partition_size'
            // Example: U4 -> 0000 1111, BINARY -> 0000 0011, SAME -> 0000 0001
            let m1: usize = usize::MAX >> ((vbi * partition_size) - partition_size);
            // shifting bit-region (m1) to prepare the clearing of bits
            let m2: usize = match partition_size { 
                1 => m1 << bitnum,
                _ => m1 << ((bitnum * partition_size) - partition_size),
            };
            // clearing shifted bit-region (m2)
            self.edgevert[bitnum / vbi] &= !m2; 
        }

        pub fn dec_vn(&mut self) { self.vertnum -= 1; } // decrement vertnum

        // shifts all bits down by 1 partition
        // ADVICE: possibly replacing the last two args with vbi

        /*
            Notes: 
                - Loop does not always start at the beginning, edgevert[0]
                - The first edgevert may not have all bits shifted downwards
                - Bits may need to be shifted accompanied by a XOR
                - Analogy: Partition is like the area of an edgevert
                - minimum shifts is equal to partition_size
                - IMPORTANT: need to take 1st partition_size amount of 
                bits from the next edgevert and put in the last of the 
                current to fill the empty space created by the shift

            'Newer' Notes:
                - !(usize::MAX >> ocbp_1) and ev_mask are used to help oscillate between the
                different amount of bits. This is neccesary because
                once one section is changed of one edgevert, the
                next will need to be compensated. This concept may
                have some flaws of its own
        */
        pub fn shift_after_vertex(&mut self, vertex: usize, partition_size: usize, bits: usize) {
            
            let vbi: usize = bits / partition_size;
            let compd_vn_bit_pos: usize = (vertex % vbi) * partition_size;
            let ev_start: usize = vertex / vbi;
            let ev_end: usize = self.edgevert.len() - 1;
            // DEBUG statements part1...
            // saved data mask scalar
            // used for saving data after deleted vertex+weight
            let saved_data_mask: usize = self.edgevert[ev_start] & !(usize::MAX 
                    >> (bits - compd_vn_bit_pos - partition_size));
            // DEBUG statements part2...
            // println!("(bits - compd_vn_bit_pos) = {}", bits - compd_vn_bit_pos);
            // delete everything from vertex+weight and onward...
            // Not subtracting partition_size to delete selected vertex+weight 
            // '% bits' helps avoid a right shift overflow
            if  compd_vn_bit_pos != 0 { // to avoid overflow 
                self.edgevert[ev_start] &= usize::MAX >> (bits - compd_vn_bit_pos);
                // self.edgevert[ev_start] &= usize::MAX >> (bits - compd_vn_bit_pos);
                // insert and shift saved data to edgevert
                println!("saved_data_mask = {}", saved_data_mask);
                self.edgevert[ev_start] |= saved_data_mask >> partition_size;
            }
            if ev_start < ev_end {
                // The mask for acquiring bits from next edgevert
                let m1: usize = usize::MAX >> (bits - partition_size);
                // moving bits from next edgevert into the 'start' edgevert
                self.edgevert[ev_start] |= (self.edgevert[ev_start + 1] & m1) 
                  << compd_vn_bit_pos; 
                // All edgeverts till 'end': shift, replace, repeat...  
                for e in (ev_start + 1)..ev_end { 
                    self.edgevert[e] >>= partition_size;
                    self.edgevert[e] |= (self.edgevert[e + 1] & m1)
                        << compd_vn_bit_pos;
                }
                // a final shift at the end is needed without a replacement
                // since the end edgevert does not have another proceeding
                // edgevert to extract bits from.
                self.edgevert[ev_end] >>= partition_size;
            }
                      
        }
    }

    /*
        EdgeScale: This enum dictates how many bits are within each 
        edgevert (minus 1 for the vertex position). If SAME is chosen, 
        then there are no weighted edges between vertices. SAME is 
        useful for just establishing connections between vertices 
        without needing to specify a certain weight. Once an 
        EdgeScale has been chosen, it cannot be changed.
        
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

        pub fn check_bounds(source: &usize, dest: &usize, vert_len: usize) {
            let source_is_greater: bool = *source >= vert_len;
            let dest_is_greater: bool = *dest >= vert_len; 
            if  source_is_greater || dest_is_greater {
                if source_is_greater && dest_is_greater {
                    panic!("ERROR: check_bounds() -> source and destination are out of bounds");
                } else {
                    if source_is_greater { 
                        panic!("ERROR: check_bounds() -> source is greater"); 
                    } else { // dest_is_greater
                        panic!("ERROR: check_bounds() -> destination is greater"); 
                    }
                }
            }
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

        // adds copies of the same type of 'data' a specified amount of 'times'
        pub fn add_copies(&mut self, data: T, times: usize) where T : Clone {
            for v in 0..times { self.add(data.clone()); }
        }

        // The connect_to() function found in Vertex<T> impl
        // pub fn connect_to(&mut self, bitnum: usize, weight: usize, bits_partition: usize, partition_size: usize) {
        pub fn connect(&mut self, source: usize, dest: usize, weight: usize) {
            auxf::check_bounds(&source, &dest, self.vertices.len());
            if weight <= self.max_weight { 
                // performing connection by calling a Vertex<T> function...
                // self.vert_bit_indexing is needed for proper edgevert indexing for any given vertex
                self.vertices[source].connect_to(dest, weight, self.vert_bit_indexing, self.partition);
            } else {
                panic!("ERROR: from connect() -> wieght exceeds max wieght")
            } 
        }

        pub fn disconnect(&mut self, source: usize, dest: usize) {
            auxf::check_bounds(&source, &dest, self.vertices.len());            
            self.vertices[source].disconnect_from(dest, self.vert_bit_indexing, self.partition);
        }

        pub fn ev_num_at(&self, vert_idx: usize, ev_idx: usize) -> usize { 
            self.vertices[vert_idx].get_ev_num(ev_idx)
        }

        pub fn is_connected(&self, source: usize, dest: usize) -> bool {
            auxf::check_bounds(&source, &dest, self.vertices.len());
            let bit_pos_scalar: usize = dest % self.vert_bit_indexing;
            // retrieving edgevert and isolating the appropriate bit
            match self.partition {
                1 => self.vertices[source].get_ev_num(dest / self.vert_bit_indexing)
                    & (1 << bit_pos_scalar) != 0,
                _ => self.vertices[source].get_ev_num(dest / self.vert_bit_indexing)
                    & (1 << ((bit_pos_scalar + 1) * self.partition) - 1) != 0,
            }
        }
        /*
            remove: most work is done in shift_after_vertex(). Every vertices vertnum
            is decreased after the given vertex because once the vertex is removed,
            empty spaces will be created, leaving wasted space. Overall, this will be
            the most expensive function. shift_after_vertex() removes all possible 
            connections with 'vertex'. There are 2 loops to reduce redundant 
            conditionals.
        */
        pub fn remove(&mut self, vertex: usize) {
            let mut len: usize = self.vertices.len();
            if vertex >= len {
                panic!("cannot remove non-existent element");
            }

            for v in 0..vertex { // pre vertex work
                // Debug
                println!("v1 = {}", v);
                self.vertices[v].shift_after_vertex(vertex, self.partition, self.bits);
            }
            self.vertices.remove(vertex); // Finally, removing the vertex
            // This loop does not happen if vertex == (self.vertices.len() - 1)
            len -= 1;
            // fails here for complex_remove2
            for v in vertex..len { // post vertex work
                // Debug
                println!("v2 = {}", v);
                self.vertices[v].dec_vn();
                self.vertices[v].shift_after_vertex(vertex, self.partition, self.bits);
            }
            //self.vertices.remove(vertex); // Finally, removing the vertex
        }
    }

}
