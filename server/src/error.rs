use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

pub type Result<T> = actix_web::Result<T, Error>;
pub type ResponseResult = actix_web::Result<HttpResponse, Error>;

#[non_exhaustive]
#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "Failed to start/stop service '{}': {}", unit, e)]
    ServiceError {
        e: std::io::Error,
        unit: &'static str,
    },

    #[display(fmt = "Failed to blacklist kernel module '{}': {}", module, e)]
    ModuleError {
        e: std::io::Error,
        module: &'static str,
    },

    #[display(fmt = "Could not open file '{}': {}", file, e)]
    FileIOError { e: std::io::Error, file: String },

    #[display(fmt = "Command '{}' did not run successfully: {}", cmd, e)]
    CommandFailed {
        e: std::io::Error,
        cmd: &'static str,
    },

    #[display(fmt = "Command output contained non UTF8 characters.")]
    BadCommandOutput(std::str::Utf8Error),

    #[display(fmt = "Unkown error occured: {}", message)]
    Other { message: &'static str },
}

pub trait ToServiceError {
    type OkVariant;
    fn map_service(self, unit: &'static str) -> std::result::Result<Self::OkVariant, Error>;
}

impl<T> ToServiceError for std::result::Result<T, std::io::Error> {
    type OkVariant = T;
    fn map_service(self, unit: &'static str) -> std::result::Result<Self::OkVariant, Error> {
        self.map_err(|e| Error::ServiceError { e, unit })
    }
}

pub trait ToFileIOError {
    type OkVariant;
    fn map_fileio(self, file: impl AsRef<str>) -> std::result::Result<Self::OkVariant, Error>;
}

impl<T> ToFileIOError for std::result::Result<T, std::io::Error> {
    type OkVariant = T;
    fn map_fileio(self, file: impl AsRef<str>) -> std::result::Result<Self::OkVariant, Error> {
        self.map_err(|e| Error::FileIOError {
            e,
            file: file.as_ref().to_string(),
        })
    }
}

pub trait ToModuleError {
    type OkVariant;
    fn map_module(self, module: &'static str) -> std::result::Result<Self::OkVariant, Error>;
}

impl<T> ToModuleError for std::result::Result<T, std::io::Error> {
    type OkVariant = T;
    fn map_module(self, module: &'static str) -> std::result::Result<Self::OkVariant, Error> {
        self.map_err(|e| Error::ModuleError { e, module })
    }
}

pub trait ToCommandError {
    type OkVariant;
    fn map_command(self, cmd: &'static str) -> std::result::Result<Self::OkVariant, Error>;
}

impl<T> ToCommandError for std::result::Result<T, std::io::Error> {
    type OkVariant = T;
    fn map_command(self, cmd: &'static str) -> std::result::Result<Self::OkVariant, Error> {
        self.map_err(|e| Error::CommandFailed { e, cmd })
    }
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(value: std::str::Utf8Error) -> Self {
        Error::BadCommandOutput(value)
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::Other { message: value }
    }
}
