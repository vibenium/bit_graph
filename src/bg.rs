/*
    BitGraph Description: BitGraph is a graph (directed) data structure 
    that uses usize values to store edge wieghts and their corresponding 
    outgoing vertex (based on the index of the array, a.k.a, vertnum).
*/
pub mod bit_graph {

    #[derive(Debug, Clone)]
    struct Vertex<T> { // For storing vertex data/metadata
        data: T, // Optional data inside
        vertnum: usize,       // the 'source' vertex number
        edgevert: Vec<usize>, // the 'dest' vertex number(s) and their weight(s)
    }

    // None of the functionality is called directly from a type Vertex object.
    // Instead, the BitGraph containing the vertices will do what is needed.
    impl<T> Vertex<T> {
        
        /// returns data of the vertex
        pub fn get_vert_data(&self) -> T where  T: Clone, {
            self.data.clone()
        }
        /// used when more edgeverts are needed to initialize potential connections
        pub fn push_new_ev(&mut self) {
            self.edgevert.push(0); // 0x000.....
        }
        /// size of the Vector of type usize, edgevert
        pub fn get_ev_size(&self) -> usize {
            self.edgevert.len()
        }
        /// retrieving the edgevert itself (useful for debugging)
        pub fn get_ev_num(&self, idx: usize) -> usize {
            self.edgevert[idx]
        }
        /// useful for refreshing the # of edgeverts to keep things symmetric
        pub fn dec_ev(&mut self) {
            self.edgevert.pop();
        }
        /// simply returns the vertnum of a given edgevert (useful for debugging)
        pub fn get_vertnum(&self) -> usize {
            self.vertnum
        }
        /// Connects an the current vertex to the vertex based on 'bitnum'
        pub fn connect_to(&mut self, bitnum: usize, weight: usize, 
                          vbi: usize, 
                          partition_size: usize) {
            // How far the bit_pos will left shift scaled by 'partition_size'
            let bit_pos_scalar: usize = bitnum % vbi;
            // Initializing bit position for the bitnum
            let vert_bit_pos: usize = match partition_size {
                1 => 1 << bit_pos_scalar,
                _ => 1 << ((bit_pos_scalar + 1) * partition_size) - 1,
            };
            // Initializing bits for the 'weight region'
            let vert_weight_pos: usize = match partition_size {
                1 => 0, // EdgeScale::SAME means no weight
                _ => weight << bit_pos_scalar * partition_size,
            };
            // Finalizing edgevert
            self.edgevert[bitnum / vbi] |= vert_bit_pos | vert_weight_pos;
        }

        /// flips the bits associated with the vertex index in the BitGraph
        /// and its weight (i.e., bit chunks flipped to ...0000...)
        pub fn disconnect_from(&mut self, bitnum: usize, vbi: usize, 
                               partition_size: usize,
                               bits: usize) {
            // creating block size for removal
            let m1: usize = usize::MAX >> (bits - partition_size);
            // shifting block where the vertnum (and possibly wieght) exits
            let m2: usize = m1 << ((bitnum * partition_size) % bits);
            // flip m2 to preserve all bits except the region m2 occupies
            self.edgevert[bitnum / vbi] &= !m2;
        }
        /// decrements the vertnum (needed for vertex removal method
        /// in the BitGraph struct impl
        pub fn dec_vn(&mut self) {
            self.vertnum -= 1;
        }

        /* Notes on shift_after_vertex:
         * This is probably the most involved function in the entire bg.rs file.
         * If there is a bug due to using BitGraph::remove(), it is probably
         * because of this function.
         *
         * Its primary purpose is to shift the bits in every edgevert from the
         * vertex(self) after a vertex removal invoked by BitGraph::remove(). 
         */

        /// carefully shifts edgevert bits down by 1 partition size
        pub fn shift_after_vertex(&mut self, vertex: usize, 
                                  partition_size: usize, bits: usize) {
            // vbi => how the 'vertex' is compressed via (%)
            let vbi: usize = bits / partition_size;
            // compd_vn_bit_pos => bit position in an edgevert
            let compd_vn_bit_pos: usize = (vertex % vbi) * partition_size;
            // ev_start => edgevert start index (Not always 0!)
            let ev_start: usize = vertex / vbi;
            // ev_end => edgevert end index (must always be 0 because
            // all bits must be shifted after a removal).
            let ev_end: usize = self.edgevert.len() - 1;
            // saved_data_mask => extract bits excluding 'vertex', its
            // associated weight, and all other bits before it in the
            // edgevert at ev_start.
            let saved_data_mask: usize = self.edgevert[ev_start]
                & !(usize::MAX >> (bits - compd_vn_bit_pos - partition_size));
            
            /* Shifting bits in edgevert[ev_start] is slightly different than
             * the rest of shifts below, especially if the destination vertex
             * is not represented as the last bits (vertnum+weight) in 
             * edgevert[ev_start]. The rest of the shifts are just a matter of
             * shifting 1 partition size to the right, and taking one partition
             * sized chunk from the next edgevert to fill the void. This method
             * preserves the linear nature of the BitGraph in terms of how 
             * vertices are stored, and how they are connected.
             */  
            if compd_vn_bit_pos != 0 { // to avoid right shift overflow
                
                // Not subtracting partition_size to delete selected 
                // vertex+weight region
                self.edgevert[ev_start] &= usize::MAX >> (bits - compd_vn_bit_pos);
                // self.edgevert[ev_start] &= usize::MAX >> (bits - compd_vn_bit_pos);
                // insert and shift saved data to edgevert
                self.edgevert[ev_start] |= saved_data_mask >> partition_size;
            }
            /* Shift the remaining edgeverts down by 1 partition size. 
             * In theory, this should not take have too many iterations
             * depending on how many vertices there are, and the partition
             * size. A greater partition size paired with many veritces will
             * make this operation very expensive. If this is the case, then
             * using a BitGraph data structure may not be the best solution
             * for the user wanting to construct a graph.
            */ 
            if ev_start < ev_end {
                // The mask for acquiring bits from next edgevert
                let m1: usize = usize::MAX >> (bits - partition_size);
                // moving bits from next edgevert into the 'start' edgevert
                self.edgevert[ev_start] |= (self.edgevert[ev_start + 1] & m1) << compd_vn_bit_pos;
                // All edgeverts till 'ev_end' => shift, replace, repeat...
                for e in (ev_start + 1)..ev_end {
                    self.edgevert[e] >>= partition_size;
                    self.edgevert[e] |= (self.edgevert[e + 1] & m1) << compd_vn_bit_pos;
                }
                // A final shift at the end is needed without a replacement
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
        without needing to specify a certain weight. 
    */
    #[derive(PartialEq, Debug, Clone)]
    pub enum EdgeScale {
        SAME,
        BINARY,
        U4,
        U8,
        U16,
        U32,
    }

    // Used only when the user does not want to store any data inside a Vertex
    // pub struct NoData;

    #[derive(Debug, Clone)]
    pub struct BitGraph<T> {
        vertices: Vec<Vertex<T>>,
        vert_bit_indexing: usize,
        max_weight: usize, // The max weight an edge can have
        partition: usize,  // how the bits are divided up
        bits: usize,       // the number of bits on any given machine
    }

    // private auxiliary functions
    mod auxf {
        use crate::BitGraph;
        use crate::EdgeScale;
        // checks if the amount of bits available is enough for a given EdgeScale
        // WARNING: THIS HAS NOT BEEN rigorously tested...
        pub fn verify_partition_size(scale: &EdgeScale, bits: &usize) {
            if *scale == EdgeScale::U8 && *bits < 8
                || *scale == EdgeScale::U16 && *bits < 16
                || *scale == EdgeScale::U32 && *bits < 32
            {
                panic!("Not enough bits for the given EdgeScale");
            }
        }

        pub fn check_bounds(source: &usize, dest: &usize, vert_len: usize) {
            let source_is_greater: bool = *source >= vert_len;
            let dest_is_greater: bool = *dest >= vert_len;
            if source_is_greater || dest_is_greater {
                if source_is_greater && dest_is_greater {
                    panic!("ERROR: check_bounds() -> source and destination are out of bounds");
                } else {
                    if source_is_greater {
                        panic!("ERROR: check_bounds() -> source is greater");
                    } else {
                        // dest_is_greater
                        panic!("ERROR: check_bounds() -> destination is greater");
                    }
                }
            }
        }
        // Helper for type_connect and type_disconnect
        pub fn find_src_dest_idx<T: std::clone::Clone + std::cmp::PartialEq>(
            graph: &mut BitGraph<T>,
            source: &T,
            dest: &T,
        ) -> [(bool, usize); 2] {
            // return example: [(false, 0), (true, 1)]

            let len: usize = graph.vertices.len();
            let mut found_source: bool = false;
            let mut found_dest: bool = false;
            let mut source_idx: usize = 0;
            let mut dest_idx: usize = 0;
            let mut _v: usize = 0;

            // look and assign...
            while (!found_source || !found_dest) && _v < len {
                if !found_source && *source == graph.vertices[_v].get_vert_data() {
                    found_source = true;
                    source_idx = _v;
                }
                if !found_dest && *dest == graph.vertices[_v].get_vert_data() {
                    found_dest = true;
                    dest_idx = _v;
                }
                _v += 1;
            }
            // return ...
            [(found_source, source_idx), (found_dest, dest_idx)]
        }
    }

    impl<T: std::cmp::PartialEq + std::clone::Clone> BitGraph<T> {
        // Creates new BitGraph with no vertices
        pub fn new(scale: EdgeScale) -> BitGraph<T> {
            let b = std::mem::size_of::<usize>() * 8; // the amount of bits in any given machine
            auxf::verify_partition_size(&scale, &b); // checking for an overflow
            match scale {
                EdgeScale::SAME => BitGraph {
                    vertices: Vec::<Vertex<T>>::new(),
                    vert_bit_indexing: b / 1,
                    max_weight: 0,
                    partition: 1,
                    bits: b,
                },
                EdgeScale::BINARY => BitGraph {
                    vertices: Vec::<Vertex<T>>::new(),
                    vert_bit_indexing: b / 2,
                    max_weight: 1, // 01
                    partition: 2,
                    bits: b,
                },
                EdgeScale::U4 => BitGraph {
                    vertices: Vec::<Vertex<T>>::new(),
                    vert_bit_indexing: b / 4,
                    max_weight: 7, // 0111
                    partition: 4,
                    bits: b,
                },
                EdgeScale::U8 => BitGraph {
                    vertices: Vec::<Vertex<T>>::new(),
                    vert_bit_indexing: b / 8,
                    max_weight: 127, // 0111 1111
                    partition: 8,
                    bits: b,
                },
                EdgeScale::U16 => BitGraph {
                    vertices: Vec::<Vertex<T>>::new(),
                    vert_bit_indexing: b / 16,
                    max_weight: 32_767, // 0111 1111 1111 1111
                    partition: 16,
                    bits: b,
                },
                EdgeScale::U32 => BitGraph {
                    vertices: Vec::<Vertex<T>>::new(),
                    vert_bit_indexing: b / 32,
                    max_weight: 2_147_483_647, // 0111 1111 1111 1111 1111 1111 1111 1111
                    partition: 32,
                    bits: b,
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
                EdgeScale::SAME => BitGraph {
                    vertices: Vec::<Vertex<T>>::with_capacity(capacity),
                    vert_bit_indexing: b / 1,
                    max_weight: 0,
                    partition: 1,
                    bits: b,
                },
                EdgeScale::BINARY => BitGraph {
                    vertices: Vec::<Vertex<T>>::with_capacity(capacity),
                    vert_bit_indexing: b / 2,
                    max_weight: 1, // 01
                    partition: 2,
                    bits: b,
                },
                EdgeScale::U4 => BitGraph {
                    vertices: Vec::<Vertex<T>>::with_capacity(capacity),
                    vert_bit_indexing: b / 4,
                    max_weight: 7, // 0111
                    partition: 4,
                    bits: b,
                },
                EdgeScale::U8 => BitGraph {
                    vertices: Vec::<Vertex<T>>::with_capacity(capacity),
                    vert_bit_indexing: b / 8,
                    max_weight: 127, // 0111 1111
                    partition: 8,
                    bits: b,
                },
                EdgeScale::U16 => BitGraph {
                    vertices: Vec::<Vertex<T>>::with_capacity(capacity),
                    vert_bit_indexing: b / 16,
                    max_weight: 32_767, // 0111 1111 1111 1111
                    partition: 16,
                    bits: b,
                },
                EdgeScale::U32 => BitGraph {
                    vertices: Vec::<Vertex<T>>::with_capacity(capacity),
                    vert_bit_indexing: b / 32,
                    max_weight: 2_147_483_647, // 0111 1111 1111 1111 1111 1111 1111 1111
                    partition: 32,
                    bits: b,
                },
            }
        }

        /// Create a new BitGraph<T> with existing data
        pub fn initialize(scale: EdgeScale, d: &[T]) 
            -> BitGraph<T> where T: Clone {

            let len: usize = d.len();
            let mut verts: Vec<Vertex<T>> = Vec::<Vertex<T>>::with_capacity(len); 
            
            let b = std::mem::size_of::<usize>() * 8;
            auxf::verify_partition_size(&scale, &b);
            
            // initialize BitGraph meta data
            // (vert_bit_indexing, max_weight, partition)
            let (vbi, mw, p): (usize, usize, usize) = match scale {
                EdgeScale::SAME => (b / 1, 0, 1),
                EdgeScale::BINARY => (b / 2, 1, 2),
                EdgeScale::U4 => (b / 4, 7, 4),
                EdgeScale::U8 => (b / 8, 127, 8),
                EdgeScale::U16 => (b / 16, 32_767, 16),
                EdgeScale::U32 => (b / 32, 2_147_483_647, 32),
            }; 
            let edgevert_len: usize = (len * p) / b + 1; 
            // Initializiazing all Vertex<T> info...
            for v in 0..len {
                verts.push(
                    Vertex {
                        data: d[v].clone(),
                        vertnum: v,
                        edgevert: vec![0; edgevert_len], // fill with 0's
                    }
                );
            }
            // Return the new BitGraph
            BitGraph {
                vertices: verts,
                vert_bit_indexing: vbi,
                max_weight: mw,
                partition: p,
                bits: b,
            }
            
        }

        pub fn get_data(&self, vert_idx: usize) -> T
        where
            T: Clone,
        {
            self.vertices[vert_idx].get_vert_data()
        }

        // For debugging/testing
        pub fn get_vn(&self, vert_idx: usize) -> usize {
            self.vertices[vert_idx].get_vertnum()
        }

        pub fn get_partition_size(&self) -> usize {
            self.partition
        }

        // Returns the amount of vertices within a given BitGraph
        pub fn size(&self) -> usize {
            self.vertices.len()
        }

        // Returns the size of the edgevert at idx
        pub fn ev_len_at(&self, idx: usize) -> usize {
            self.vertices[idx].get_ev_size()
        }

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
                for i in 0..new_vertnum {
                    self.vertices[i].push_new_ev();
                }
            }

            // initializing new edgevert, THIS IS THE PROBLEM, I THINK
            let mut ev: Vec<usize> = Vec::with_capacity(new_vertnum / vbi + 1);
            for _ in 0..ev.capacity() {
                // filling pre-allocated array
                ev.push(0); // 0x000...
            }

            // initializing new Vertex with new edgevert and new_data
            let v: Vertex<T> = Vertex {
                data: new_data,
                vertnum: new_vertnum,
                edgevert: ev,
            };

            self.vertices.push(v);
        }

        // adds a vertex with a returned value (the vertnum of the newly added Vertex)
        pub fn addv(&mut self, data: T) -> usize {
            self.add(data); // uses the add method above
            return self.vertices.len() - 1;
        }

        // adds copies of the same type of 'data' a specified amount of 'times'
        pub fn add_copies(&mut self, data: T, times: usize)
        where
            T: Clone,
        {
            for v in 0..times {
                self.add(data.clone());
            }
        }

        // The connect_to() function found in Vertex<T> impl
        // pub fn connect_to(&mut self, bitnum: usize, weight: usize, bits_partition: usize, partition_size: usize) {
        pub fn connect(&mut self, source: usize, dest: usize, weight: usize) {
            auxf::check_bounds(&source, &dest, self.vertices.len());
            if weight <= self.max_weight {
                // performing connection by calling a Vertex<T> function...
                // self.vert_bit_indexing is needed for proper edgevert indexing for any given vertex
                self.vertices[source].connect_to(
                    dest,
                    weight,
                    self.vert_bit_indexing,
                    self.partition,
                );
            } else {
                panic!("ERROR: from connect() -> wieght exceeds max wieght")
            }
        }

        pub fn disconnect(&mut self, source: usize, dest: usize) {
            auxf::check_bounds(&source, &dest, self.vertices.len());
            self.vertices[source].disconnect_from(
                dest,
                self.vert_bit_indexing,
                self.partition,
                self.bits,
            );
        }

        // Connect by type. Runtime: Worst=O(n); Best=O(1)
        pub fn type_connect(&mut self, source: &T, dest: &T, weight: usize) {
            // [(found_source, source_idx), (found_dest, dest_idx)]
            let result: [(bool, usize); 2] = auxf::find_src_dest_idx(self, source, dest);
            if result[0].0 && result[1].0 {
                self.vertices[result[0].1].connect_to(
                    result[1].1,
                    weight,
                    self.vert_bit_indexing,
                    self.partition,
                );
            } else {
                panic!("ERROR in type_connect: invalid source or destination");
            }
        }

        // Disconnect by type Runtime: Worst=O(n); Best=O(1)
        pub fn type_disconnect(&mut self, source: &T, dest: &T) {
            // [(found_source, source_idx), (found_dest, dest_idx)]
            let result: [(bool, usize); 2] = auxf::find_src_dest_idx(self, source, dest);
            if result[0].0 && result[1].0 {
                self.vertices[result[0].1].disconnect_from(
                    result[1].1,
                    self.vert_bit_indexing,
                    self.partition,
                    self.bits,
                );
            } else {
                panic!("ERROR in type_connect: invalid source or destination");
            }
        }

        pub fn ev_num_at(&self, vert_idx: usize, ev_idx: usize) -> usize {
            self.vertices[vert_idx].get_ev_num(ev_idx)
        }

        pub fn is_connected(&self, source: usize, dest: usize) -> bool {
            auxf::check_bounds(&source, &dest, self.vertices.len());
            let bit_pos_scalar: usize = dest % self.vert_bit_indexing;
            // retrieving edgevert and isolating the appropriate bit
            match self.partition {
                1 => {
                    self.vertices[source].get_ev_num(dest / self.vert_bit_indexing)
                        & (1 << bit_pos_scalar)
                        != 0
                }
                _ => {
                    self.vertices[source].get_ev_num(dest / self.vert_bit_indexing)
                        & (1 << ((bit_pos_scalar + 1) * self.partition) - 1)
                        != 0
                }
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
            // For when the other edgeverts are not needed
            // It helps create symmetry for the graph.
            let needs_ev_dec = match (len - 1) % self.vert_bit_indexing {
                0 => true,
                _ => false,
            };
            for v in 0..vertex {
                // pre vertex work
                // Debug
                println!("v1 = {}", v);
                self.vertices[v].shift_after_vertex(vertex, self.partition, self.bits);
                if needs_ev_dec {
                    self.vertices[v].dec_ev();
                }
            }
            self.vertices.remove(vertex); // Finally, removing the vertex
                                          // This loop does not happen if vertex == (self.vertices.len() - 1)
            len -= 1;

            for v in vertex..len {
                // post vertex work
                // Debug
                println!("v2 = {}", v);
                self.vertices[v].dec_vn();
                self.vertices[v].shift_after_vertex(vertex, self.partition, self.bits);
                if needs_ev_dec {
                    self.vertices[v].dec_ev();
                }
            }
        }
        // ROUGH IDEA... NOT BEING TESTED
        pub fn type_remove(&mut self, data: &T) {
            let len: usize = self.vertices.len();
            let mut found_data: bool = false;
            let mut vertex: usize = 0;

            // Search for data
            while !found_data && vertex < len {
                if self.vertices[vertex].get_vert_data() == *data {
                    found_data = true;
                } else {
                    // captures the data index
                    vertex += 1;
                }
            }

            if found_data {
                // Proceed with removal of 'vertex'
                for v in vertex..len {
                    self.vertices[v].dec_vn();
                    self.vertices[v].shift_after_vertex(vertex, self.partition, self.bits);
                }
                self.vertices.remove(vertex);
            } else {
                panic!("ERROR in type_remove: data does not exist");
            }
        }
    }
}
