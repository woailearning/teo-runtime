use crate::arguments::Arguments;
use crate::model::property::Property;
use crate::result::Result;

#[derive(Debug)]
pub struct Decorator {
    pub path: Vec<String>,
    pub(crate) call: fn(&Arguments, &mut Property) -> Result<()>
}