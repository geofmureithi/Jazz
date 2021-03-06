use jazz_vm::{
    machine::Machine, object::{Object, ObjectAddon}, object_pool::ObjectPool, value::Value
};

use std::{any::Any, cell::UnsafeCell, collections::HashMap};
/// Class
///
/// Every value in Jazz is an object, but not every object is a instance of Class.
///
/// Before initializing Class is just a `Function` object, after calling initializer `init` function returns `this` value
///
/// For stroing fields used UnsafeCell because RefCell returns BorrowMutError in initializer
///
///
#[derive(Debug)]
pub struct Class
{
    pub name: String,
    pub fields: UnsafeCell<HashMap<String, Value>>,
}

impl Clone for Class
{
    fn clone(&self) -> Class
    {
        let fields = unsafe { (*self.fields.get()).clone() };
        Class {
            name: self.name.clone(),
            fields: UnsafeCell::new(fields),
        }
    }
}

impl Class
{
    pub fn new() -> Class
    {
        Class {
            name: String::from("<uninitialized>"),
            fields: UnsafeCell::new(HashMap::new()),
        }
    }
}

impl ObjectAddon for Class
{
    fn o_clone(&self, m: &mut Machine) -> Value
    {
        let c = self.clone();

        let v = Value::Object(m.pool.allocate(Box::new(c)));
        v
    }
    fn to_String(&self, _m: &mut Machine) -> String
    {
        let fields = unsafe { &*self.fields.get() };
        format!("Class {} ({:?})", self.name, fields)
    }
}
impl Object for Class
{
    fn as_any(&self) -> &dyn Any
    {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any
    {
        self as &mut dyn Any
    }
    fn initialize(&mut self, _p: &mut ObjectPool)
    {
    }

    fn get_children(&self) -> Vec<usize>
    {
        Vec::new()
    }

    fn call(&self, m: &mut Machine, args: Vec<Value>) -> Value
    {
        let class = if let Value::Object(id) = args[0] {
            let obj = m.pool.get(id);

            obj.as_any().downcast_ref::<Class>().unwrap()
        } else {
            panic!("Expected Value::Object");
        };

        let ret = {
            let fields = unsafe { &mut *class.fields.get() };
            let field = fields.get("init").expect("Couldn't find initializer");
            let mut args = args.clone();
            args[0] = class.o_clone(m);
            let v = m.invoke(*field, args);
            m.stack.pop();
            v
        };
        ret
    }

    fn store_at(&self, m: &mut Machine, args: Vec<Value>, _: usize)
    {
        let fname = args[1].to_String(m);
        let fields = unsafe { &mut *self.fields.get() };
        fields.insert(fname, args[2]);
    }

    fn load_at(&self, m: &mut Machine, args: Vec<Value>, rindex: usize)
    {
        let _this = args[0];
        if let Value::Object(id) = args[1] {
            let str = m.pool.get(id).to_String(m);
            let fields = unsafe { &*self.fields.get() };
            let field = fields.get(&str).expect(&format!("No such field {}", str));

            m.set(rindex, *field);
        }
        if let Value::Int(_) = args[1] {
            let fields = unsafe { &*self.fields.get() };
            let field = fields
                .get("__get__")
                .expect("Class doesn't have __get__ method");
            let v = m.invoke(*field, args);
            m.set(rindex, v);
        }
    }
}
