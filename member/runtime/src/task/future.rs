use std::future::Future;

/// 封装传入的future -- 内部重写poll函数由轮询线程调用
pub struct FutureWrapper<T> {
    future: *mut (dyn Future<Output=T> + 'static),
}


pub fn print_test(){
    println!("------- print_test -------");
}