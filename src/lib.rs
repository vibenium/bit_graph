// mod itb;
mod bg;
use crate::bg::bit_graph::*;
use crate::BitGraph;
use std::thread;
use std::thread::JoinHandle;

#[cfg(test)]
mod tests {
    /*
        COMPLEX VS SIMPLE ...
        complex: USUALLY dealing with edgeverts.len() >= 2
        simple:a USUALLY dealing with edgeverts.len() == 1

        terminology: vX and evY are ways to define an edgevert number
        where 'X' is the index of the 'vertex' and 'Y' is the
        index of the 'edgevert'. For example, v0 and ev2 means get
        the edgevert of vertex[0] at edgevert[2].
    */

    use super::*;

    #[derive(PartialEq, Clone)]
    struct NoData;

    const BITS: usize = std::mem::size_of::<usize>() * 8;

    #[test]
    fn simple_type_remove1() {
    
    }

    #[test] // FAILS for unknown reason(s)...
    fn simple_type_connect1() {
        let mut i8_graph: BitGraph<i8> = BitGraph::new_with_capacity(EdgeScale::U8, 2);
        let num1: i8 = 20;
        let num2: i8 = 4;
        i8_graph.add(num1);
        i8_graph.add(num2);

        // sanity check
        assert_eq!(i8_graph.get_data(0), 20);
        assert_eq!(i8_graph.get_data(1), 4);
        
        i8_graph.type_connect(&num1, &num2, 7);
        
        let mut street_graph: BitGraph<String> = BitGraph::new_with_capacity(EdgeScale::U8, 2);
        let street1 = String::from("Main st.");
        let street2 = String::from("Elm st.");
        street_graph.add(street1.clone());
        street_graph.add(street2.clone());
        
        street_graph.type_connect(&street1, &street2, 7); 
        assert!(street_graph.is_connected(0, 1));
   }

    #[test]
    fn simple_type_disconnect1() {
        #[derive(Clone, PartialEq)]
        struct Car<'a> {
            year: u16,
            model: &'a str,
        }
        let mut dealership: BitGraph<Car> = BitGraph::new_with_capacity(EdgeScale::U16, 2);
        let ford1: Car = Car{year: 2012, model: "Ford Focus"};
        let ford2: Car = Car{year: 2014, model: "Ford Focus"};
        dealership.add(ford1.clone());
        dealership.add(ford2.clone());
        dealership.type_connect(&ford1, &ford2, 5_000);

        assert!(dealership.is_connected(0, 1));
        dealership.type_disconnect(&ford1, &ford2);
        assert!(!dealership.is_connected(0, 1));
    }

    #[test]
    fn simple_get_data1() {
        
        let mut animals: BitGraph<&str> = BitGraph::new_with_capacity(EdgeScale::U8, 4);
        animals.add("Dog");
        animals.add("Cat");
        animals.add("Rat");
        animals.add("Lizard");
    
        assert_eq!(animals.size(), 4);

        assert_eq!(animals.get_data(0), "Dog");
        assert_eq!(animals.get_data(1), "Cat");
        assert_eq!(animals.get_data(2), "Rat");
        assert_eq!(animals.get_data(3), "Lizard");
    }

    #[test] // for vertnum after removing (all vertices have 4 edgeverts)
    fn complex_get_vn1() {
 
        const same_size: usize = BITS * 4;
        const binary_size: usize = BITS * 2;
        const u4_size: usize = BITS;
        const u8_size: usize = BITS / 2;
        const u16_size: usize = BITS / 4;
        const u32_size: usize = BITS / 8;

        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, same_size);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, binary_size);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, u4_size);
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, u8_size);
        let mut bg_u16: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U16, u16_size);
        let mut bg_u32: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U32, u32_size);

        for v in 0..same_size { bg_same.add(NoData); }
        for v in 0..binary_size { bg_binary.add(NoData); }
        for v in 0..u4_size { bg_u4.add(NoData); }
        for v in 0..u8_size { bg_u8.add(NoData); }
        for v in 0..u16_size { bg_u16.add(NoData); }
        for v in 0..u32_size { bg_u32.add(NoData); }

        // sanity checking edgevert lengths
        for v in 0..same_size {
            assert_eq!(4, bg_same.ev_len_at(v));
            if v < binary_size {
                assert_eq!(4, bg_binary.ev_len_at(v));
            }
            if v < u4_size {
                assert_eq!(4, bg_u4.ev_len_at(v));
            }
            if v < u8_size {
                assert_eq!(4, bg_u8.ev_len_at(v));
            }
            if v < u16_size {
                assert_eq!(4, bg_u16.ev_len_at(v));
            }
            if v < u32_size {
                assert_eq!(4, bg_u32.ev_len_at(v));
            }

        }

        // Testing get_vn() with remove()
        for _ in 0..same_size {
            for v in 0..bg_same.size() { assert_eq!(v, bg_same.get_vn(v)); }
            bg_same.remove(0);
        }
        for _ in 0..binary_size {
            for v in 0..bg_binary.size() { assert_eq!(v, bg_binary.get_vn(v)); }
            bg_binary.remove(0);
        }
        for _ in 0..u4_size {
            for v in 0..bg_u4.size() { assert_eq!(v, bg_u4.get_vn(v)); }
            bg_u4.remove(0);
        }
        for _ in 0..u8_size {
            for v in 0..bg_u8.size() { assert_eq!(v, bg_u8.get_vn(v)); }
            bg_u8.remove(0);
        }
        for _ in 0..u16_size {
            for v in 0..bg_u16.size() { assert_eq!(v, bg_u16.get_vn(v)); }
            bg_u16.remove(0);
        }
        for _ in 0..u32_size {
            for v in 0..bg_u32.size() { assert_eq!(v, bg_u32.get_vn(v)); }
            bg_u32.remove(0);
        } 
        assert_eq!(0, bg_same.size());
        assert_eq!(0, bg_binary.size());
        assert_eq!(0, bg_u4.size());
        assert_eq!(0, bg_u8.size());
        assert_eq!(0, bg_u16.size());
        assert_eq!(0, bg_u32.size());
    }

    #[test]
    fn simple_get_vn1() {
 
        const same_size: usize = BITS;
        const binary_size: usize = BITS / 2;
        const u4_size: usize = BITS / 4;
        const u8_size: usize = BITS / 8;
        const u16_size: usize = BITS / 16;
        const u32_size: usize = BITS / 32;

        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, same_size);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, binary_size);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, u4_size);
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, u8_size);
        let mut bg_u16: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U16, u16_size);
        let mut bg_u32: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U32, u32_size);
      

        for v in 0..same_size { bg_same.add(NoData); }
        for v in 0..binary_size { bg_binary.add(NoData); }
        for v in 0..u4_size { bg_u4.add(NoData); }
        for v in 0..u8_size { bg_u8.add(NoData); }
        for v in 0..u16_size { bg_u16.add(NoData); }
        for v in 0..u32_size { bg_u32.add(NoData); }

        // Testing get_vn() with remove()
        for _ in 0..same_size {
            for v in 0..bg_same.size() { assert_eq!(v, bg_same.get_vn(v)); }
            bg_same.remove(0);
        }
        for _ in 0..binary_size {
            for v in 0..bg_binary.size() { assert_eq!(v, bg_binary.get_vn(v)); }
            bg_binary.remove(0);
        }
        for _ in 0..u4_size {
            for v in 0..bg_u4.size() { assert_eq!(v, bg_u4.get_vn(v)); }
            bg_u4.remove(0);
        }
        for _ in 0..u8_size {
            for v in 0..bg_u8.size() { assert_eq!(v, bg_u8.get_vn(v)); }
            bg_u8.remove(0);
        }
        for _ in 0..u16_size {
            for v in 0..bg_u16.size() { assert_eq!(v, bg_u16.get_vn(v)); }
            bg_u16.remove(0);
        }
        for _ in 0..u32_size {
            for v in 0..bg_u32.size() { assert_eq!(v, bg_u32.get_vn(v)); }
            bg_u32.remove(0);
        }
    }
    
    #[test]
    fn human_evolution() {
        const same_size: usize = 7;
        // based on https://i.pinimg.com/originals/5e/35/19/5e35191ccc1d0d7c7f40009d358157b9.jpg 
        let mut human_evol_tree: BitGraph<String> = BitGraph::new_with_capacity(EdgeScale::SAME, same_size);
        
        let habilis = "habilis";
        let erectus = "erectus";
        let heidelbergensis = "heidelbergensis";
        let naledi = "naledi";
        let neanderthal = "neanderthal"; 
        let floresiensis  = "floresiensis";     
        let sapiens = "sapiens";
        
        human_evol_tree.add(habilis.to_string()); // vertnum = 0
        human_evol_tree.add(erectus.to_string()); // vertnum = 1
        human_evol_tree.add(heidelbergensis.to_string()); // vertnum = 2
        human_evol_tree.add(naledi.to_string()); // vertnum = 3
        human_evol_tree.add(floresiensis.to_string()); // vertnum = 4
        human_evol_tree.add(neanderthal.to_string()); // vertnum = 5 
        human_evol_tree.add(sapiens.to_string()); // vertnum = 6

        assert_eq!(human_evol_tree.size(), 7);
        human_evol_tree.connect(0, 1, 0);
        human_evol_tree.connect(0, 3, 0);
        human_evol_tree.connect(1, 2, 0);
        human_evol_tree.connect(1, 4, 0);
        human_evol_tree.connect(2, 5, 0);
        human_evol_tree.connect(2, 6, 0);
        
        // verifying connection...
        assert_eq!(human_evol_tree.ev_num_at(0, 0), 0xa); // ...._1010
        assert_eq!(human_evol_tree.ev_num_at(1, 0), 0x14); // ...._0001_0100
        assert_eq!(human_evol_tree.ev_num_at(2, 0), 0x60); // ...._0110_0000
        assert_eq!(human_evol_tree.ev_num_at(3, 0), 0x0); // ...._000
        assert_eq!(human_evol_tree.ev_num_at(4, 0), 0x0); // ...._0000
        assert_eq!(human_evol_tree.ev_num_at(5, 0), 0x0); // ...._0000   
        assert_eq!(human_evol_tree.ev_num_at(6, 0), 0x0); // ...._0000

        // another verification...
        assert!(human_evol_tree.is_connected(0, 1));
        assert!(human_evol_tree.is_connected(0, 3));
        assert!(human_evol_tree.is_connected(1, 2));
        assert!(human_evol_tree.is_connected(1, 4));
        assert!(human_evol_tree.is_connected(2, 5));
        assert!(human_evol_tree.is_connected(2, 6));
        assert!(!human_evol_tree.is_connected(0, 2));
        assert!(!human_evol_tree.is_connected(5, 6));
        assert!(!human_evol_tree.is_connected(3, 4));

        // GOING BACK IN TIME!!!
        human_evol_tree.remove(6); // removing sapiens...
        assert_eq!(human_evol_tree.ev_num_at(2, 0), 0x20); // ...._0010_0000
        human_evol_tree.remove(5); // removing neanderthal...
        assert_eq!(human_evol_tree.ev_num_at(2, 0), 0x0); // ...._0000_0000
        assert_eq!(human_evol_tree.size(), 5);

        human_evol_tree.remove(4)
    }

    #[test]
    fn complex_remove3() { // enough vertices to fill ev0 and ev1

        const same_size: usize = BITS * 2;
        const binary_size: usize = BITS;
        const u4_size: usize = BITS / 2;
        const u8_size: usize = BITS / 4;
        const u16_size: usize = BITS / 8;
        const u32_size: usize = BITS / 16;

        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, same_size);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, binary_size);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, u4_size);
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, u8_size);
        let mut bg_u16: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U16, u16_size);
        let mut bg_u32: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U32, u32_size);

        for v in 0..same_size { bg_same.add(NoData); }
        for v in 0..binary_size { bg_binary.add(NoData); }
        for v in 0..u4_size { bg_u4.add(NoData); }
        for v in 0..u8_size { bg_u8.add(NoData); }
        for v in 0..u16_size { bg_u16.add(NoData); }
        for v in 0..u32_size { bg_u32.add(NoData); }

        // sanity check
        for v in 0..same_size { assert_eq!(bg_same.ev_len_at(v), 2); }
        for v in 0..binary_size { assert_eq!(bg_binary.ev_len_at(v), 2); }
        for v in 0..u4_size { assert_eq!(bg_u4.ev_len_at(v), 2); }
        for v in 0..u8_size { assert_eq!(bg_u8.ev_len_at(v), 2); }
        for v in 0..u16_size { assert_eq!(bg_u16.ev_len_at(v), 2); }
        for v in 0..u32_size { assert_eq!(bg_u32.ev_len_at(v), 2); }

        // connecting with max
        bg_same.connect(BITS / 2 - 1, BITS * 2 - (BITS / 2) - 1, 0);
        bg_binary.connect(BITS / 4 - 1, BITS - (BITS / 4) - 1, 1);            
        bg_u4.connect(BITS / 8 - 1, (BITS / 2) - (BITS / 8) - 1, 7);
        bg_u8.connect(BITS / 16 - 1, (BITS / 4) - (BITS / 16) - 1, 127);
        bg_u16.connect(BITS / 32 - 1, (BITS / 8) - (BITS / 32) - 1, 32_767);
        bg_u32.connect(BITS / 64 - 1, (BITS / 16) - (BITS / 64) - 1, 2_147_483_647);
    }

    #[test] // U8, U16, U32
    fn complex_remove2() { // testing for connections across 2 edgeverts (0 to 1)

        // '+ 1' to get the extra edgevert
        const u8_size: usize = BITS / 8 + 1;
        const u16_size: usize = BITS / 16 + 1;
        const u32_size: usize = BITS / 32 + 1;

        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, u8_size);
        let mut bg_u16: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U16, u16_size);
        let mut bg_u32: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U32, u32_size);
        
        // Initializing graphs
        for _ in 0..u8_size { bg_u8.add(NoData); }
        for _ in 0..u16_size { bg_u16.add(NoData); }
        for _ in 0..u32_size { bg_u32.add(NoData); }

        // sanity check
        assert_eq!(u8_size, bg_u8.size());
        assert_eq!(u16_size, bg_u16.size());
        assert_eq!(u32_size, bg_u32.size());
        
        // middle ev0 to middle ev1. weights are at max
        bg_u8.connect(0, u8_size - 1, 127);
        bg_u16.connect(0, u16_size - 1, 32_767); 
        bg_u32.connect(0, u32_size - 1, 2_147_483_647);

        // another sanity check
        assert!(bg_u8.is_connected(0, u8_size - 1));
        assert!(bg_u16.is_connected(0, u16_size - 1));
        assert!(bg_u32.is_connected(0, u32_size - 1));

        // and another sanity check...
        assert_eq!(bg_u8.ev_num_at(0, 1), 0xff); // ...._1111_1111
        assert_eq!(bg_u16.ev_num_at(0, 1), 0xffff); // ...._1111_1111_1111_1111 
        assert_eq!(bg_u32.ev_num_at(0, 1), 0xffffffff); // 32 1's :)


        // sanity check for size of edgeverts
        for v in 0..u8_size { assert_eq!(bg_u8.ev_len_at(v), 2); }
        for v in 0..u16_size { assert_eq!(bg_u16.ev_len_at(v), 2); }
        for v in 0..u32_size { assert_eq!(bg_u32.ev_len_at(v), 2); }
 
        // where the fun begins (i.e., removing the middle bits at ev1)
        bg_u8.remove(u8_size - 1);
        bg_u16.remove(u16_size - 1);
        bg_u32.remove(u32_size - 1);

        // sanity checking non connected vertices within ev0
        assert_eq!(bg_u8.ev_num_at(0, 0), 0);
        assert_eq!(bg_u16.ev_num_at(0, 0), 0);
        assert_eq!(bg_u32.ev_num_at(0, 0), 0);

        // should be '1' for all since removing till no need for an extra edgevert
        for v in 0..(u8_size - 1) { assert_eq!(bg_u8.ev_len_at(v), 1); }
        for v in 0..(u16_size - 1) { assert_eq!(bg_u16.ev_len_at(v), 1); }
        for v in 0..(u32_size - 1) { assert_eq!(bg_u32.ev_len_at(v), 1); }

    }

    #[test] // SAME, BINARY, U4
    fn complex_remove1() { // testing for connections across 2 edgeverts (0 to 1)

        // '+ 1' to get the extra edgevert
        const same_size: usize = BITS + 1;
        const binary_size: usize = BITS / 2 + 1;
        const u4_size: usize = BITS / 4 + 1;
        
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, same_size);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, binary_size);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, u4_size);

        // Initializing graphs
        for _ in 0..same_size { bg_same.add(NoData); }
        for _ in 0..binary_size { bg_binary.add(NoData); }
        for _ in 0..u4_size { bg_u4.add(NoData); } 
       
        // sanity check
        assert_eq!(same_size, bg_same.size());
        assert_eq!(binary_size, bg_binary.size());
        assert_eq!(u4_size, bg_u4.size());

        // from v0 to v(BITS / EdgeScale) with max weights  
        bg_same.connect(0, same_size - 1, 0);
        bg_binary.connect(0, binary_size - 1, 1); 
        bg_u4.connect(0, u4_size - 1, 7);

        // another sanity check
        assert!(bg_same.is_connected(0, same_size - 1));
        assert!(bg_binary.is_connected(0, binary_size - 1));
        assert!(bg_u4.is_connected(0, u4_size - 1));

        // and another sanity check...
        assert_eq!(bg_same.ev_num_at(0, 1), 1); // ...._0001
        assert_eq!(bg_binary.ev_num_at(0, 1), 3); // ...._0011
        assert_eq!(bg_u4.ev_num_at(0, 1), 15); // ...._1111

        // sanity check for size of edgeverts
        for v in 0..same_size { assert_eq!(bg_same.ev_len_at(v), 2); }
        for v in 0..binary_size { assert_eq!(bg_binary.ev_len_at(v), 2); }
        for v in 0..u4_size { assert_eq!(bg_u4.ev_len_at(v), 2); }
         
        // where the fun begins (i.e., removing the middle bits at ev1)
        bg_same.remove(same_size - 1);
        bg_binary.remove(binary_size - 1);
        bg_u4.remove(u4_size - 1);

        // sanity checking non connected vertices within ev0
        assert_eq!(bg_same.ev_num_at(0, 0), 0);
        assert_eq!(bg_binary.ev_num_at(0, 0), 0);
        assert_eq!(bg_u4.ev_num_at(0, 0), 0);

        // should be '1' for all since removing till no need for an extra edgevert
        for v in 0..(same_size - 1) { assert_eq!(bg_same.ev_len_at(v), 1); }
        for v in 0..(binary_size - 1) { assert_eq!(bg_binary.ev_len_at(v), 1); }
        for v in 0..(u4_size - 1) { assert_eq!(bg_u4.ev_len_at(v), 1); }

    }
    
    fn simple_remove5() { // removing where connections start at index 0 in ev1 
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, BITS * 2);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, BITS * 2);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, BITS * 2);
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, BITS * 2);
        let mut bg_u16: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U16, BITS * 2);
        let mut bg_u32: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U32, BITS * 2);
 
        // Initializing graphs
        for _ in 0..(BITS * 2) { bg_same.add(NoData); }
        for _ in 0..BITS { bg_binary.add(NoData); }
        for _ in 0..(BITS / 2) { bg_u4.add(NoData); } 
        for _ in 0..(BITS / 4) { bg_u8.add(NoData); }
        for _ in 0..(BITS / 8) { bg_u16.add(NoData); }
        for _ in 0..(BITS / 16) { bg_u32.add(NoData); }

       
        // weights are at max
        bg_same.connect(0, BITS, 0);
        bg_binary.connect(0, BITS / 2, 1);
        bg_u4.connect(0, BITS / 4, 7);
        bg_u8.connect(BITS / 16 - 1, (BITS * 3) / 16 - 1, 127);
        bg_u16.connect(BITS / 32 - 1, (BITS * 3) / 32 - 1, 32_767); 
        bg_u32.connect(BITS / 64 - 1, (BITS * 3) / 64 - 1, 2_147_483_647);
 
        // sanity check
        assert!(bg_same.is_connected(0, BITS));
        assert!(bg_binary.is_connected(0, BITS / 2));
        assert!(bg_u4.is_connected(0, BITS / 4));
        assert!(bg_u8.is_connected(0, BITS / 8));
        assert!(bg_u16.is_connected(0, BITS / 16));
        assert!(bg_u32.is_connected(0, BITS / 32));
         

        bg_same.remove(BITS);
        bg_binary.remove(BITS / 2);
        bg_u4.remove(BITS / 4); 
        bg_u8.remove(BITS / 8);
        bg_u16.remove(BITS / 16);
        bg_u32.remove(BITS / 32);

        // '+ bg_?.ev_num_at(0,0)' to make sure there is not some weird connection issues
        assert_eq!(bg_same.ev_num_at(0, 1) + bg_same.ev_num_at(0, 0), 0);
        assert_eq!(bg_binary.ev_num_at(0, 1) + bg_binary.ev_num_at(0, 0), 0);
        assert_eq!(bg_u4.ev_num_at(0, 1) + bg_u4.ev_num_at(0, 0), 0);
        assert_eq!(bg_u8.ev_num_at(0, 1) + bg_u8.ev_num_at(0, 0), 0);
        assert_eq!(bg_u16.ev_num_at(0, 1) + bg_u16.ev_num_at(0, 0), 0);
        assert_eq!(bg_u32.ev_num_at(0, 1) + bg_u32.ev_num_at(0, 0), 0);
 
    }

    fn simple_remove4() { // BIT CIRCLE
        // utilizing all bits
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, BITS);
        for _ in 0..BITS { bg_same.add(NoData); } // pushing data...
        // making big circle
        for v in 0..(BITS - 1) { bg_same.connect(v, v + 1, 0); }
        // final (cyclic) connection. Last vertex to 1st
        bg_same.connect(BITS - 1, 0, 0);
        // Double checking connections...
        for v in 0..(BITS - 1) { assert!(bg_same.is_connected(v, v + 1)); }
        assert!(bg_same.is_connected(BITS - 1, 0));
        let n: usize = 2; // For potential editing purposes 
        bg_same.remove(BITS / n); // removing middle vertex...
        // assert_eq!(BITS - 1, bg_same.size());

        // all pre-vertex removal should be unchanged
        for v in 0..(BITS / n - 1) { 
            println!("WHEN Vl1 = {}...\n", v);
            assert_eq!(2 << v, bg_same.ev_num_at(v, 0));
        }
        // all post-vertex removal should be '>>' by 1
        // minus 2 since bg_same.connect(BITS - 1, 0, 0);
        for v in (BITS / n)..(BITS - 2) { 
            println!("WHEN Vl2 = {}...\n", v);
            assert_eq!(1 << (v + 1), bg_same.ev_num_at(v, 0));
        }
        assert_eq!(1, bg_same.ev_num_at(BITS - 2, 0));
    }

    #[test] 
    fn simple_remove3() {
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, 4);
        for _ in 0..4 { bg_u4.add(NoData); } // pushing data..
        for x in 0..4 { bg_u4.connect(0, x, 7); } // Connects 0 to 1, 2, and 3
        
        // chopping edgevert bits from vertex1
        // Pre-removal: 1111 1111 1111 1111
        bg_u4.remove(1); // 0000 1111 1111 1111
        assert_eq!(3, bg_u4.size());
        assert_eq!(4_095, bg_u4.ev_num_at(0, 0));
        bg_u4.remove(1); // 0000 0000 1111 1111
        assert_eq!(2, bg_u4.size());
        assert_eq!(255, bg_u4.ev_num_at(0, 0));
        bg_u4.remove(1); // 0000 0000 0000 1111
        assert_eq!(1, bg_u4.size());
        assert_eq!(15, bg_u4.ev_num_at(0, 0));

        // Finally, removing last vertex (vertex0)
        bg_u4.remove(0);
        assert_eq!(0, bg_u4.size());
    }

    #[test] // Removing  0 -> 1 (Same as simple_remove1, but EdgeScale::SAME)
    fn simple_remove2() {
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, 2);
        bg_same.add(NoData);
        bg_same.add(NoData);

        bg_same.connect(0, 1, 0);
        bg_same.remove(1);
        assert!(bg_same.size() == 1);
        assert_eq!(0, bg_same.ev_num_at(0, 0));
    }

    #[test] // Removing  0 -> 1
    fn simple_remove1() {
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, 2);
        bg_binary.add(NoData);
        bg_binary.add(NoData);

        bg_binary.connect(0, 1, 0);
        bg_binary.remove(1);
        assert!(bg_binary.size() == 1);
        assert_eq!(0, bg_binary.ev_num_at(0, 0));
    }

    #[test] // testing edgevert encoding from v0 to vX (by invocing edgevert replenish)
    fn complex_is_connected3(){
        let graph_length: usize = BITS * 2;
        
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, graph_length / 8);
        let mut bg_u16: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U16, graph_length / 16);
        let mut bg_u32: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U32, graph_length / 32);
        
        for _ in 0..(graph_length / 8) { bg_u8.add(NoData); }
        for _ in 0..(graph_length / 16) { bg_u16.add(NoData); }
        for _ in 0..(graph_length / 32) { bg_u32.add(NoData); }
       
        // sanity check
        assert_eq!(BITS / 4, bg_u8.size());
        assert_eq!(BITS / 8, bg_u16.size());
        assert_eq!(BITS / 16, bg_u32.size());
        
        // '- 1' for indexing
        bg_u8.connect(0, graph_length / 16 - 1, 127);
        bg_u16.connect(0, graph_length / 32 - 1, 32_767);
        bg_u32.connect(0, graph_length / 64 - 1, 2_147_483_647);

        // with sanity checks
        assert!(bg_u8.is_connected(0, graph_length / 16 - 1));
        assert!(bg_u16.is_connected(0, graph_length / 32 - 1));
        assert!(bg_u32.is_connected(0, graph_length / 64 - 1));
        }
 

    #[test] // testing edgevert encoding from v0 to vX (by invocing edgevert replenish)
    fn complex_is_connected2(){
        let graph_length: usize = BITS * 2;
        
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, graph_length);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, graph_length / 2);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, graph_length / 4);
        
        for _ in 0..(graph_length) { bg_same.add(NoData); }
        for _ in 0..(graph_length / 2) { bg_binary.add(NoData); }
        for _ in 0..(graph_length / 4) { bg_u4.add(NoData); }
       
        // sanity check
        assert_eq!(BITS * 2, bg_same.size());
        assert_eq!(BITS, bg_binary.size());
        assert_eq!(BITS / 2, bg_u4.size());

        // '- 1' for indexing
        bg_same.connect(0, graph_length / 2 - 1, 0);
        bg_binary.connect(0, graph_length / 4 - 1, 1);
        bg_u4.connect(0, graph_length / 8 - 1, 7);

        // with sanity checks
        assert!(bg_same.is_connected(0, graph_length / 2 - 1));
        assert!(bg_binary.is_connected(0, graph_length / 4 - 1));
        assert!(bg_u4.is_connected(0, graph_length / 8 - 1));
    }
    #[test] // testing edgevert encoding from v0 to vX (by invocing edgevert replenish)
    fn complex_is_connected1(){
        let graph_length: usize = BITS * 2;
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, graph_length);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, graph_length / 2);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, graph_length / 4);
        
        for _ in 0..(graph_length) { bg_same.add(NoData); }
        for _ in 0..(graph_length / 2) { bg_binary.add(NoData); }
        for _ in 0..(graph_length / 4) { bg_u4.add(NoData); }
       
        // sanity check
        assert_eq!(BITS * 2, bg_same.size());
        assert_eq!(BITS, bg_binary.size());
        assert_eq!(BITS / 2, bg_u4.size());

        // '- 1' for indexing
        bg_same.connect(0, graph_length / 2 - 1, 0);
        bg_binary.connect(0, graph_length / 4 - 1, 1);
        bg_u4.connect(0, graph_length / 8 - 1, 7);

        assert!(bg_same.is_connected(0, graph_length / 2 - 1));
        assert!(bg_binary.is_connected(0, graph_length / 4 - 1));
        assert!(bg_u4.is_connected(0, graph_length / 8 - 1));
    }

    #[test] // test if not connected
    fn simple_is_connected4() {
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, 4);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, 4);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, 4);

        for _ in 0..4 {
            bg_same.add(NoData);
            bg_binary.add(NoData);
            bg_u4.add(NoData);
        }
        // No connections made. Therefore, ...
        for x in 0..4 {
            assert!(!bg_same.is_connected(0, 1));
            assert!(!bg_binary.is_connected(1, 3));
            assert!(!bg_u4.is_connected(2, 0));
        }

    }

    #[test] // connecting one element to itself
    fn simple_is_connected3() {
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, 1);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, 1);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, 1);
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, 1);
        let mut bg_u16: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U16, 1);
        let mut bg_u32: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U32, 1);
        
        bg_same.add(NoData);
        bg_binary.add(NoData);
        bg_u4.add(NoData);
        bg_u8.add(NoData);
        bg_u16.add(NoData);
        bg_u32.add(NoData);

        // connecting 0 -> 0 with a weight of 0 in all graph types
        bg_same.connect(0, 0, 0);
        bg_binary.connect(0, 0, 0);
        bg_u4.connect(0, 0, 0);
        bg_u8.connect(0, 0, 0);
        bg_u16.connect(0, 0, 0);
        bg_u32.connect(0, 0, 0);

        assert!(bg_same.is_connected(0, 0));
        assert!(bg_binary.is_connected(0, 0));
        assert!(bg_u4.is_connected(0, 0));
        assert!(bg_u8.is_connected(0, 0));
        assert!(bg_u16.is_connected(0, 0));
        assert!(bg_u32.is_connected(0, 0));
    }

    #[test] // connects v0 to v1, v2, ..., v499 and checks if connected
    fn simple_is_connected2() {
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, 500);
        for _ in 0..500 { bg_u8.add(NoData); }
        for x in 0..500 { bg_u8.connect(0, x, 127); }
        for x in 0..500 { 
            println!("{} passed for simple_is_connected_test2()", x);
            assert!(bg_u8.is_connected(0, x)); 
        }
    }

    #[test]
    fn simple_is_connected1() {
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, 2);
        bg_same.add(NoData);
        bg_same.add(NoData);
        bg_same.connect(0, 1, 0);
        assert!(bg_same.is_connected(0, 1));
        bg_same.add(NoData);
        bg_same.connect(0, 2, 0);
        assert!(bg_same.is_connected(0, 2));
    }


    #[test]
    fn simple_add_copies1() {
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, 10);
        bg_same.add_copies(NoData, 10);
        assert_eq!(10, bg_same.size());
        bg_same.add_copies(NoData, 10);
        assert_eq!(20, bg_same.size());
    }

    #[test]
    fn complex_disconnect1() {
        let mut bg_u32: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U32, 4);
        for _ in 0..4 { bg_u32.add(NoData); }
        bg_u32.connect(0, 2, 1_000_000);
        bg_u32.connect(1, 3, 2_000_000);
        for v in 0..4 { assert_eq!(bg_u32.ev_len_at(v), BITS / 32); }
        assert!(bg_u32.is_connected(0, 2));
        bg_u32.disconnect(0, 2); 
        assert!(!bg_u32.is_connected(0, 2));
        assert!(bg_u32.is_connected(1, 3));
        bg_u32.disconnect(1, 3);
        assert!(!bg_u32.is_connected(1, 3));

    }

    #[test]
    fn simple_disconnect2() {
        let mut bg_u16: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U16, 2);
        bg_u16.add(NoData);
        bg_u16.add(NoData);
        bg_u16.connect(0, 1, 10_000);
        assert!(bg_u16.is_connected(0, 1));
        bg_u16.disconnect(0, 1);
        assert!(!bg_u16.is_connected(0, 1));
    }

    #[test]
    fn simple_disconnect1() {
        
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, 4);
        bg_same.add_copies(NoData, 4);

        for v in 0..4 { bg_same.connect(0, v, 0); } // v0ev0: .... 0000 1111
        // vXevY @ 'POST REMOVAL'
        bg_same.disconnect(0, 3); // v0ev0: .... 0000 0111
        assert_eq!(7, bg_same.ev_num_at(0,0));
        bg_same.disconnect(0, 2); // v0ev0: .... 0000 0011
        assert_eq!(3, bg_same.ev_num_at(0,0));
        bg_same.disconnect(0, 1); // v0ev0: .... 0000 0001
        assert_eq!(1, bg_same.ev_num_at(0,0));
        bg_same.disconnect(0, 0); // v0ev0: .... 0000 0000
        assert_eq!(0, bg_same.ev_num_at(0,0));

    }

    #[test]
    #[should_panic]
    fn panic_connect() { 
        let bits2: usize = BITS / 8 + 1;
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, bits2); 
        for _ in 0..bits2 { bg_u8.add(NoData); }
        bg_u8.connect(0, bits2, 50); // fails because bg_u8.size() = bits2
    }

    #[test] // verifying connection using the edgevert nums
    fn complex_connect1() {
        // bits2 explained: BITS / 8 + 1 is needed to add an extra edgevert for all vertices
        // to get exactly 2 edgeverts for all vertices.
        let bits2: usize = BITS / 8 + 1;
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, bits2); 
        for _ in 0..=bits2 { bg_u8.add(NoData); }

        // checking for exactly 2 edgeverts per vertex
        for x in 0..bits2 { assert_eq!(2, bg_u8.ev_len_at(x)); }

        // If BITS = 64, then
        // {0} --> {bits2 - 1 = 8} with weight 122
        // {0} --> {bits2 = 9} with weight 16
        // {0} --> {bits2 + 1 = 9} with weight 70
        
        bg_u8.connect(0, bits2 - 1, 122);
        // bg_u8.add(NoData); // need to add extra here, or it will panic
        assert_eq!(250, bg_u8.ev_num_at(0, 1));   

        // Connecting from v0 to v9 (which is in edgevert1)
        bg_u8.connect(0, bits2, 16);
        bg_u8.add(NoData); // same here
        assert_eq!(37_114, bg_u8.ev_num_at(0, 1));   
        
        bg_u8.connect(0, bits2 + 1, 70);
        assert_eq!(13_013_242, bg_u8.ev_num_at(0, 1));   

    }

    #[test]
    fn simple_connect5() {
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, 1);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, 1);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, 1);
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, 1);
        let mut bg_u16: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U16, 1);
        let mut bg_u32: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U32, 1);
        
        bg_same.add(NoData);
        bg_binary.add(NoData);
        bg_u4.add(NoData);
        bg_u8.add(NoData);
        bg_u16.add(NoData);
        bg_u32.add(NoData);

        // connecting 0 -> 0 with a weight of 0 in all graph types
        bg_same.connect(0, 0, 0);
        bg_binary.connect(0, 0, 0);
        bg_u4.connect(0, 0, 0);
        bg_u8.connect(0, 0, 0);
        bg_u16.connect(0, 0, 0);
        bg_u32.connect(0, 0, 0);

        // all failing because all bits are off by the next edge scale
        assert_eq!(1, bg_same.ev_num_at(0, 0)); // <- The only one that works...
        assert_eq!(2, bg_binary.ev_num_at(0, 0));
        assert_eq!(8, bg_u4.ev_num_at(0, 0));
        assert_eq!(128, bg_u8.ev_num_at(0, 0));
        assert_eq!(32_768, bg_u16.ev_num_at(0, 0));
        assert_eq!(2_147_483_648, bg_u32.ev_num_at(0, 0));
    }

    #[test] // Only works for 64-bit machines...
    fn simple_connect4() { // connect v0 to v0, v1, v2, ..., v63
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, 64);
        for _ in 0..64 { bg_same.add(NoData); }
        for x in 0..64 { bg_same.connect(0, x, 0); }
        assert_eq!(0xffffffffffffffff, bg_same.ev_num_at(0, 0));
    }

    #[test] // May only work for 64-bit machines...
    fn simple_connect3() {

        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, 11);
        for _ in 0..11 { bg_binary.add(NoData); }

        // 0's
        bg_binary.connect(0, 1, 0);
        bg_binary.connect(0, 3, 1);
        // 1's
        bg_binary.connect(1, 3, 1);
        bg_binary.connect(1, 4, 0);
        bg_binary.connect(1, 5, 0);
        // 2's
        bg_binary.connect(2, 1, 1);        
        bg_binary.connect(2, 5, 0);
        // singles...
        bg_binary.connect(3, 6, 0); // 3
        bg_binary.connect(4, 7, 1); // 4
        bg_binary.connect(5, 8, 1); // 5
        bg_binary.connect(6, 9, 1); // 6
        bg_binary.connect(8, 10, 1);
        // 7's
        bg_binary.connect(7, 9, 0);
        bg_binary.connect(7, 10, 0);

        assert_eq!(200, bg_binary.ev_num_at(0, 0));
        assert_eq!(2_752, bg_binary.ev_num_at(1, 0));
        assert_eq!(2_060, bg_binary.ev_num_at(2, 0));
        assert_eq!(8_192, bg_binary.ev_num_at(3, 0));
        assert_eq!(49_152, bg_binary.ev_num_at(4, 0));
        assert_eq!(196_608, bg_binary.ev_num_at(5, 0));
        assert_eq!(786_432, bg_binary.ev_num_at(6, 0));
        assert_eq!(2_621_440, bg_binary.ev_num_at(7, 0));
        assert_eq!(3_145_728, bg_binary.ev_num_at(8, 0));
        assert_eq!
        (
            0, bg_binary.ev_num_at(9, 0) + bg_binary.ev_num_at(10, 0) 
        );   
    }

    #[test] // May only work for 64-bit machines...
    fn simple_connect2() {
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, 5);
        for _ in 0..5 { bg_u4.add(NoData); }

        bg_u4.connect(0, 1, 1);
        bg_u4.connect(0, 2, 5);
        bg_u4.connect(0, 3, 2);
        bg_u4.connect(1, 3, 2);
        bg_u4.connect(1, 4, 3);
        bg_u4.connect(2, 4, 4);
        bg_u4.connect(3, 2, 7);
        bg_u4.connect(3, 4, 6);

        assert_eq!(44_432, bg_u4.ev_num_at(0, 0));
        assert_eq!(761_856, bg_u4.ev_num_at(1, 0));
        assert_eq!(786_432, bg_u4.ev_num_at(2, 0));
        assert_eq!(921_344, bg_u4.ev_num_at(3, 0));
        assert_eq!(0, bg_u4.ev_num_at(4, 0));

    }

    #[test] // May only work for 64-bit machines...
    fn simple_connect1() {
        
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, 4);
        for _ in 0..4 { bg_same.add(NoData); } // adding 5 elements

        // weigth must be 0 since EdgeScale::SAME
        bg_same.connect(0, 2, 0); 
        bg_same.connect(1, 2, 0); 
        bg_same.connect(1, 3, 0);
        bg_same.connect(2, 3, 0);
        bg_same.connect(3, 0, 0); 

        assert_eq!(4, bg_same.ev_num_at(0, 0));
        assert_eq!(12, bg_same.ev_num_at(1, 0));
        assert_eq!(8, bg_same.ev_num_at(2, 0));
        assert_eq!(1, bg_same.ev_num_at(3, 0));
    }

    // Testing many edgevert lenghts.... This takes a long time
    #[test] 
    fn complex_ev_len_at1() {        
        pub fn test_for_many_verts(vert_amt: usize, bg: BitGraph<NoData>, es: usize) { 
            let num: usize;
            if vert_amt % (BITS / es) != 0 {
                num = 1; 
            } else {
                num = 0;
            }
            for x in 0..vert_amt {
                assert_eq!(vert_amt / (BITS / es) + num, bg.ev_len_at(x));
            }
        }

        pub fn add_verts(bg: &mut BitGraph<NoData>, c: usize) {
            for _ in 0..c { bg.add(NoData); }
        }

        for cap in 1..2_048 {

            let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, cap);
            let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, cap);
            let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, cap);
            let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, cap);
            let mut bg_u16: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U16, cap);
            let mut bg_u32: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U32, cap);
            
            add_verts(&mut bg_same, cap);
            add_verts(&mut bg_binary, cap);
            add_verts(&mut bg_u4, cap);
            add_verts(&mut bg_u8, cap);
            add_verts(&mut bg_u16, cap);
            add_verts(&mut bg_u32, cap);

            let t1: JoinHandle<()> =
                std::thread::spawn(move || { test_for_many_verts(cap, bg_same, 1) }); 
            let t2: JoinHandle<()> =
                std::thread::spawn(move || { test_for_many_verts(cap, bg_binary, 2) }); 
            let t3: JoinHandle<()> =
                std::thread::spawn(move || { test_for_many_verts(cap, bg_u4, 4) });
            let t4: JoinHandle<()> =
                std::thread::spawn(move || { test_for_many_verts(cap, bg_u8, 8) });  
            let t5: JoinHandle<()> =
                std::thread::spawn(move || { test_for_many_verts(cap, bg_u16, 16) });  
            let t6: JoinHandle<()> =
                std::thread::spawn(move || { test_for_many_verts(cap, bg_u32, 32) });  

            match t1.join() {
                Ok(_) => assert!(true),
                Err(_) => panic!("t1.join() failed"),
            }
            match t2.join() {
                Ok(_) => assert!(true),
                Err(_) => panic!("t2.join() failed"),
            }
            match t3.join() {
                Ok(_) => assert!(true),
                Err(_) => panic!("t3.join() failed"),
            }
            match t4.join() {
                Ok(_) => assert!(true),
                Err(_) => panic!("t4.join() failed"),
            }
            match t5.join() {
                Ok(_) => assert!(true),
                Err(_) => panic!("t5.join() failed"),
            }
            match t6.join() {
                Ok(_) => assert!(true),
                Err(_) => panic!("t6.join() failed"),
            }
        }    
    }

    #[test]
    fn simple_ev_len_at1() {

        let vert_amt: usize = 0;
        
        let mut my_bg1: BitGraph<NoData> = BitGraph::new(EdgeScale::BINARY);
        let mut my_bg2: BitGraph<NoData> = BitGraph::new(EdgeScale::U4);
        let mut my_bg3: BitGraph<NoData> = BitGraph::new(EdgeScale::U8);
        let mut my_bg4: BitGraph<NoData> = BitGraph::new(EdgeScale::U16);
        let mut my_bg5: BitGraph<NoData> = BitGraph::new(EdgeScale::U32);
        
        assert_eq!(0, my_bg1.size());
        assert_eq!(0, my_bg2.size());
        assert_eq!(0, my_bg3.size());
        assert_eq!(0, my_bg4.size());
        assert_eq!(0, my_bg5.size());

        assert_eq!(2, my_bg1.get_partition_size());
        assert_eq!(4, my_bg2.get_partition_size());
        assert_eq!(8, my_bg3.get_partition_size());
        assert_eq!(16, my_bg4.get_partition_size());
        assert_eq!(32, my_bg5.get_partition_size());


        for _ in 0..vert_amt {
            my_bg1.add(NoData);
            my_bg2.add(NoData);
            my_bg3.add(NoData);
            my_bg4.add(NoData);
            my_bg5.add(NoData);
        }

        assert_eq!(vert_amt, my_bg1.size());
        assert_eq!(vert_amt, my_bg2.size());
        assert_eq!(vert_amt, my_bg3.size());
        assert_eq!(vert_amt, my_bg4.size());
        assert_eq!(vert_amt, my_bg5.size());

        let mut bg_vec = Vec::<BitGraph<NoData>>::with_capacity(5);
        
        bg_vec.push(my_bg1);
        bg_vec.push(my_bg2);
        bg_vec.push(my_bg3);
        bg_vec.push(my_bg4);
        bg_vec.push(my_bg5);

        let mut nums: [usize; 5] = [0, 0, 0, 0, 0];

        // This is needed because of how the adjustments of edgeverts works in bg.rs
        // The amount of bits must be equivalent to vert_amt / (BITS / EdgeScale)
        // Therefore, since floor(vert_amt / (BITS / EdgeScale)) may be off by 1,
        // it is necessary to add 1 in order to properly compare bg_vec[_].ev_len_at(_).
        if vert_amt % (BITS / 2) != 0 { nums[0] = 1; }
        if vert_amt % (BITS / 4) != 0 { nums[1] = 1; }
        if vert_amt % (BITS / 8) != 0 { nums[2] = 1; }
        if vert_amt % (BITS / 16) != 0 { nums[3] = 1; }
        if vert_amt % (BITS / 32) != 0 { nums[4] = 1; }

        for x in 0..vert_amt {
            // Example: 32 / (64 / 2) + (0 or 1) = 1 
            assert_eq!(vert_amt / (BITS / 2) + nums[0], bg_vec[0].ev_len_at(x)); 
            assert_eq!(vert_amt / (BITS / 4) + nums[1], bg_vec[1].ev_len_at(x)); 
            assert_eq!(vert_amt / (BITS / 8) + nums[2], bg_vec[2].ev_len_at(x)); 
            assert_eq!(vert_amt / (BITS / 16) + nums[3], bg_vec[3].ev_len_at(x));
            assert_eq!(vert_amt / (BITS / 32) + nums[4], bg_vec[4].ev_len_at(x)); 
        }
    }

    #[test]
    fn simple_add_verts1() {
        let mut my_bg1: BitGraph<NoData> = BitGraph::new(EdgeScale::SAME);
        assert_eq!(0, my_bg1.size());
        for _ in 0..200 { my_bg1.add(NoData); }
        assert_eq!(200, my_bg1.size());

        // verifying the amount of edgeverts within each vertex
        // '+ 1' since there is always at least 1 edgevert per vertex
        for x in 0..200 { assert_eq!(200 / BITS + 1, my_bg1.ev_len_at(x)); }
    }

    #[test]
    fn new_bitgraphs() {
        let my_bg1: BitGraph<NoData> = BitGraph::new(EdgeScale::SAME);
        let my_bg2: BitGraph<String> = BitGraph::new(EdgeScale::U8);
        let my_bg3: BitGraph<i8> = BitGraph::new(EdgeScale::U8);
        let my_bg4: BitGraph<Option<i32>> = BitGraph::new_with_capacity(EdgeScale::SAME, 20);
        let my_bg5: BitGraph<Vec::<i32>> = BitGraph::new_with_capacity(EdgeScale::U32, 100);
    }

}
