// use std::future::Future;
//
// pub struct FutureObj<R>(FutureWrapper<R>);
//
// /// 封装传入的future -- 内部重写poll函数由轮询线程调用
// /// R: future result type
// pub struct FutureWrapper<R> {
//     future: *mut (dyn Future<Output=R> + 'static),
// }

pub fn print_test() -> i32 {
    println!("------- print_test -------");
    37
}
