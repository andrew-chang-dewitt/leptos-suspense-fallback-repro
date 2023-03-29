use async_std::task::sleep;
use core::time::Duration;
use leptos::{
    component, create_resource, create_server_action, server,
    server_fn::{self, ServerFn, ServerFnError},
    view, Children, ChildrenFn, IntoView, Scope, SignalGet, Suspense, SuspenseProps,
};
use leptos_meta::*;
use leptos_router::{AProps, Route, RouteProps, Router, RouterProps, Routes, RoutesProps, A};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/suspense-fallback-repro.css"/>
        <Title text="Suspense Fallback Reproduction"/>
        // content
        <Router>
            <Routes>
                <Route path="" view=|cx| view!{cx, <Layout>""</Layout>}/>
                <Route path="/basic-nesting" view=|cx| view!{cx, <Layout><BasicNesting/></Layout>}/>
                <Route path="/action-nesting" view=|cx| view!{cx, <Layout><ActionSuspense><Data/></ActionSuspense></Layout>}/>
            </Routes>
        </Router>
    }
}

#[component]
fn Layout(cx: Scope, children: Children) -> impl IntoView {
    view! {
        cx,
        <nav><ul>
            <li><A href="/basic-nesting">"Basic Nesting"</A></li>
            <li><A href="/action-nesting">"Action Suspense Nesting"</A></li>
        </ul></nav>
        {children(cx)}
    }
}

#[component]
fn BasicNesting(cx: Scope) -> impl IntoView {
    let resource = create_resource(cx, || (), move |_| get_wrapper());
    let data = move || resource.read(cx);

    view! {
        cx,
        <h1>"Basic Nesting"</h1>
        <p>"\
            This example shows what happens when you render a Suspense inside another Suspense. When \
            navigating to this route via client side routing (e.g. start at "<code>"/"</code>", then \
            click on \"Basic Nesting\" above), everything behaves as expected. However, when \
            navigating to this page via server side routing (e.g. via a hard refresh on this url or \
            typing it in and going to it directly), the content of the Outer Suspense (the "
            <code>"BasicNesting"</code>" component) is replaced with the content of the Inner Suspense \
            (the "<code>"Data"</code>" component) when Data's resource resolves. \
        "</p>
        <Suspense fallback=move || view!{cx, <p>"Loading wrapper..."</p>}>
            <p>"Inside wrapper suspense: "{data()}</p>
            <Data />
        </Suspense>
    }
}

async fn get_wrapper() -> bool {
    true
}

#[component]
fn ActionSuspense(cx: Scope, children: ChildrenFn) -> impl IntoView {
    let action = create_server_action::<B>(cx);
    let resource = create_resource(cx, move || action.version().get(), move |_| b(cx));
    let result = move || {
        resource.read(cx).map(|r| {
            log::debug!("result in ActionSuspense: {r:#?}");

            match r {
                Ok(true) => view! {
                    cx,
                    <p>"Result is true, rendering children:"</p>
                    {children(cx)}
                }
                .into_view(cx),
                _ => view! {
                    cx,
                    <p>"Result is false/err, not rendering children"</p>
                }
                .into_view(cx),
            }
        })
    };

    view! {
        cx,
        <h1>"Action Suspense"</h1>
            <p>"
                This example shows something closer to what I was dealing with in my project when \
                I discovered this issue. When getting data from a ServerFn via an action wrapped in \
                a resource, there is a similarity to the \"Basic Nesting\" example in that \
                everything works as expected when navigating here via client side routing and that \
                things begin to behave in unexpected ways via server side routing. The difference \
                is that instead of replacing the Outer Suspense's child on the Inner Suspense's \
                resource resolution, we see the Inner Suspense's "<code>"fallback"</code>" component \
                isn't replaced by the child—instead it stays while the child is appended to \
                the DOM after it.
            "</p>
        <Suspense fallback=move || view!{cx, <p>"Loading"</p>}>
            {result()}
        </Suspense>
    }
}

#[server(B, "/api")]
async fn b(_: Scope) -> Result<bool, ServerFnError> {
    log::debug!("getting from server fn");
    sleep(Duration::from_secs(2)).await;

    Ok(true)
}

#[component]
fn Data(cx: Scope) -> impl IntoView {
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
        <h2>"Data"</h2>
        <Suspense fallback=move || view!{cx, <p>"Loading..."</p>}>
            // FIXME:: the fallback component is never unmounted & replaced with the below child?
            {data()}
        </Suspense>
    }
}

pub async fn get_data() -> String {
    log::debug!("Getting data...");
    sleep(Duration::from_secs(2)).await;

    String::from("This is Data!")
}
