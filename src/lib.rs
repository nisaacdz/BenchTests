use std::{sync::Arc, time::Duration};

pub mod speed;

pub fn multiple_threads(nums: &Vec<Arc<Vec<Vec<Vec<i32>>>>>) {
    let exec = thread_runner_l::execs::FixedThreadPool::new(4);
    for i in nums.iter() {
        let arc = Arc::clone(i);
        exec.execute(move || {
            let mut _ans = 0;
            for a in arc.iter() {
                for b in a.iter() {
                    for c in b.iter() {
                        _ans += *c as i128;
                    }
                }
            }
        });
    }
    exec.join();
}

pub fn multiple_threads_l(nums: &Vec<Arc<Vec<Vec<Vec<i32>>>>>) {
    let exec = thread_runner_l::execs::FixedThreadPool::new(4);
    for i in nums.iter() {
        let arc = Arc::clone(i);
        exec.execute(move || {
            let mut _ans = 0;
            for a in arc.iter() {
                for b in a.iter() {
                    for c in b.iter() {
                        _ans += *c as i128;
                    }
                }
            }
        });
    }
    exec.join();
}

pub fn single_thread(nums: &Vec<Arc<Vec<Vec<Vec<i32>>>>>) {
    let mut _sum = 0;
    for i in nums.iter() {
        for j in i.iter() {
            for k in j.iter() {
                for l in k.iter() {
                    _sum += *l as i128;
                }
            }
        }
    }
}

use crate::speed::BenchTester;

use rand::Rng;
const RNG: std::ops::Range<i32> = i32::MIN..i32::MAX;
const DEP: usize = 30;

static mut ITEMS: Vec<Arc<Vec<Vec<Vec<i32>>>>> = vec![];

pub fn init() {
    for _ in 0..DEP {
        for _ in 0..DEP {
            let mut res2 = vec![];
            for _ in 0..DEP {
                let mut res3 = vec![];
                for _ in 0..DEP {
                    let mut res4 = vec![];
                    for i in 0..DEP {
                        res4.push(rand::thread_rng().gen_range(RNG))
                    }
                    res3.push(res4);
                }
                res2.push(res3);
            }
            unsafe {
                ITEMS.push(Arc::new(res2));
            }
        }
    }
}

pub fn bench_all() {
    let bencher = BenchTester::new(5);
    init();
    println!("Initialized");
    assert_ne!(unsafe { ITEMS.len() }, 0);
    println!("Benching Multithread_l ... ");
    bencher.bench(|| multiple_threads_l(unsafe { &ITEMS }));

    println!("Benching Multithread ... ");
    bencher.bench(|| multiple_threads(unsafe { &ITEMS }));

    println!("Benching Singlethread ... ");
    bencher.bench(|| single_thread(unsafe { &ITEMS }));
}
