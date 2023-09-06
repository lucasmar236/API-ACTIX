use crate::domain::error::RespError;
use std::error::Error;

pub struct ErrorUtils {}

impl ErrorUtils{
    fn log_error(message: &str, err: &Option<Box<dyn Error>>){
        println!("Error : {}",message);
        if let Some(error)=  err{
            println!("Stack : {}", error.to_string())
        }
    }


    pub fn bad_request_error(err_message: &str, error: Option<Box<dyn Error>>) -> RespError{
        ErrorUtils::log_error(err_message,&error);
        RespError{
            status: 400,
            message: String::from(err_message),
            error: None
        }
    }

    pub fn unauthorized_error() -> RespError{
        let error = "Error : Token expired or not authenticated";
        ErrorUtils::log_error(error,&None);
        RespError{
            status : 401,
            message : String::from(error),
            error: None
        }
    }

    pub fn forbidden_error() -> RespError {
        let error = "Error : Not allowed";
        ErrorUtils::log_error(error,&None);
        RespError{
            status : 403,
            message : String::from(error),
            error: None
        }
    }
}