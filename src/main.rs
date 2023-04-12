use std::future::Future;

trait FnHelper<'a, I: ?Sized, O> {
    type Output: Future<Output = O> + 'a;
    fn call(&self, arg: &'a mut I) -> Self::Output;
}

impl<'a, I: 'a, O, D: 'a, F> FnHelper<'a, I, O> for F
where
    I: ?Sized,
    F: Fn(&'a mut I) -> D,
    D: Future<Output = O>,
{
    type Output = D;
    fn call(&self, arg: &'a mut I) -> D {
        self(arg)
    }
}

async fn print_func_result<F, I, O>(func: F, input: &mut I) -> O
where
    for<'a> F: FnHelper<'a, I, O>,
    O: Default,
    I: ?Sized,
{
    let mut result = O::default();
    for i in 0..5 {
        result = func.call(input).await;
    }
    result
}

async fn accept_me<'a>(s: &'a mut str) -> String {
    println!("accept_me {}", s);
    s.to_owned()
}

async fn accept_i32<'a>(i: &'a mut i32) -> i32 {
    println!("accept_i32 {}", i);
    1
}

fn reject_me(s: &'static str) -> &'static str {
    s
}

#[tokio::main]
async fn main() {
    let mut val = "".to_string();
    let result = print_func_result(accept_me, &mut val).await;

    let result = print_func_result(accept_i32, &mut 1).await;

    // print_func_result(reject_me);
}
