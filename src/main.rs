use yew::prelude::*;
use web_sys::{console, HtmlElement, MouseEvent, Event, EventTarget};
use yew::{
    html, Callback, TargetCast,
};
use yew_router::prelude::*;
use gloo::events::EventListener;
use gloo::utils::window;
use std::borrow::Cow;
use std::rc::Rc;

#[function_component]
fn ButtonApp() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

#[function_component]
fn HeaderCounterCombo() -> Html {
    let header_text = "Hello world".to_string();
    let header_html: Html = html! {
        <h1>{header_text}</h1>
    };
    
    let count: usize = 51;
    let counter_html: Html = html! {
        <p>{"My age is: "}{count}</p>
    };    
    html! {
        <div>{header_html}{counter_html}</div>
    }
}

#[function_component]
fn App2() -> Html {

    let onmousemove = Callback::from(|e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let rect = target.get_bounding_client_rect();
            let x = (e.client_x() as f64) - rect.left();
            let y = (e.client_y() as f64) - rect.top();
            console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
        }
    });

    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <>
        <div id="mousemoveme" {onmousemove}><button {onclick}>{ "+1" }</button></div>
        <p>{ *counter }</p>
        </>
    }
}

#[function_component]
fn HelloWorld() -> Html {
    html!{"Hello world!"}
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[function_component]
fn HelloWorldCallback(props: &Props) -> Html {

    props.on_name_entry.emit(String::from("Bob"));

    html! { "Hello" }
}

#[function_component]
fn HelloBob() -> Html {
    let on_name_entry: Callback<String> = Callback::from(move |name: String| {
        let greeting = format!("Hey, {}!", name);
        web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });

    html! { <HelloWorldCallback {on_name_entry} /> }
}

#[function_component]
fn HelloButton() -> Html {
    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });

    html! {
        <button {onclick}>{ "Click" }</button>
    }
}

#[function_component]
fn HelloChildren() -> Html {
    html! {
        <HelloChildrenComp>
            <span>{"Hey what is up ;)"}</span>
            <h1>{"THE SKY"}</h1>
        </HelloChildrenComp>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props2 {
    pub children: Html, // the field name `children` is important!
}

#[function_component]
fn HelloChildrenComp(props: &Props2) -> Html {
    html! {
        <div class="very-stylized-container">
            { props.children.clone() } // you can forward children like this
        </div>
    }
}



#[function_component(ShowStorageChanged)]
pub fn show_storage_changed() -> Html {
    let state_storage_changed = use_state(|| false);

    {
        let state_storage_changed = state_storage_changed.clone();
        use_effect(|| {
            let listener = EventListener::new(&window(), "storage", move |_| state_storage_changed.set(true));

            move || { drop(listener); }
        });
    }

    html! { <div>{"Storage Event Fired: "}{*state_storage_changed}</div> }
}

#[hook]
pub fn use_event<E, F>(target: &EventTarget, event_type: E, callback: F)
where
    E: Into<Cow<'static, str>>,
    F: Fn(Event) + 'static,
{
    #[derive(PartialEq, Clone)]
    struct EventDependents {
        target: EventTarget,
        event_type: Cow<'static, str>,
        callback: Callback<Event>,
    }

    let deps = EventDependents {
        target: target.clone(),
        event_type: event_type.into(),
        callback: Callback::from(callback),
    };

    use_effect_with(
        deps,
        |deps| {
            let EventDependents {
                target,
                event_type,
                callback,
            } = deps.clone();

            let listener = EventListener::new(&target, event_type, move |e| {
                callback.emit(e.clone());
            });

            move || {
                drop(listener);
            }
        },
    );
}

#[function_component]
fn HTMLChild() -> Html {
    html! {
        <div>
            <div data-key="abc"></div>
            <div class="parent">
                <span class="child" value="anything"></span>
                <label for="first-name">{ "First Name" }</label>
                <input type="text" id="first-name" value="placeholder" />
                <input type="checkbox" checked=true />
                <textarea value="write a story" />
                <select name="status">
                    <option selected=true disabled=false value="">{ "Selected" }</option>
                    <option selected=false disabled=true value="">{ "Unselected" }</option>
                </select>
            </div>
        </div>
    }
}

#[function_component]
fn SVGChild() -> Html {
    html! {
        <svg width="149" height="147" viewBox="0 0 149 147" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M60.5776 13.8268L51.8673 42.6431L77.7475 37.331L60.5776 13.8268Z" fill="#DEB819"/>
            <path d="M108.361 94.9937L138.708 90.686L115.342 69.8642" stroke="black" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <g filter="url(#filter0_d)">
                <circle cx="75.3326" cy="73.4918" r="55" fill="#FDD630"/>
                <circle cx="75.3326" cy="73.4918" r="52.5" stroke="black" stroke-width="5"/>
            </g>
            <circle cx="71" cy="99" r="5" fill="white" fill-opacity="0.75" stroke="black" stroke-width="3"/>
            <defs>
                <filter id="filter0_d" x="16.3326" y="18.4918" width="118" height="118" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                    <@{"feGaussianBlur"} stdDeviation="2"/>
                    <@{"feColorMatrix"} in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"/>
                </filter>
            </defs>
        </svg>
    }
}


#[function_component]
fn MyComponent() -> Html {
    html! {
        { "This component has no properties!" }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct BasicProps {
    user_first_name: String,
    user_last_name: String,
}

#[function_component]
fn MyComponentWithProps(basicprops: &BasicProps) -> Html {
    let BasicProps { user_first_name, user_last_name } = basicprops;
    html! {
        <>{"user_first_name: "}{user_first_name}{" and user_last_name: "}{user_last_name}</>
    }
}

#[function_component]
fn BasicComponent() -> Html {
    let basicprops = BasicProps {
        user_first_name: "Bob".to_owned(),
        user_last_name: "Smith".to_owned(),
    };

    html!{
        <>
            // No properties
            <MyComponent />

            // With Properties
            <MyComponentWithProps user_first_name="Sam" user_last_name="Idle" />

            // With the whole set of props provided at once
            <MyComponentWithProps ..basicprops.clone() />

            // With Properties from a variable and specific values overridden
            <MyComponentWithProps user_last_name="Elm" ..basicprops />
        </>
    }
}

#[derive(PartialEq, Properties)]
struct ChildrenProps {
    id: String,
    children: Html,
}

#[function_component]
fn Container(props: &ChildrenProps) -> Html {
    html! {
        <div id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}

#[function_component]
fn ChildrenPropApp() -> Html {
    let props1 = yew::props!(ChildrenProps {
        id: "container-1",
        children: Html::default(),
    });
    let props2 = yew::props!(ChildrenProps {
        id: "container-2",
        children: Html::default(),
    });

    html! {
    <div>
        <Container ..props1 >
            // props.children will be overwritten with this
        </Container>
        <Container ..props2 >
            // props.children will be overwritten with this
            <span>{ "I am a child, as you can see" }</span>
        </Container>
    </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
fn RouterApp() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

#[function_component]
fn App3() -> Html {
    html!{ 
        <div>
            <BasicComponent />
            <HelloWorld /><HeaderCounterCombo /><ButtonApp /><HelloBob /> <HelloButton /> <HelloChildren />
            <HTMLChild /> <SVGChild />
            <ChildrenPropApp /><HTMLChild /> 
            <input id="my_input" />
            <button onclick={Callback::from(|_| ())}>
                //      ^^^^^^^ event listener name
                    { "Click me!" }
            </button>
            <RouterApp />
        </div>
    }
}

fn main() {

    // yew::Renderer::<App>::new().render();


    yew::Renderer::<App3>::new().render();
}