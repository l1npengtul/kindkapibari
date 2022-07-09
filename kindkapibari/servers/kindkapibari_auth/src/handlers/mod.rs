use kindkapibari_core::route;

pub mod login;
pub mod signup;

route!{ 
    "/login" => login,
    "/signup" => signup
}