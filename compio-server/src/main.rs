use compio::dispatcher::Dispatcher;
use compio::net::{TcpListener, TcpStream};
use compio::runtime::spawn;
use compio::BufResult;
use config::{ServerConfig, PACKET_SIZE};
use futures_util::{stream::FuturesUnordered, StreamExt};
use std::num::NonZeroUsize;
use std::sync::Arc;
use compio::io::{AsyncRead, AsyncWrite};

#[compio::main]
async fn main() {
    let cfg = Arc::new(ServerConfig::parse());
    let nr_cores = **&cfg.cores.last().unwrap() as usize;
    println!(
        "Running ping pong server with Compio.\nPacket size: {}\nListen {}\nCPU slot: {}",
        PACKET_SIZE,
        cfg.bind,
        config::format_cores(&cfg.cores)
    );

    let listener = TcpListener::bind(&cfg.bind).await.unwrap();
    let dispatcher = Dispatcher::builder()
        .worker_threads(NonZeroUsize::new(nr_cores).unwrap())
        .build()
        .unwrap();

    spawn(async move {
        let mut futures = FuturesUnordered::from_iter((0..nr_cores).map(|_| {
            let addr = &cfg.bind;
            async move {
                let _stream = TcpStream::connect(addr).await.unwrap();
                // now what?
            }
        }));
        while let Some(()) = futures.next().await {}
    })
    .detach();

    let mut handles = FuturesUnordered::new();
    for _i in 0..nr_cores {
        let (mut srv, _) = listener.accept().await.unwrap();
        let handle = dispatcher
            .dispatch(move || async move {
                // read from client
                let BufResult(res, buf) = srv.read(Vec::with_capacity(PACKET_SIZE)).await;
                res.unwrap();

                //  send back to client a.k.a echo
                srv.write(buf).await.unwrap();

            })
            .unwrap();
        handles.push(handle);
    }

    while handles.next().await.is_some() {}

    dispatcher.join().await.unwrap();
}
