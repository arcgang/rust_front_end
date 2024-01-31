
// brings in all the macros and functions that we need for the web-app
use yew::prelude::*;

// simple struct with one value. This is the model for the web  application
struct Model {
    value: i64 // 64 bit integer
}

// need to create a function component. Need to anotate the function with the function component macro
#[function_component(App)]
fn app() -> Html {

    
    let state = use_state(|| Model {
        value: 0
    });

    let onclick = {
        // this state variable will shadow the original state variable
        let state = state.clone(); // create a new state variable as the original state variable is owned by the yew framework and cannot be used by our program.

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    };

    html! {
        <div>
            <button {onclick}>{ "+1"}</button>
            <p> { state.value }</p>
        
        </div>
    }

}

fn main() {


    yew::start_app::<App>();
    // yew::Renderer::<App>::new().render();
}


