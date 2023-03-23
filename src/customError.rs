// use thiserror::Error;
//
// #[derive(Error, Debug)]
// enum LoginError{
//     #[error("database error")]
//     DatabaseError(#[from] SqlError),
//
//     #[error("password expired")]
//     PasswordExpired,
//     #[error("user not found error")]
//     UserNotFound,
//
//     #[error("network connection error")]
//     NetworkError(#[from] std::io::Error),
//     // from 사용해서 해당 에러를 사용자 지정 에러로 변환한다.
//     #[error("wrong error")]
//     WrongPassword,
// }
