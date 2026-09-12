// ECS = E, C, and S :)
pub type EntityId = u64;
pub type ComponentType = TypeId;
pub trait System {
    type Component: Any;
    fn run(&self, eid: EntityId, component: &mut Self::Component);
}

// -------- BEWARE ALL YE WHO ENTER BELOW ------------

// Please don't look, I'm naked...
// (evil hackery needed to read System::Component)
trait InternalSystem {
    fn run_internal(&self, eid: EntityId, component: &mut dyn Any);
}
impl<S: System> InternalSystem for S {
    fn run_internal(&self, eid: EntityId, component: &mut dyn Any) {
        if let Some(c) = component.downcast_mut::<S::Component>() {
            self.run(eid, c);
        } else {
            panic!("run_internal attempted to run a system on the wrong component!!!");
        }
    }
}
