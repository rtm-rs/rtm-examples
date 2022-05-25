// Due to Rust issue #54726#issuecomment-464404849 with inner attributes they
// have to be inside module braces - top of a module file does not work.
//
// ```rust
// mod some_mod {
//   #![hello]
//   fn foo() { }
//   fn bar() { }
// }
// ```
// The item passed to my proc macro is
// `mod some_mod { fn foo() { } fn bar() { } }`
// this is rewritten, removing the placeholder `some_mod` 
mod reservation {
    #![rtm(aggregate = Reservation)]

    #[derive(Create, Update, Delete)]
    #[rtm(aggregate = Reservation)]
    struct Reservation;

    type UpdatedError = Error;
    type DeletedError = Error;
    type CreatedError = Error;

    fn is_created(&self) {
        if !self.state.is_some() {
            return Err(CreatedError);
        }
    }

    // Visitor pattern:
    // Impl Command for CreateCmd { fn execute<V: CommandVisitor>(&self, cv: &mut V) {cv.create(&self);}}
    // impl CommandVisitor for CommandExecutor { fn create(&self, c: &CreateCmd) { ... } }
    //
    // self is the command type or aggregate type?
    fn create(&mut self) {
        self.is_created()?;
        // This is the aggregate
        let data = Reservation {
            state: Some(true),
            order_id,
            reservation_items: data,
        };
        apply(data) // macro checks `apply` is last
    }

    // Visitor pattern:
    // Impl Command for DeleteCmd { fn execute<V: CommandVisitor>(&self, cv: &mut V) {cv.delete(&self);}}
    // impl CommandVisitor for CommandExecutor { fn delete(&self, c: &DeleteCmd) { ... } }
    fn delete(&self) {
        match self.state {
            State::Completed => return Err(UpdatedError),
            State::Canceled => return Err(DeletedError),
        }
        let data = aggregates::Reservation {
            state: self.state,
            order_id: self.order_id,
            reservation_items: self.reservation_items,
        };
        apply(data) // proc-macro checks apply(data) is last
    }

    // Visitor pattern:
    // Impl Command for UpdateCmd { fn execute<V: CommandVisitor>(&self, cv: &mut V) { cv.update(&self);}}
    // impl CommandVisitor for CommandExecutor { fn update(&self, c: &UpdateCmd) { ... } }
    fn update(&self) {
        is_created()?;
        match self.state {
            State::Completed => return Err(UpdatedError),
            State::Canceled => return Err(DeletedError),
        }
        let data = aggregates::Reservation {
            state: self.state,
            order_id: self.order_id,
            reservation_items: self.reservation_items,
        };
        apply(data) // proc-macro checks apply(data) is last
    }

    trait CommandVisitor {
        fn create(&mut self, a: &impl Aggregate);
        fn update(&mut self, a: &impl Aggregate);
        fn delete(&mut self, a: &impl Aggregate);
    }

    trait Command {
        fn execute(&self, sv: &mut ShapeVisitor);
    }
    // `rtm` implements this for the visitor pattern
    #[derive(rtm::Event)]
    struct ReservationCreated;

    // This is in the commands namespace so won't clash with relation,
    // aggregate or event types named Reservation.
    struct Reservation;

    impl rtm::Command for Reservation {
        // Use the visitor pattern
        fn execute(data: impl Aggregate) {
            let cmd = create_reservation(data);
            apply(ReservationCreated.new(cmd))
        }
    }
    //Need to implement something like this for commands:
    fn computeArea(s: &Shape) -> f64 {
        struct Commands {
            area: f64,
        }

        impl CommandVisitor for CommandExecutor {
            fn visit_circle(&mut self, c: &Circle) {
                self.area = std::f64::consts::PI * c.radius * c.radius;
            }
            fn visit_rectangle(&mut self, r: &Rectangle) {
                self.area = r.length() * r.width();
            }
        }

        let mut ac = AreaCalculator { area: 0.0 };
        s.accept(&mut ac);
        ac.area
    }
}
