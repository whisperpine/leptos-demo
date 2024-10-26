use std::any::type_name;

use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    });
}

#[component]
fn app() -> impl IntoView {
    view! {
        <DynamicStyleButton />
        <ThreeProgressBars />
        <br />
        <InnerHtml />
        <br />
        <MySizeOf<u32> />
    }
}

#[component]
fn dynamic_style_button() -> impl IntoView {
    let (x, set_x) = create_signal(0);
    view! {
        <button
            on:click=move |_| set_x.update(|n| *n += 10)
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", x() + 300)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", x)
        >
            "Click to Move"
        </button>
    }
}

#[component]
fn three_progress_bars() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let doubled_count = move || count() * 2;
    view! {
        <button class=" bg-slate-500" on:click=move |_| set_count.update(|n| *n += 5)>
            "add progress"
        </button>
        <div>
            <ProgressBar progress=count />
        </div>
        <div>
            <ProgressBar max=50 progress=count />
        </div>
        <div>
            <ProgressBar max=50 progress=doubled_count />
        </div>
    }
}

/// # My awesome progress bar
///
/// balabala...
#[component]
fn progress_bar(
    /// This is a comment for `max`
    #[prop(default = 100)]
    max: u32,
    /// This is a comment for `progress`
    progress: impl Fn() -> u32 + 'static,
) -> impl IntoView {
    view! { <progress max=max value=progress /> }
}

/// The [`view`] macro provides support for an additional attribute, `inner_html`,
/// which can be used to directly set the HTML contents of any element,
/// wiping out any other children you've given it.
#[component]
fn inner_html() -> impl IntoView {
    let html = "<p>This HTML will be injected.</p>";
    view! { <div inner_html=html /> }
}

/// # Generic component
///
/// Because props are built into a struct,
/// so all generic types must be used somewhere in the struct,
/// which is often easily accomplished using an optional PhantomData prop.
#[component]
fn my_size_of<T: Sized>(#[prop(optional)] _p: std::marker::PhantomData<T>) -> impl IntoView {
    let size = std::mem::size_of::<T>();
    let type_name = type_name::<T>();
    view! { <div>"size of " {type_name} " is " {size}</div> }
}
