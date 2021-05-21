/*
    Notes:
        - There is no purpose for having a weight at all of '1' between each bit.
        The reason for this is that if there is only a weight of 1 per node, 
        which means that all nodes will be the EXACT SAME LENGTH from eachother,
        then there is no reason to record the weigths at all. This will free up the 
        other 4 bits per u8. 
*/


pub mod bit_graph8 {    
    #[derive(Debug, Clone)]
    struct Vertex8 {
        vertnum: u8,
        edgevert: Vec<u8>,
    }

    impl Vertex8 {
        pub fn new(vertnum: u8, edgevert: Vec<u8>) -> Vertex8 {
            Vertex8 {vertnum, edgevert} 
        }
        pub fn getvn(&self) -> u8 { self.vertnum }
    
        // adds new edgevert number (0x00 since no connections made)
        pub fn push_new_ev(&mut self) { self.edgevert.push(0x00) }

        pub fn get_ev_size(&self) -> usize { self.edgevert.len() }

        pub fn getev(&self, idx: usize) -> u8 { 
            if idx < self.edgevert.len() {
                self.edgevert[idx] 
            } else { 
                panic!("getev out of bounds"); 
            }
        }

        pub fn connect_to(&mut self, bitnum: usize) {
            self.edgevert[bitnum / 8] |= 1 << (bitnum % 8);    
        }
    }
    
    #[derive(Debug)]
    pub struct BitGraph8 {
        // WARNING: cannot exceed 255 elements
        vertices: Vec<Vertex8>,
    }

    impl BitGraph8 {
        
        pub fn new() -> BitGraph8 { BitGraph8 { vertices: Vec::new() } }
        pub fn size(&self) -> usize {self.vertices.len()}
        
        // gets the vertnum from the corresponding Vertex8 
        pub fn getv(&self, idx: usize) -> u8 { self.vertices[idx].getvn() }
        
        // Only going out one 
        pub fn is_connected(&self, source: usize, dest: usize) -> bool {
            if source < self.vertices.len() && dest < self.vertices.len() {                
                (self.vertices[source].getev(dest / 8) & (1 << (dest % 8))) != 0
            } else { 
                false 
            }
        }

        // gets a Vertex8 (ONLY FOR DEBUGGING)
        // pub fn getv8(&self, idx: usize) -> Vertex8 { self.vertices[idx].clone() }

        

        pub fn connect(&mut self, source: usize, dest: usize) {
            if source < self.vertices.len() && dest < self.vertices.len() {
                self.vertices[source].connect_to(dest);
            } else {
                panic!("out of bounds or connecting non-existent edges");
            }
        }

        // Adds new vertex to BitGraph8's vertices.
        // Each vertnum starts from 0 up to 255.
        pub fn addv(&mut self) {
            if self.vertices.len() <= 255 {
                let _i: u8 = self.vertices.len() as u8;
                // Checks if all other vertices edge verts need to be updated (via new_ev).
                if _i == 0 || _i % 8 == 0 { 
                    // adding extra edgevert forall vertives
                    for x in 0.._i {
                        self.vertices[x as usize].push_new_ev();
                    }
                }
                // init capacity for new vec for Vertex8
                let mut ev: Vec<u8> = Vec::with_capacity(_i as usize / 8 + 1);
                for _ in 0..ev.capacity() { // filling pre-allocated array 
                    ev.push(0x00); 
                }
                let v8: Vertex8 = Vertex8::new(_i, ev);
                self.vertices.push(v8); 
            } else {
                panic!("Error: BitGraph8 out of bounds! exceeded 255 elements");
            }
        }

    }
}