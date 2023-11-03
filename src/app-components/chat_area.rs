use leptos::{*, html::Input};

const TYPE_AREA_CLASS: &str = "h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t";
const TYPE_AREA_CLASS_LIGHT: &str = "bg-white border-gray-300";
const TYPE_AREA_CLASS_DARK: &str = "bg-zinc-900 border-zinc-700";

const TEXT_AREA_CLASS: &str = "w-2/3 p-4 border rounded-full input-field";
const TEXT_AREA_CLASS_LIGHT: &str = "border-gray-300 text-black";
const TEXT_AREA_CLASS_DARK: &str = "bg-zinc-700 border-zinc-700 text-white";

const BUTTON_CLASS: &str = "h-full p-4 rounded-full cursor-pointer" ;
const BUTTON_CLASS_LIGHT: &str = "bg-blue-500 text-white";
const BUTTON_CLASS_DARK: &str = "bg-green-700 text-white";

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
