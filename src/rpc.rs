use std::future::Future;

trait RpcMethod<'a, Client: ?Sized, Req, Resp> {
    type Error;
    type CallFut: Future<Output = Result<Resp, Self::Error>> + 'a;
    fn call(&self, client: &'a mut Client, req: Req) -> Self::CallFut;
}

impl<'a, Client, Req, Resp, Err, Fut, F> RpcMethod<'a, Client, Req, Resp> for F
where
    Client: ?Sized + 'a,
    F: Fn(&'a mut Client, Req) -> Fut,
    Fut: Future<Output = Result<Resp, Err>> + 'a,
{
    type Error = Err;
    type CallFut = Fut;
    fn call(&self, client: &'a mut Client, req: Req) -> Self::CallFut {
        self(client, req)
    }
}

async fn retry<F, Client, Req, Resp>(
    f: F,
    client: &mut Client,
    req: Req,
    times: usize,
) -> Result<Resp, ()>
where
    for<'a> F: RpcMethod<'a, Client, Req, Resp>,
    Client: ?Sized,
    Req: Clone,
{
    for _ in 0..times {
        if let Ok(res) = f.call(client, req.clone()).await {
            return Ok(res);
        }
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Client;

    async fn hello(client: &mut Client, req: String) -> Result<String, ()> {
        Ok(format!("hello {}", req))
    }

    #[tokio::test]
    async fn retry_test() {
        let mut client = Client;
        let req = String::from("world");

        assert!(retry(hello, &mut client, req.clone(), 5).await.is_ok());

        let fall_rpc = |_: &mut Client, _: String| async move {
            let res: Result<String, ()> = Err(());
            res
        };
        assert!(retry(fall_rpc, &mut client, req, 5).await.is_err());
    }
}
