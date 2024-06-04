use shared::{proto::echo::{echo_server, EchoReply, EchoRequest}, tonic}
#[derive(Clone)]
pub struct EchoService {}


#[tonic::async_trait]
impl echo_server::Echo for EchoService {
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn echo(&self,request:tonic::Request<EchoRequest> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = std::result::Result<tonic::Response<super::EchoReply> ,tonic::Status> > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }
}
