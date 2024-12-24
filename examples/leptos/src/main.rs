use leptos::prelude::*;
use radiors::leptos::{Group, Radio};
use radiors::{Orientation, Size, Type};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Home />
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let os_selected = signal("mac".to_string());
    let browser_selected = signal("chrome".to_string());
    let size_selected = signal("medium".to_string());
    let onchange = |state: (ReadSignal<String>, WriteSignal<String>)| {
        Callback::from(move |value: String| {
            state.1.set(value);
        })
    };

    view! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Radio RS Leptos Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">

                // Default Headless Radio Buttons
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Default Headless Radio Buttons" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Group
    selected={selected_state}
    onchange={Callback::from(move |value: String| selected_state.set(value))}
>
    <Radio value="mac" label="Mac" input_style="" />
    <Radio value="windows" label="Windows" input_style="" />
    <Radio value="linux" label="Linux" input_style="" />
</Group>"# }
                    </pre>
                    <Group
                        selected={os_selected.0.get()}
                        onchange={onchange(os_selected)}
                    >
                        <Radio value="mac" label="Mac" input_style="" />
                        <Radio value="windows" label="Windows" input_style="" />
                        <Radio value="linux" label="Linux" input_style="" />
                    </Group>
                </div>

                // Horizontal Headless Radio Buttons
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Horizontal Headless Radio Buttons" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Group
    selected={selected_state}
    onchange={Callback::from(move |value: String| selected_state.set(value))}
    orientation={Orientation::Horizontal}
>
    <Radio value="mac" label="Mac" input_style="" />
    <Radio value="windows" label="Windows" input_style="" />
    <Radio value="linux" label="Linux" input_style="" />
</Group>"# }
                    </pre>
                    <Group
                        selected={os_selected.0.get()}
                        onchange={onchange(os_selected)}
                        orientation={Orientation::Horizontal}
                    >
                        <Radio value="mac" label="Mac" input_style="" />
                        <Radio value="windows" label="Windows" input_style="" />
                        <Radio value="linux" label="Linux" input_style="" />
                    </Group>
                </div>

                // Disabled Radio Buttons
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Disabled Radio Buttons" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Group
    selected={selected_state}
    onchange={Callback::from(move |value: String| selected_state.set(value))}
>
    <Radio value="chrome" label="Chrome" disabled=true input_style="" />
    <Radio value="firefox" label="Firefox" disabled=true input_style="" />
    <Radio value="edge" label="Edge" disabled=true input_style="" />
</Group>"# }
                    </pre>
                    <Group
                        selected={browser_selected.0.get()}
                        onchange={onchange(browser_selected)}
                    >
                        <Radio value="chrome" label="Chrome" disabled=true input_style="" />
                        <Radio value="firefox" label="Firefox" disabled=true input_style="" />
                        <Radio value="edge" label="Edge" disabled=true input_style="" />
                    </Group>
                </div>

                // Horizontal Radio Buttons With Images
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Horizontal Radio Buttons With Images" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Group
    selected={selected_state}
    onchange={Callback::from(move |value: String| selected_state.set(value))}
    orientation={Orientation::Horizontal}
    class="flex items-center gap-4 justify-center"
>
    <Radio
        value="mac"
        label="Mac"
        src="assets/apple.png"
        class="p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md"
        selected_class="border-blue-500 bg-blue-100 text-blue-800"
        image_class="w-16"
    />
    <Radio
        value="windows"
        label="Windows"
        src="assets/windows.png"
        class="p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md"
        selected_class="border-blue-500 bg-blue-100 text-blue-800"
        image_class="w-16"
    />
    <Radio
        value="linux"
        label="Linux"
        src="assets/linux.png"
        class="p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md"
        selected_class="border-blue-500 bg-blue-100 text-blue-800"
        image_class="w-16"
    />
</Group>"# }
                    </pre>
                    <Group
                        selected={os_selected.0.get()}
                        onchange={onchange(os_selected)}
                        orientation={Orientation::Horizontal}
                        class="flex items-center gap-4 justify-center"
                    >
                        <Radio
                            value="mac"
                            label="Mac"
                            src="assets/apple.png"
                            class="p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md"
                            selected_class="border-blue-500 bg-blue-100 text-blue-800"
                            image_class="w-16"
                        />
                        <Radio
                            value="windows"
                            label="Windows"
                            src="assets/windows.png"
                            class="p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md"
                            selected_class="border-blue-500 bg-blue-100 text-blue-800"
                            image_class="w-16"
                        />
                        <Radio
                            value="linux"
                            label="Linux"
                            src="assets/linux.png"
                            class="p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md"
                            selected_class="border-blue-500 bg-blue-100 text-blue-800"
                            image_class="w-16"
                        />
                    </Group>
                </div>

                // Horizontal Radio Buttons With All Sizes
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Size Selector" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Group
    selected={selected_state}
    onchange={Callback::from(move |value: String| selected_state.set(value))}
    orientation={Orientation::Horizontal}
    class="flex items-center gap-4"
>
    <Radio
        value="xsmall"
        label="XSmall"
        size={Size::XSmall}
        class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
        selected_class="border-green-500 bg-green-100 text-green-800"
    />
    <Radio
        value="small"
        label="Small"
        size={Size::Small}
        class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
        selected_class="border-green-500 bg-green-100 text-green-800"
    />
    <Radio
        value="medium"
        label="Medium"
        size={Size::Medium}
        class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
        selected_class="border-green-500 bg-green-100 text-green-800"
    />
    <Radio
        value="large"
        label="Large"
        size={Size::Large}
        class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
        selected_class="border-green-500 bg-green-100 text-green-800"
    />
    <Radio
        value="xlarge"
        label="XLarge"
        size={Size::XLarge}
        class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
        selected_class="border-green-500 bg-green-100 text-green-800"
    />
    <Radio
        value="xxlarge"
        label="XXLarge"
        size={Size::XXLarge}
        class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
        selected_class="border-green-500 bg-green-100 text-green-800"
    />
</Group>"# }
                    </pre>
                    <Group
                        selected={size_selected.0.get()}
                        onchange={onchange(size_selected)}
                        orientation={Orientation::Horizontal}
                        class="flex items-center gap-4 m-4"
                    >
                        <Radio
                            value="xsmall"
                            label="XSmall"
                            size={Size::XSmall}
                            class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
                            selected_class="border-green-500 bg-green-100 text-green-800"
                        />
                        <Radio
                            value="small"
                            label="Small"
                            size={Size::Small}
                            class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
                            selected_class="border-green-500 bg-green-100 text-green-800"
                        />
                        <Radio
                            value="medium"
                            label="Medium"
                            size={Size::Medium}
                            class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
                            selected_class="border-green-500 bg-green-100 text-green-800"
                        />
                    </Group>
                    <Group
                        selected={size_selected.0.get()}
                        onchange={onchange(size_selected)}
                        orientation={Orientation::Horizontal}
                        class="flex items-center gap-4 m-4"
                    >
                        <Radio
                            value="large"
                            label="Large"
                            size={Size::Large}
                            class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
                            selected_class="border-green-500 bg-green-100 text-green-800"
                        />
                        <Radio
                            value="xlarge"
                            label="XLarge"
                            size={Size::XLarge}
                            class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
                            selected_class="border-green-500 bg-green-100 text-green-800"
                        />
                        <Radio
                            value="xxlarge"
                            label="XXLarge"
                            size={Size::XXLarge}
                            class="border border-gray-400 rounded-lg cursor-pointer hover:shadow-md"
                            selected_class="border-green-500 bg-green-100 text-green-800"
                        />
                    </Group>
                </div>

            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount::mount_to_body(|| view! { <App/> });
}
