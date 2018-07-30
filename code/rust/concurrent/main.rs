// From 123hc at rust.cc, https://rust.cc/article/5db352ac-3e81-4bd0-81f3-61581b2e4328
extern crate crossbeam;
extern crate num_cpus;
extern crate rand;
extern crate rayon;
use rayon::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
fn main() {
    //可共享可更改
    let data_w = Arc::new(Mutex::new(vec![0; 10]));
    //可共享不可更改
    //let data_r = Arc::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    //消息通道
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let (data_w, tx) = (data_w.clone(), tx.clone());
        thread::spawn(move || {
            let x = rand::random::<i32>() % 220;
            let start = Instant::now();
            {
                let mut data_w = data_w.lock().unwrap();
                data_w[i] = x;
            }
            let elapsed = start.elapsed();
            tx.send((i, start, elapsed)).unwrap();
        });
    }
    //关闭通道
    drop(tx);
    for _ in 0..10 {
        let x = rx.recv().unwrap();
        println!("{:?}", x);
    }
    println!("{:?}", data_w);

    // 用rayon
    //自定义线程池
    let _pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get() + 0)
        .build()
        .unwrap();
    let r = rand::random::<i32>() % 20;
    println!("random:{:?}", r);
    let mut data_p = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    data_p.par_iter_mut().for_each(|x| *x *= *x + r);
    println!("data_p: {:?}", data_p);

    //使用crossbeam mpmc
    let (s, r) = crossbeam::channel::unbounded();
    for i in 0..10 {
        let s = s.clone();

        thread::spawn(move || {
            s.send(i);
        });
    }

    for i in 0..10 {
        let r = r.clone();

        thread::spawn(move || {
            let x = r.recv().unwrap();

            println!("r{} << s{:?}", i, x);
        });
    }
    //主线程等待1秒
    thread::sleep(Duration::from_secs(1));
}
