// TODO: check license for https://github.com/Stefanuk12/axum-protobuf
// this is a modified version of the original code

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use futures_util::StreamExt;

pub struct Protobuf<T>(pub T);

#[allow(unused)]
pub enum ProtobufRejection {
    ProtobufDecodeError(prost::DecodeError),
    FailedToBufferBody,
    MissingProtobufContentType,
}
impl IntoResponse for ProtobufRejection {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ProtobufRejection::ProtobufDecodeError(_) => {
                (StatusCode::BAD_REQUEST, "Protobuf decoding error")
            }
            ProtobufRejection::FailedToBufferBody => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error reading request body",
            ),
            ProtobufRejection::MissingProtobufContentType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Missing 'content-type: application/protobuf' header",
            ),
        };

        Response::builder()
            .status(status)
            .body(Body::from(body))
            .unwrap() // we know this will be valid since we made it
    }
}

#[async_trait]
impl<S, T> FromRequest<S> for Protobuf<T>
where
    T: prost::Message + Default,
    S: Send + Sync,
{
    type Rejection = ProtobufRejection;

    async fn from_request(req: Request<Body>, _: &S) -> Result<Self, Self::Rejection> {
        // if content type exists but is not application/protobuf, reject
        if let Some(content_type) = req.headers().get("content-type") {
            content_type
                .to_str()
                .ok()
                .filter(|value| {
                    *value == "application/protobuf" || *value == "application/octet-stream"
                })
                .ok_or(ProtobufRejection::MissingProtobufContentType)?;
        }

        let mut body = req.into_body().into_data_stream();
        let mut buf = Vec::new();

        while let Some(chunk) = body.next().await {
            let chunk = chunk.map_err(|_| ProtobufRejection::FailedToBufferBody)?;
            buf.extend_from_slice(&chunk);
        }

        T::decode(buf.as_slice())
            .map(|x| Self(x))
            .map_err(ProtobufRejection::ProtobufDecodeError)
    }
}

// fn get_header_value(parts: &Parts, name: &str) -> Option<String> {
//     parts
//         .headers
//         .get(name)
//         .and_then(|v| v.to_str().ok())
//         .map(|v| v.to_string())
// }
