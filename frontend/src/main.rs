use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> });
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1>test1</h1>
    }
}
