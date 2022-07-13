use std::any::{Any, TypeId};
use std::sync::Arc;
use std::time::Instant;

struct Class {
    name: String,
    type_id: TypeId
}

impl Class {
    fn new<T: 'static>() -> Self {
        Self {
            name: std::any::type_name::<T>().to_string(),
            type_id: TypeId::of::<T>(),
        }
    }
}

struct Instance {
    inner: Arc<dyn Any>,
}

impl Instance {
    fn new(obj: impl Any) -> Self {
        Self {
            inner: Arc::new(obj)
        }
    }

    fn instance_of(&self, class: &Class) -> bool {
        self.inner.as_ref().type_id() == class.type_id
    }
}

struct Foo {}
struct Bar {}

#[test]
fn test_any() {
    let foo_class: Class = Class::new::<Foo>();
    let bar_class: Class = Class::new::<Bar>();
    let foo_instance: Instance = Instance::new(Foo {});

    assert!(foo_instance.instance_of(&foo_class));
    assert!(!foo_instance.instance_of(&bar_class));
}