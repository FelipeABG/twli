use crate::runtime::{Callable, Object};

pub struct Println {}
impl Callable for Println {
    fn call(
        &mut self,
        _: &mut crate::interpreter::Interpreter,
        args: Vec<crate::runtime::Object>,
    ) -> anyhow::Result<crate::runtime::Object> {
        println!("{}", args[0]);
        return Ok(Object::Null);
    }

    fn arity(&self) -> usize {
        1
    }

    fn to_string(&self) -> String {
        "<std fn println>".to_string()
    }

    fn clone_box(&self) -> Box<dyn Callable + Send + Sync + 'static> {
        Box::new(Println {})
    }
}
