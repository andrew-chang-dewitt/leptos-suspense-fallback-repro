use leptos::{
    component, create_resource, view, IntoView, Scope, ServerFnError, Suspense, SuspenseProps,
};
use leptos_meta::*;
use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/suspense-fallback-repro.css"/>

        // sets the document title
        <Title text="Suspense Fallback Reproduction"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Data/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Data(cx: Scope) -> impl IntoView {
    let data_resource = create_resource(
        cx,
        // TODO: for now just run this resource on component load
        || (),
        move |_| get_data(),
    );
    let data = move || {
        let value = data_resource.read(cx);
        log::debug!("processing data... {value:#?}");
        value
    };

    view! {
        cx,
        <h1>"Data"</h1>
        <Suspense fallback={ move || view!{cx, <p>"Loading..."</p>}}>
            // FIXME:: the fallback component is never unmounted & replaced with the below child?
            {move || data().map(|r| match r {
                Ok(a) => view!{cx, <p>{serde_json::to_string(&a)}</p>},
                Err(e) => view!{cx, <p>{e.to_string()}</p>},
            })}
        </Suspense>
    }
}

pub async fn get_data() -> Result<Vec<String>, ServerFnError> {
    log::debug!("Getting data...");

    Ok(vec![String::from("This is Data!")])
}
