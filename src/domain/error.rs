use  std::{error::Error,fmt};

#[derive(Debug)]
pub struct RespError{
    pub status: u32,
    pub message: String,
    pub error : Option<Box<dyn Error>>
}

impl fmt::Display for RespError{
    fn fmt(&self,formatter: &mut fmt::Formatter) -> fmt::Result  {
        write!(formatter,"Error, please try again")
    }
}


impl RespError{
    pub fn get_status_error(&self) -> u32{
        self.status
    }

    pub fn get_message_error(&self) -> String{
        String::from(&self.message)
    }
}