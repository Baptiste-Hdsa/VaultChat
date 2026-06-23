use crate::data_models::user::User;
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_use::storage::use_local_storage;

pub fn provide_auth_state() {
    let (current_user, set_current_user, _) =
        use_local_storage::<Option<User>, JsonSerdeCodec>("current_user");

    provide_context(current_user);
    provide_context(set_current_user);
}

pub fn get_current_user() -> Signal<Option<User>> {
    expect_context::<Signal<Option<User>>>()
}

pub fn set_current_user(user: Option<User>) {
    expect_context::<WriteSignal<Option<User>>>().set(user);
}
