#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Title(String);

impl Title {
    pub fn validate(title: String) -> Result<Title, &'static str> {
        if title.is_empty() {
            return Err("Title cannot be empty");
        }
        if title.len() > 255 {
            return Err("Title cannot be over 255 characters");
        }
        Ok(Title(title))
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Body(String);

impl Body {
    pub fn validate(body: String) -> Result<Body, &'static str> {
        if body.is_empty() {
            return Err("Body cannot be empty");
        }
        Ok(Body(body))
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct Page {
    pub title: Title,
    pub body: Body,
}
