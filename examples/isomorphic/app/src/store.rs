use crate::state::Msg;
use crate::state::State;

use std::ops::Deref;
use std::rc::Rc;
use router_rs::prelude::Router;

pub struct Store {
    state: StateWrapper,
    after_route: Option<Box<Fn(&str) -> ()>>,
    router: Option<Rc<Router>>
}

impl Store {
    pub fn new(state: State) -> Store {
        Store {
            state: StateWrapper(state),
            after_route: None,
            router: None
        }
    }

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::SetPath(path) => {
                if let Some(router) = &self.router {
                    if let Some(route_handler) = router.matching_routerhandler(path.as_str()) {
                        route_handler.on_visit(path.as_str());
                    }
                }

                self.state.msg(msg);

                if let Some(after_route) = &self.after_route {
                    after_route(path.as_str());
                }
            }
            _ => self.state.msg(msg),
        }
    }

    pub fn subscribe(&mut self, callback: Box<Fn() -> ()>) {
        self.state.subscribe(callback);
    }

    pub fn set_after_route(&mut self, after_route: Box<Fn(&str) -> ()>) {
        self.after_route = Some(after_route);
    }

    pub fn set_router(&mut self, router: Rc<Router>) {
        self.router = Some(router);
    }
}

impl Deref for Store {
    type Target = State;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.state
    }
}

struct StateWrapper(State);

impl Deref for StateWrapper {
    type Target = State;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl StateWrapper {
    fn msg(&mut self, msg: &Msg) {
        self.0.msg(msg)
    }

    fn subscribe(&mut self, callback: Box<Fn() -> ()>) {
        self.0.subscribe(callback);
    }
}
