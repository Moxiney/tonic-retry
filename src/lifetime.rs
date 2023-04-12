use std::future::Future;

trait FnHelper<'i, 'o>
where
    'i: 'o,
{
    type Output: 'o;
    type Fut: Future<Output = Self::Output> + 'o;
    fn call(&self, arg: &'i str) -> Self::Fut;
}

impl<'i, 'o, FO: 'o, F, O: 'o> FnHelper<'i, 'o> for F
where
    'i: 'o,
    F: Fn(&'i str) -> FO,
    FO: Future<Output = O>,
{
    type Output = O;
    type Fut = FO;
    fn call(&self, arg: &'i str) -> FO {
        self(arg)
    }
}

async fn print_func_result<'i, F>(s: &'i str, func: F) -> <F as FnHelper<'_, '_>>::Output
where
    for<'o> F: FnHelper<'i, 'o>,
{
    let _s1 = func.call(s).await;
    let s2 = func.call(s).await;
    s2
    // println!("{}", s1);
}

async fn accept_me(s: &str) -> &str {
    println!("accept {}", s);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn main() {
        let res = print_func_result("test", accept_me).await;
        println!("get result {}", res);
    }
}
