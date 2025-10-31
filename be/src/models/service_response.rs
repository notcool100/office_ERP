use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorItem {
    pub code: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ServiceResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
    pub errors: Vec<ErrorItem>,
    pub count: Option<usize>,
}

impl<T> ServiceResponse<T>
where
    T: Serialize,
{
    pub fn builder() -> ServiceResponseBuilder<T> {
        ServiceResponseBuilder {
            success: true,
            message: None,
            data: None,
            errors: vec![],
            count: None,
        }
    }
}

#[derive(Serialize)]
pub struct ServiceResponseBuilder<T>
where
    T: Serialize,
{
    success: bool,
    message: Option<String>,
    data: Option<T>,
    errors: Vec<ErrorItem>,
    count: Option<usize>,
}

impl<T> ServiceResponseBuilder<T>
where
    T: Serialize,
{
    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn error(mut self, code: impl Into<String>, message: impl Into<String>) -> Self {
        self.errors.push(ErrorItem {
            code: code.into(),
            message: message.into(),
        });
        self
    }

    pub fn errors(mut self, errors: Vec<ErrorItem>) -> Self {
        self.errors = errors;
        self
    }

    pub fn count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }

    pub fn build(self) -> ServiceResponse<T> {
        ServiceResponse {
            success: self.success,
            message: self.message,
            data: self.data,
            errors: self.errors,
            count: self.count,
        }
    }
}
