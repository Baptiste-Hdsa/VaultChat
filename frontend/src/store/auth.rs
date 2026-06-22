use crate::data_models::user::User;
use leptos::prelude::*;

pub fn provide_auth_state() {
    let (current_user, set_current_user) = signal::<Option<User>>(None);

    provide_context(current_user);
    provide_context(set_current_user);
}

pub fn get_current_user() -> ReadSignal<Option<User>> {
    expect_context::<ReadSignal<Option<User>>>()
}
