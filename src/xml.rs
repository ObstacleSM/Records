use actix_web::HttpResponse;
use serde::Serialize;

pub fn to_string<T>(elements: Vec<T>) -> String
where
    T: Serialize,
{
    let mut result = String::with_capacity(59);
    result.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    result.push_str("<response>");

    if !elements.is_empty() {
        let first_serialized = serde_xml_rs::to_string(elements.first().unwrap()).unwrap();
        // Reserve the right size in advance, the parameter is additional len so we add to add the </response> too
        result.reserve(first_serialized.len() * elements.len() + 11);

        for record in elements {
            result.push_str(&serde_xml_rs::to_string(&record).unwrap());
        }
    }

    result.push_str("</response>");
    result
}

pub fn xml_response(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/xml; charset=utf-8")
        .body(&body)
}
