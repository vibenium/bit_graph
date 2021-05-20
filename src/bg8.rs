#[derive(Debug, Clone)]
pub struct Vertex8 {
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

    pub fn connect(&mut self, bitnum: u8, ev_idx: usize) {
        // Bellow is an example of edgevert from the perspective
        // of bits and the corresponding vertices
        // bitnum =>    8 6 4 2                  8 6 4 2 
        // edgevert[1]: 01100100    edgevert[0]: 11000000
        //              7w6w5w4w                 3w2w1w0w
        self.edgevert[ev_idx] |= (1 << (bitnum - 1)) | (1 << (bitnum - 2))    
    }
}
 
#[derive(Debug)]
pub struct BitGraph8 {
    // WARNING: cannot exceed 255 elements
    vertices: Vec<Vertex8>,
}

impl BitGraph8 {
    pub fn new() -> BitGraph8 { BitGraph8 {vertices: Vec::new() } }
    pub fn size(&self) -> usize {self.vertices.len()}
    // gets the vertnum from the corresponding Vertex8 
    pub fn getv(&self, idx: usize) -> u8 { self.vertices[idx].getvn() }
    // gets a Vertex8
    pub fn getv8(&self, idx: usize) -> Vertex8 { self.vertices[idx].clone() }

    pub fn connect(&mut self, from: u8, to: u8) { 
        // check if both indices are in bound
        if usize::from(from) < self.vertices.len() && usize::from(to) < self.vertices.len() {
            // bitnum =>    8 6 4 2                  8 6 4 2 
            // edgevert[1]: 01100100    edgevert[0]: 11000000
            //              7w6w5w4w                 3w2w1w0w
            // 'from' is a vertnum and 'to' is the bit position
            let bitnum: u8 = (to % 4) * 2 + 2; // bit position in edgevert
            let ev_idx: usize = to as usize / 4; // the array to use 
            self.vertices[from as usize].connect(bitnum, ev_idx);
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
            for _ in 0..ev.capacity() { ev.push(0x00); }
            let v8: Vertex8 = Vertex8::new(_i, ev);
            self.vertices.push(v8); 
        
        } else {
            panic!("Error: BitGraph8 out of bounds! exceeded 255 elements");
        }
    }

}