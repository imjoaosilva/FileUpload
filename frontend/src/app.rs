use leptos::*;
use web_sys::{Event, FileList, SubmitEvent};
use web_sys::wasm_bindgen::JsCast;

use crate::services::api::upload_file;

#[component]
pub fn app() -> impl IntoView {
    let (loading, set_loading) = create_signal(false);
    let (files, set_files) = create_signal(None);
    let (url, set_url) = create_signal(String::new());
    let upload = create_action(move |f: &FileList| {
        let f = f.clone();
        set_loading.set(true);
        async move {
            let data = upload_file(f).await;
            set_loading.set(false);

            if data.status() == 200 {
                let body = data.text().await.unwrap();
                set_url.set(format!("http://localhost:3000/uploads/{}", body));
            }
        }
    });

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        let file: FileList = files.get().unwrap();
        upload.clone().dispatch(file)
    };

    let on_input = move |e: Event| {
        let files = e
            .target()
            .unwrap()
            .unchecked_ref::<web_sys::HtmlInputElement>()
            .files()
            .unwrap();
        set_files(Some(files.into()))
    };

    view! {
        <>
            { move || {
                if loading.get() {
                    view! {
                        <>
                            <p>"Loading..."</p>
                        </>
                    }
                } else {
                    view! {
                        <>
                            <form on:submit=on_submit>
                                <label for="picture">Select a picture:</label>
                                <input type="file" id="picture" name="picture" accept="image/png, image/jpeg" on:input=on_input/>
                                <button type="submit">Upload</button>
                            </form>

                            { if url.get() != "" {
                                view! {
                                    <>
                                        <img src=url/>
                                    </>
                                }
                            } else {
                                view! {
                                    <>
                                        <p>"No image uploaded"</p>
                                    </>
                                }
                            }}
                        </>
                    }
                }
            }}
        </>
    }
}
