use std::fmt::Display;
use axum::http::StatusCode;

/// simple helper function to map unexpected errors
/// into an axum response with `500 HTTP Status code` and displaying the error message
pub fn map_err<E>(err: E) -> (StatusCode, String)
where
    E: Display
{
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {}", err))
}

/// a macro for boilerplate wrapper code around
/// the actual image processing functionality for that endpoint
///
/// it takes in `function` assumed to be a function and then
/// a struct to deserialize optional query arguments into.
/// Since macros cannot have optional arguments, simply use [`models::NoArgs`] to represent no arguments
///
#[macro_export]
macro_rules! wrap_fn {
    ( $function:expr, $query:ty ) => {
        |axum::extract::Query(query): axum::extract::Query<$query>,
        mut multipart: axum::extract::Multipart|
            async move {
                if let Some(mut field) = multipart.next_field()
                    .await
                    .map_err(wrapper::map_err)?
                {

                    /*if !field.content_type()
                        .map_or(false, |c| c.starts_with("image/"))
                    {
                        return Err((
                            StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            "Only media of `image/` content type are supported".to_string()
                        ));
                    }*/

                    let mut size = 0;
                    let mut buffer = Vec::<u8>::new();

                    while let Some(chunk) = field.chunk()
                        .await
                        .map_err(wrapper::map_err)?
                    {
                        size += chunk.len();

                        if size > MAX_IMAGE_SIZE {
                            return Err((
                                StatusCode::PAYLOAD_TOO_LARGE,
                                format!("The size of the image provided is {} bytes which exceeds the limit of {} bytes", size, MAX_IMAGE_SIZE)
                            ));
                        }

                        buffer.extend_from_slice(&chunk);
                    }

                    let bytes = tokio::task::spawn_blocking(
                        move || -> ril::Result<Vec<u8>> {
                            let image = ImageSequence::<Rgba>::from_bytes_inferred(&*buffer)?
                                .into_sequence()?
                                .into_first_image();

                            let image = $function(image, query);
                            let mut bytes = Vec::<u8>::new();
                            image.encode(ImageFormat::Png, &mut bytes)?;

                            Ok(bytes)
                        }
                    )
                        .await
                        .map_err(wrapper::map_err)?
                        .map_err(wrapper::map_err)?;

                    Ok((axum::TypedHeader(axum::headers::ContentType::png()), bytes))
                } else {
                    Err((
                        StatusCode::BAD_REQUEST,
                        "Missing required multipart field for image bytes".to_string(),
                    ))
                }
            }
    }
}