use leptos::{*, html::Input};

const TYPE_AREA_CLASS: &str = "h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t";
const TYPE_AREA_CLASS_LIGHT: &str = "bg-white border-gray-300";
const TYPE_AREA_CLASS_DARK: &str = "bg-zinc-900 border-zinc-700";

#[component]
pub fn TypeArea(send: Action<String, Result<(), ServerFnError>>) -> impl IntoView {
    let dark_mode = use_context::<ReadSignal<bool>>().expect("should be able to get dark mode state");

    let type_area_class = Signal::derive(move || {
      if dark_mode.get() {
        format!("{TYPE_AREA_CLASS} {TYPE_AREA_CLASS_DARK}")
      } else {
        format!("{TYPE_AREA_CLASS} {TYPE_AREA_CLASS_LIGHT}")
      }
    });

    

    let input_ref = create_node_ref::<Input>();
    view!{
        <div class={type_area_class.get()}>
           <form class="w-full flex justify-center items-center gap-4" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("");
           }
           >
                
           </form>
        </div>
    }
}