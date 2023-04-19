use std::arch::asm;

macro_rules! MARKER {
    ($num:literal) => {
        unsafe { asm!(concat!("add rax, 0xabcd", $num)) };
    };
}

#[inline(never)]
#[no_mangle]
async fn func1() -> usize {
    MARKER!(0);
    1
}

#[inline(never)]
#[no_mangle]
async fn func2() -> usize {
    MARKER!(1);
    func3().await;
    MARKER!(2);
    2
}

#[inline(never)]
#[no_mangle]
async fn func3() -> usize {
    MARKER!(3);
    3
}

#[inline(never)]
#[no_mangle]
async fn func4() -> usize {
    MARKER!(4);
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    MARKER!(5);
    let b = func1().await;
    MARKER!(6);
    let c = 3;
    let d = func2().await;
    MARKER!(7);
    let e = 5;
    MARKER!(8);
    let f = func3().await;
    let ret = a.iter().sum::<usize>() + b + c + d + e + f;
    MARKER!(9);
    ret
}

#[inline(never)]
#[no_mangle]
async fn foo() {
    MARKER!('a');
    let a = {
        let arr = [0, 1, 2, 3, 4];
        MARKER!('b');
        let b = func1().await;
        MARKER!('c');
        let c = func2().await;
        MARKER!('d');
        b + c + arr[3]
    };
    MARKER!('e');
    let b = func4().await;
    MARKER!('f');
    let _ = a + b + func3().await;
}

fn main() {
    executor::spawn(foo());
    executor::run_until_idle();
}
