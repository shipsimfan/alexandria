use crate::EventKind;

impl<UserEvent: 'static + Send> From<UserEvent> for EventKind<UserEvent> {
    fn from(event: UserEvent) -> Self {
        EventKind::User(event)
    }
}
