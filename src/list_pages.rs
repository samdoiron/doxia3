use crate::domain::{Title, Body};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListedPage {
    pub title: Title,
    pub body: Body,
}

pub trait ListPagesGateway {
    fn list_pages(&self) -> Result<Vec<ListedPage>, &'static str>;
}

pub fn list_pages(gateway: impl ListPagesGateway) -> Result<Vec<ListedPage>, &'static str> {
    gateway.list_pages()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StubListPagesGateway {
        response: Vec<ListedPage>
    }

    impl ListPagesGateway for StubListPagesGateway {
        fn list_pages(&self) -> Result<Vec<ListedPage>, &'static str> {
            Ok(self.response.clone())
        }
    }

    #[test]
    fn it_works() {
        let canned_response = vec![
            ListedPage {
                title: Title::validate("Page title 1".to_owned()).unwrap(),
                body: Body::validate("Page body 1".to_owned()).unwrap(),
            },
            ListedPage {
                title: Title::validate("Page title 2".to_owned()).unwrap(),
                body: Body::validate("Page body 2".to_owned()).unwrap(),
            }
        ];

        let gateway = StubListPagesGateway {
            response: canned_response.clone()
        };

        let result = list_pages(gateway);
        assert_eq!(Ok(canned_response), result);
    }
}
