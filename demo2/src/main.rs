use {
    hyper:: {
        // hyper 通过一下函数 处理 请求
        service::{make_service_fn,service_fn},
        // http 请求处理 结构体
        Body,
        Client,
        Request,
        Response,
        Server,
        Uri,
    },
    std::{
        net::SocketAddr,
        str::FromStr,
    },
};


// simple http func
async fn hello_world(req: Request<Body>) -> Result<Response<Body>,hyper::Error> {
    let url_str = format!("http://www.baidu.com{:?}", req.uri());
    let url = url_str.parse::<Uri>().expect("failed to parse URL");
    let res = Client::new().get(url).await?;
    // Ok(Response::new(Body::from("Hello World!")))
    Ok(res)
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}",addr);
    let server_future = Server::bind(&addr)
        .serve(make_service_fn(|_| async {
            Ok::<_,hyper::Error>(service_fn(hello_world))
        }));

    if let Err(e) = server_future.await {
        eprintln!("Server error: {}",e);
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from_str("0.0.0.0:8081").expect("fuck addr error");
    run_server(addr).await;
}
