#[macro_export]
macro_rules! hash {
    ($($k:expr , $v:expr);*)=>{
        {
                let mut h = std::collections::HashMap::new();
                $(
                    h.insert($k,$v);
                )*
                h
        }
    };
    ($($k:expr , $v:expr;)*)=>(hash!($($k , $v);*));
}

fn main() {
    let h = hash!("a",1;"b",2;);
    for (k,v) in h.iter(){
        println!("{},{}", k,v);
    }
}
