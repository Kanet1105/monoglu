use std::future::Future;

pub fn dispatch<F, P, R>(data: P, handler: F) 
where 
    F: FnOnce(P) -> R + Send + 'static,
    P: Clone + Send + Sync + 'static,
    R: Future + Send + 'static,
{
    tokio::spawn(async move {
        handler(data).await;
    });
}
