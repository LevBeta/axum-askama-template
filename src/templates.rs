use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response}
};

//INDEX

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

//HTML TEMPLATE

struct HtmlTemplate<T>(T);

impl <T> IntoResponse for HtmlTemplate<T> 
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render, error: {}", err)
            ).into_response()
        }
    }
}


