use crate::domain::{Title, Body};

#[derive(Debug)]
pub struct CreatePage {
    title: String,
    body: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Page {
    title: Title,
    body: Body,
}

pub trait CreatePageGateway {
    fn create_page(&mut self, _: Page) -> Result<(), &'static str>;
}

pub fn create_page(mut gateway: impl CreatePageGateway, request: CreatePage) -> Result<(), &'static str> {
    let title = Title::validate(request.title)?;
    let body = Body::validate(request.body)?;
    let page = Page {
        title,
        body,
    };

    gateway.create_page(page)
}


#[cfg(test)]
mod tests {
    use super::*;

    struct BlackHoleGateway;
    impl CreatePageGateway for BlackHoleGateway {
        fn create_page(&mut self, _: Page) -> Result<(), &'static str> {
            Ok(())
        }
    }

    #[test]
    fn title_cannot_be_empty() {
        let result = create_page(BlackHoleGateway, CreatePage {
            title: "".to_owned(),
            body: "foo".to_owned(),
        });
        assert_eq!(Err("Title cannot be empty"), result);
    }

    #[test]
    fn title_cannot_be_too_long() {
        let bit_too_long = create_page(BlackHoleGateway, CreatePage {
            title: "a".repeat(256),
            body: "foo".to_owned(),
        });
        assert_eq!(Err("Title cannot be over 255 characters"), bit_too_long);

        let just_barely_ok = create_page(BlackHoleGateway, CreatePage {
            title: "a".repeat(255),
            body: "foo".to_owned(),
        });
        assert_eq!(Ok(()), just_barely_ok);
    }

    #[test]
    fn body_cannot_be_empty() {
        let result = create_page(BlackHoleGateway, CreatePage {
            title: "foo".to_owned(),
            body: "".to_owned(),
        });
        assert_eq!(Err("Body cannot be empty"), result);
    }
}
