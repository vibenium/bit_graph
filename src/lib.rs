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

        terminology: vXevY is a way to define an edgevert number
        where 'X' is the index of the 'vertex' and 'Y' is the
        index of the 'edgevert'. For example, v0ev2 means get
        the edgevert of vertex[0] at edgevert[2].

    */
    use super::*;

    #[derive(Clone)]
    struct NoData;

    const BITS: usize = std::mem::size_of::<usize>() * 8;

    #[test]
    fn complex_remove1() { // testing for connections across 2 edgeverts (0 to 1)
        let mut bg_same: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::SAME, BITS * 2);
        let mut bg_binary: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::BINARY, BITS * 2);
        let mut bg_u4: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U4, BITS * 2);

        for _ in 0..(BITS * 2) { bg_same.add(NoData); }
        for _ in 0..BITS { bg_binary.add(NoData); }
        for _ in 0..(BITS / 2) { bg_u4.add(NoData); } 
       
        // sanity check
        assert_eq!(BITS * 2, bg_same.size());
        assert_eq!(BITS, bg_binary.size());
        assert_eq!(BITS / 2, bg_u4.size());

        // middle ev0 to middle ev1
        bg_same.connect(BITS / 2 - 1, (BITS * 3) / 2 - 1, 0);
        bg_binary.connect(BITS / 4 - 1, (BITS * 3) / 4 - 1, 1); 
        bg_u4.connect(BITS / 8 - 1, (BITS * 3) / 8 - 1, 7);

        // another sanity check
        assert!(bg_same.is_connected(BITS / 2 - 1, (BITS * 3) / 2 - 1));
        assert!(bg_binary.is_connected(BITS / 4 - 1, (BITS * 3) / 4 - 1));
        assert!(bg_u4.is_connected(BITS / 8 - 1, (BITS * 3) / 8 - 1));

        // where the fun begins (i.e., removing the middle bits at ev1)
        bg_same.remove((BITS * 3) / 2 - 1);
        bg_binary.remove((BITS * 3) / 4 - 1);
        bg_u4.remove((BITS * 3) / 8 - 1);

        assert_eq!(bg_same.ev_num_at(BITS / 2 - 1, 1), 0);
        assert_eq!(bg_binary.ev_num_at(BITS / 4 - 1, 1), 0);
        assert_eq!(bg_u4.ev_num_at(BITS / 8 - 1, 1), 0);

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

    #[test] // test if not connected
    fn simple_is_connected_test4() {
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
    fn simple_is_connected_test3() {
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
    fn simple_is_connected_test2() {
        let mut bg_u8: BitGraph<NoData> = BitGraph::new_with_capacity(EdgeScale::U8, 500);
        for _ in 0..500 { bg_u8.add(NoData); }
        for x in 0..500 { bg_u8.connect(0, x, 127); }
        for x in 0..500 { 
            println!("{} passed for simple_is_connected_test2()", x);
            assert!(bg_u8.is_connected(0, x)); 
        }
    }

    #[test]
    fn simple_is_connected_test1() {
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

    /*
    #[test]
    fn test_itb8() {
        let str1 = itb::int_to_bit8(&0);
        assert_eq!("00000000", str1);
        let str2 = itb::int_to_bit8(&0xff); // 255
        assert_eq!("11111111", str2);
        let str3 = itb::int_to_bit8(&127);
        assert_eq!("01111111", str3);
        let str4 = itb::int_to_bit8(&170);
        assert_eq!("10101010", str4);
    }
    #[test]
    fn test_itb16() {
        let str1 = itb::int_to_bit16(&0);
        assert_eq!("0000000000000000", str1);
        let str2 = itb::int_to_bit16(&0xffff);
        assert_eq!("1111111111111111", str2);
    }
    #[test]
    fn test_itb32() {
        let str1 = itb::int_to_bit32(&0);
        assert_eq!("00000000000000000000000000000000", str1);
        let str2 = itb::int_to_bit32(&0xffffffff);
        assert_eq!("11111111111111111111111111111111", str2);
    }
    #[test]
    fn test_itb64() {
        let str1 = itb::int_to_bit64(&0);
        assert_eq!
        (
            "0000000000000000000000000000000000000000000000000000000000000000",
            str1
        );
        let str2 = itb::int_to_bit64(&0xffffffffffffffff);
        assert_eq!
        (
            "1111111111111111111111111111111111111111111111111111111111111111",
            str2
        );
    }
    */
}
