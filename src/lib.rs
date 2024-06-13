pub use wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};
use wasi_async_runtime::Reactor;

#[wasi_http_attributes::main]
async fn main(_reactor: Reactor, request: IncomingRequest, outparam: ResponseOutparam) {
    let max_width = 80;

    let body = request.consume().unwrap();
    let stream = body.stream().unwrap();
    let msg = stream.read(max_width).unwrap_or_default();
    let msg = String::from_utf8_lossy(&msg);
    drop(stream);

    let hdrs = Fields::new();
    let resp = OutgoingResponse::new(hdrs);
    let body = resp.body().expect("outgoing response");

    ResponseOutparam::set(outparam, Ok(resp));

    let out = body.write().expect("outgoing stream");
    let mut output = Vec::new();
    ferris_says::say(&msg, max_width as usize, &mut output).unwrap();
    out.blocking_write_and_flush(&output)
        .expect("writing response");

    drop(out);
    OutgoingBody::finish(body, None).unwrap();
}
