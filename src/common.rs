pub mod error {
    use thiserror::Error;
    use actix_web::{ResponseError};
    
    #[derive(Error, Debug)]
    pub enum AlmaError {
        #[error("Failed to render HTML")]
        AskamaError(#[from] askama::Error),
    }
    
    // actix_web:ResposenErrorをAlmaErrorに実装する
    // 内部処理は使いまわすため、実装はしない
    impl ResponseError for AlmaError {}
}