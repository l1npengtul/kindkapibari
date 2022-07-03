// use crate::{error::KKBCoreError, user_data::UserData};
// use tera::{Context, Tera};
//
// #[derive(Clone, Debug)]
// pub struct Templater {
//     tera: Tera,
//     context: Context,
// }
//
// impl Templater {
//     pub fn new(user_data: &UserData) -> Result<Templater, KKBCoreError> {
//         let tera = Tera::default();
//         let context = Context::from_value(serde_json::to_value(user_data).unwrap_or_default())
//             .map_err(|why| KKBCoreError::TemplateInit(why.to_string()))?;
//         Ok(Templater { tera, context })
//     }
// }
