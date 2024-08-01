use team_runtime::task::future::print_test;

#[test]
fn test1() {
    println!("------- test main -------");
    let result: i32 = print_test();
    assert_eq!(result, 37);
}
