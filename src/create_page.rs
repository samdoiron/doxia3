#[derive(Debug)]
pub struct CreatePage {
    title: String,
    body: String,
}

pub fn create_page(request: CreatePage) -> Result<(), &'static str> {
    if request.title.is_empty() {
        return Err("Title cannot be empty");
    }
    if request.title.len() > 255 {
        return Err("Title cannot be over 255 characters");
    }
    if request.body.is_empty() {
        return Err("Body cannot be empty");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_cannot_be_empty() {
        let result = create_page(CreatePage {
            title: "".to_owned(),
            body: "foo".to_owned(),
        });
        assert_eq!(Err("Title cannot be empty"), result);
    }

    #[test]
    fn title_cannot_be_too_long() {
        let bit_too_long = create_page(CreatePage {
            title: "a".repeat(256),
            body: "foo".to_owned(),
        });
        assert_eq!(Err("Title cannot be over 255 characters"), bit_too_long);

        let just_barely_ok = create_page(CreatePage {
            title: "a".repeat(255),
            body: "foo".to_owned(),
        });
        assert_eq!(Ok(()), just_barely_ok);
    }

    #[test]
    fn body_cannot_be_empty() {
        let result = create_page(CreatePage {
            title: "foo".to_owned(),
            body: "".to_owned(),
        });
        assert_eq!(Err("Body cannot be empty"), result);
    }
}
