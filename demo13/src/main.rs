// alloc, dealloc,Layout
use std::alloc::{alloc,dealloc,Layout};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>>{
    println!("Memory Model: ");

    // Layout 内存模型
    let l1= Layout::from_size_align(1,2)?; // 申请一个内存空间 对齐方式2字节
    let l2 = Layout::from_size_align(2,2)?;

    println!("l1: {:?}",l1);
    println!("l1: {:?}",l2);

    // Auto Layout
    let l3 = Layout::new::<u8>();
    println!("l3: {:?}",l3);

    // alloc 分配内存
    unsafe {
        let lay1 = Layout::new::<u8>();
        let prt1 = alloc(lay1);
        println!("prt1 = {:?}",prt1);
        *prt1 = 12;
        println!("prt1 = {}",*prt1);
        dealloc(prt1,lay1); // 回收内存
    }


    Ok(())
}

// 内存模型
// struct Layout {
//     size_: usize, // 内存大小
//     allign_: usize, // 内存对齐1字节2字节
// }
