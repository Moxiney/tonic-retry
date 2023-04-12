use std::future::Future;

trait FnHelper<'o, I> {
    type Output: 'o;
    type Fut: Future<Output = Result<Self::Output, ()>> + 'o;
    fn call(&self, arg: I) -> Self::Fut;
}

impl<'o, FO: 'o, F, O: 'o, I: 'o> FnHelper<'o, I> for F
where
    F: Fn(I) -> FO,
    FO: Future<Output = Result<O, ()>>,
{
    type Output = O;
    type Fut = FO;
    fn call(&self, arg: I) -> Self::Fut {
        self(arg)
    }
}

async fn print_func_result<'o, F, I>(s: I, func: F) -> Result<<F as FnHelper<'o, I>>::Output, ()>
where
    F: FnHelper<'o, I>,
    I: Copy,
{
    if let Ok(s1) = func.call(s).await {
        return Ok(s1);
    }
    let s2 = func.call(s).await;
    s2
}

async fn accept_me(s: &str) -> Result<&str, ()> {
    println!("accept {}", s);
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn main() {
        let res = print_func_result("test", accept_me).await;
    }
}
