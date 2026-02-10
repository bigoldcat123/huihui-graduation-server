use faithea::data::Json;
use serde::{Deserialize, Serialize};

use crate::service::error::ServiceError;

pub mod input;
pub mod output;
pub mod raw;

#[derive(Serialize,Deserialize,Debug)]
pub struct ApiResponse<T:Serialize> {
    code:i32,
    message:String,
    data:Option<T>
}
impl <T:Serialize> From<Result<T,ServiceError>> for ApiResponse<T> {
    fn from(value: Result<T,ServiceError>) -> Self {
        match value {
            Ok(r) => {
                Self::ok().data(r)
            }
            Err(e) => {
                Self::err(format!("{:?}",e))
            }
        }
    }
}
impl <T:Serialize> ApiResponse<T> {
    pub fn ok() -> Self {
        ApiResponse {
            code:200,
            message:"ok".to_string(),
            data:None
        }
    }
    pub fn err(message:String) -> Self {
        ApiResponse {
            code:500,
            message:message,
            data:None
        }
    }
    pub fn data(mut self,data:T) -> Self {
        self.data = Some(data);
        self
    }
    pub fn json(self) ->Json<Self> {
        Json(self)
    }
}
