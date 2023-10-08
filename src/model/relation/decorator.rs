use crate::arguments::Arguments;
use crate::model::relation::Relation;
use crate::result::Result;

#[derive(Debug)]
pub struct Decorator {
    pub path: Vec<String>,
    pub(crate) call: fn(Arguments, &mut Relation) -> Result<()>
}