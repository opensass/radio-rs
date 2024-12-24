use radiors::yew::{Group, Radio};
use radiors::{Type, Orientation, Size};
use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let os_selected = use_state(|| "mac".to_string());
    let browser_selected = use_state(|| "chrome".to_string());
    let size_selected = use_state(|| "medium".to_string());
    let color_selected = use_state(|| "primary".to_string());
    let advanced_color_selected = use_state(|| "primary".to_string());
    let onchange =
        |state: UseStateHandle<String>| Callback::from(move |value: String| state.set(value));

    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Radio RS Yew Examples" }</h1>
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
                        selected={(*os_selected).clone()}
                        onchange={onchange(os_selected.clone())}
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
                        selected={(*os_selected).clone()}
                        onchange={onchange(os_selected.clone())}
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
                        selected={(*browser_selected).clone()}
                        onchange={onchange(browser_selected.clone())}
                    >
                        <Radio value="chrome" label="Chrome" disabled=true input_style="" />
                        <Radio value="firefox" label="Firefox" disabled=true input_style="" />
                        <Radio value="edge" label="Edge" disabled=true input_style="" />
                    </Group>
                </div>
                // Horizontal Radio Buttons With Images
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">
                        { "Horizontal Radio Buttons With Images" }
                    </h2>
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
                        selected={(*os_selected).clone()}
                        onchange={onchange(os_selected.clone())}
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
                        selected={(*size_selected).clone()}
                        onchange={onchange(size_selected.clone())}
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
                        selected={(*size_selected).clone()}
                        onchange={onchange(size_selected.clone())}
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
                // Color Type Selector
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Color Type Selector" }</h2>
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
        value="primary"
        label="Primary"
        class="p-4 rounded-full cursor-pointer border border-blue-500"
        selected_class="ring-4 ring-blue-300"
        r#type={Type::Primary}
    />
    <Radio
        value="secondary"
        label="Secondary"
        class="p-4 rounded-full cursor-pointer border border-gray-500"
        selected_class="ring-4 ring-gray-300"
        r#type={Type::Secondary}
    />
    <Radio
        value="success"
        label="Success"
        class="p-4 rounded-full cursor-pointer border border-green-500"
        selected_class="ring-4 ring-green-300"
        r#type={Type::Success}
    />
    <Radio
        value="info"
        label="Info"
        class="p-4 rounded-full cursor-pointer border border-teal-500"
        selected_class="ring-4 ring-teal-300"
        r#type={Type::Info}
    />
    <Radio
        value="warning"
        label="Warning"
        class="p-4 rounded-full cursor-pointer border border-yellow-500"
        selected_class="ring-4 ring-yellow-300"
        r#type={Type::Warning}
    />
    <Radio
        value="danger"
        label="Danger"
        class="p-4 rounded-full cursor-pointer border border-red-500"
        selected_class="ring-4 ring-red-300"
        r#type={Type::Danger}
    />
</Group>"# }
                    </pre>
                    <Group
                        selected={(*advanced_color_selected).clone()}
                        onchange={onchange(advanced_color_selected.clone())}
                        orientation={Orientation::Horizontal}
                        class="flex items-center gap-4 m-4"
                    >
                        <Radio
                            value="primary"
                            label="Primary"
                            class="p-4 rounded-full cursor-pointer border border-blue-500"
                            selected_class="ring-4 ring-blue-300"
                            r#type={Type::Primary}
                        />
                        <Radio
                            value="secondary"
                            label="Secondary"
                            class="p-4 rounded-full cursor-pointer border border-gray-500"
                            selected_class="ring-4 ring-gray-300"
                            r#type={Type::Secondary}
                        />
                        <Radio
                            value="success"
                            label="Success"
                            class="p-4 rounded-full cursor-pointer border border-green-500"
                            selected_class="ring-4 ring-green-300"
                            r#type={Type::Success}
                        />
                    </Group>
                    <Group
                        selected={(*advanced_color_selected).clone()}
                        onchange={onchange(advanced_color_selected.clone())}
                        orientation={Orientation::Horizontal}
                        class="flex items-center gap-4 m-4"
                    >
                        <Radio
                            value="info"
                            label="Info"
                            class="p-4 rounded-full cursor-pointer border border-teal-500"
                            selected_class="ring-4 ring-teal-300"
                            r#type={Type::Info}
                        />
                        <Radio
                            value="warning"
                            label="Warning"
                            class="p-4 rounded-full cursor-pointer border border-yellow-500"
                            selected_class="ring-4 ring-yellow-300"
                            r#type={Type::Warning}
                        />
                        <Radio
                            value="danger"
                            label="Danger"
                            class="p-4 rounded-full cursor-pointer border border-red-500"
                            selected_class="ring-4 ring-red-300"
                            r#type={Type::Danger}
                        />
                    </Group>
                </div>
                // Animated Radio Buttons
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Animated Radio Buttons" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Group
    selected={selected_state}
    onchange={Callback::from(move |value: String| selected_state.set(value))}
    class="flex items-center gap-4"
>
    <Radio
        value="primary"
        label="Primary"
        selected_class="ring-4 ring-blue-300"
        class="p-4 rounded-lg cursor-pointer border border-blue-500 bg-blue-500 text-white"
        animation_class="transform hover:scale-150"
    />
    <Radio
        value="secondary"
        label="Secondary"
        selected_class="ring-4 ring-gray-300"
        class="p-4 rounded-lg cursor-pointer border border-gray-500 bg-gray-500 text-white"
        animation_class="transform hover:scale-150"
    />
</Group>"# }
                    </pre>
                    <Group
                        selected={(*color_selected).clone()}
                        onchange={onchange(color_selected.clone())}
                        class="flex items-center gap-4"
                    >
                        <Radio
                            value="primary"
                            label="Primary"
                            selected_class="ring-4 ring-blue-300"
                            class="p-4 rounded-lg cursor-pointer border border-blue-500 bg-blue-500 text-white"
                            animation_class="transform hover:scale-150"
                        />
                        <Radio
                            value="secondary"
                            label="Secondary"
                            selected_class="ring-4 ring-gray-300"
                            class="p-4 rounded-lg cursor-pointer border border-gray-500 bg-gray-500 text-white"
                            animation_class="transform hover:scale-150"
                        />
                    </Group>
                </div>
                // Radio Buttons With Callbacks
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">
                        { "Radio Buttons With Callbacks (Press F12)" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Group
    selected={selected_state}
    onchange={Callback::from(move |value: String| selected_state.set(value))}
    class="flex items-center gap-4"
>
    <Radio
        value="primary"
        label="Primary"
        onclick={Callback::from(|value: String| log::info!("{} has been selected", value))}
        selected_class="ring-4 ring-blue-300"
        class="p-4 rounded-lg cursor-pointer border border-blue-500 bg-blue-500 text-white"
    />
    <Radio
        value="secondary"
        label="Secondary"
        onclick={Callback::from(|value: String| log::info!("{} has been selected", value))}
        selected_class="ring-4 ring-gray-300"
        class="p-4 rounded-lg cursor-pointer border border-gray-500 bg-gray-500 text-white"
    />
</Group>"# }
                    </pre>
                    <Group
                        selected={(*color_selected).clone()}
                        onchange={onchange(color_selected.clone())}
                        class="flex items-center gap-4"
                    >
                        <Radio
                            value="primary"
                            label="Primary"
                            selected_class="ring-4 ring-blue-300"
                            class="p-4 rounded-lg cursor-pointer border border-blue-500 bg-blue-500 text-white"
                            onclick={Callback::from(|value: String| log::info!("{} button has been selected", value))}
                        />
                        <Radio
                            value="secondary"
                            label="Secondary"
                            selected_class="ring-4 ring-gray-300"
                            class="p-4 rounded-lg cursor-pointer border border-gray-500 bg-gray-500 text-white"
                            onclick={Callback::from(|value: String| log::info!("{} button has been selected", value))}
                        />
                    </Group>
                </div>
            </div>
        </div>
    }
}
