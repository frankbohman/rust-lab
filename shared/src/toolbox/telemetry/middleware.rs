// use http::{Request, Response};
// use tower::{Layer, Service};
// #[derive(Default, Debug, Clone)]
// pub struct OtelGrpcLayer {}

// impl OtelGrpcLayer {
// }

// #[derive(Debug, Clone)]
// pub struct OtelGrpcService<S> {
//   inner: S,
// }

// impl<S> Layer<S> for OtelGrpcLayer {
//   type Service = OtelGrpcService<S>;

//   fn layer(&self, inner: S) -> Self::Service {
//     OtelGrpcService {
//       inner,
//     }
//   }
// }

// impl<S, REQ, RES> Service<REQ> for OtelGrpcService<S>
// where
//   S: Service<Request<REQ>, Response = Response<RES>, Error = BoxError> + Clone + Send + 'static,
//   S::Future: Send + 'static,
//   REQ: Send + 'static,
// {
//   type Error = S::Error;
//   type Future = ResponseFuture<S::Future>;
//   type Response = S::Response;

//   fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
//     todo!()
//   }

//   fn call(&mut self, req: REQ) -> Self::Future { todo!() }
// }
