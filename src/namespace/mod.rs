use std::collections::BTreeMap;
use std::sync::Arc;
use maplit::btreemap;
use crate::{middleware, model, model::Model, r#enum};
use crate::arguments::Arguments;
use crate::error::Error;
use crate::handler;
use crate::model::field::Field;
use crate::model::property::Property;
use crate::model::relation::Relation;
use crate::r#enum::Enum;
use crate::r#enum::member::Member;
use crate::r#struct::Struct;
use crate::utils::next_path;
use crate::result::Result;
use crate::pipeline;
use crate::stdlib::load::load;

#[derive(Debug)]
pub struct Namespace {
    pub path: Vec<String>,
    pub namespaces: BTreeMap<String, Namespace>,
    pub structs: BTreeMap<String, Struct>,
    pub models: BTreeMap<String, Model>,
    pub enums: BTreeMap<String, Enum>,
    pub model_decorators: BTreeMap<String, model::Decorator>,
    pub model_field_decorators: BTreeMap<String, model::field::Decorator>,
    pub model_relation_decorators: BTreeMap<String, model::relation::Decorator>,
    pub model_property_decorators: BTreeMap<String, model::property::Decorator>,
    pub enum_decorators: BTreeMap<String, r#enum::Decorator>,
    pub enum_member_decorators: BTreeMap<String, r#enum::member::Decorator>,
    pub pipeline_items: BTreeMap<String, pipeline::Item>,
    pub middlewares: BTreeMap<String, middleware::Definition>,
    pub handler_groups: BTreeMap<String, handler::Group>,
}

impl Namespace {

    /// Create a main namespace
    pub fn main() -> Self {
        Self::new(vec![])
    }

    fn new(path: Vec<String>) -> Self {
        Self {
            path,
            namespaces: btreemap!{},
            structs: btreemap!{},
            models: btreemap!{},
            enums: btreemap!{},
            model_decorators: btreemap!{},
            model_field_decorators: btreemap!{},
            model_relation_decorators: btreemap!{},
            model_property_decorators: btreemap!{},
            enum_decorators: btreemap!{},
            enum_member_decorators: btreemap!{},
            pipeline_items: btreemap!{},
            middlewares: btreemap! {},
            handler_groups: btreemap! {},
        }
    }

    pub fn load_standard_library(&mut self) -> Result<()> {
        if self.path.is_empty() {
            Err(Error::new("Standard library can only be loaded on main namespace"))?
        }
        load(self);
        Ok(())
    }

    pub fn path(&self) -> Vec<&str> {
        self.path.iter().map(|s| s.as_str()).collect()
    }

    pub fn namespace_mut(&mut self, name: &str) -> Option<&mut Namespace> {
        self.namespaces.get_mut(name)
    }

    pub fn namespace_mut_or_create(&mut self, name: &str) -> &mut Namespace {
        if !self.namespaces.contains_key(name) {
            self.namespaces.insert(name.to_owned(), Namespace::new(next_path(&self.path, name)));
        }
        self.namespaces.get_mut(name).unwrap()
    }

    pub fn namespace(&self, name: &str) -> Option<&Namespace> {
        self.namespaces.get(name)
    }

    pub fn define_model_decorator(&mut self, name: &str, call: fn(Arguments, &mut Model) -> Result<()>) {
        self.model_decorators.insert(name.to_owned(), model::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_model_field_decorator(&mut self, name: &str, call: fn(&Arguments, &mut Field) -> Result<()>) {
        self.model_field_decorators.insert(name.to_owned(), model::field::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_model_relation_decorator(&mut self, name: &str, call: fn(&Arguments, &mut Relation) -> Result<()>) {
        self.model_relation_decorators.insert(name.to_owned(), model::relation::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_model_property_decorator(&mut self, name: &str, call: fn(&Arguments, &mut Property) -> Result<()>) {
        self.model_property_decorators.insert(name.to_owned(), model::property::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_enum_decorator(&mut self, name: &str, call: fn(&Arguments, &mut Enum) -> Result<()>) {
        self.enum_decorators.insert(name.to_owned(), r#enum::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_enum_member_decorator(&mut self, name: &str, call: fn(&Arguments, &mut Member) -> Result<()>) {
        self.enum_member_decorators.insert(name.to_owned(), r#enum::member::Decorator { path: next_path(&self.path, name), call });
    }

    pub fn define_pipeline_item<T>(&mut self, name: &str, call: T) where T: pipeline::item::Call + 'static {
        self.pipeline_items.insert(name.to_owned(), pipeline::Item {
            path: next_path(&self.path, name),
            call: Arc::new(call)
        });
    }

    pub fn define_middleware<T>(&mut self, name: &str, call: T) where T: middleware::creator::Creator + 'static {
        self.middlewares.insert(name.to_owned(), middleware::Definition {
            path: next_path(&self.path, name),
            creator: Arc::new(call)
        });
    }

    pub fn define_handler_group<T>(&mut self, name: &str, builder: T) where T: Fn(&mut handler::Group) {
        let mut handler_group = handler::Group {
            path: next_path(&self.path, name),
            handlers: vec![],
        };
        builder(&mut handler_group);
        self.handler_groups.insert(name.to_owned(), handler_group);
    }

    pub fn define_struct<T>(&mut self, name: &str, builder: T) where T: Fn(&'static Vec<String>, &mut Struct) {
        let path = Box::leak(Box::new(next_path(&self.path, name))) as &'static Vec<String>;
        let mut r#struct = Struct {
            path: path.clone(),
            functions: btreemap! {},
            static_functions: btreemap! {}
        };
        builder(path, &mut r#struct);
        self.structs.insert(name.to_owned(), r#struct);
    }
}
