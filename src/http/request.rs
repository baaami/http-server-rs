use super::method::Method;

pub struct Request {
    path: String,
    query_string: String,
    // super는 Parent Module를 참조할 수 있음
    method: Method,
}