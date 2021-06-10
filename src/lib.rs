mod itb;
mod bg;
use crate::bg::bit_graph::*;
use crate::BitGraph;
use std::thread;
use std::thread::JoinHandle;

#[cfg(test)]
mod tests {
    
    use super::*;
    #[derive(Clone)]
    struct NoData;
    const BITS: usize = std::mem::size_of::<usize>() * 8;

    #[test]
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

    #[test] 
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

        for cap in 1..1_000 {

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
}
