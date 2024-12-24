use dioxus::prelude::*;
use dioxus_logger::tracing;
use radiors::dioxus::{Group, Radio};
use radiors::Orientation;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        LandingPage {}
    }
}

#[component]
pub fn LandingPage() -> Element {
    let os_selected = use_signal(|| "mac".to_string());
    let browser_selected = use_signal(|| "chrome".to_string());

    let onchange = |mut state: Signal<String>| move |value: String| state.set(value);

    rsx! {
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 { class: "text-3xl font-bold mb-8 text-white", "Radio RS Dioxus Examples" }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                // Default Headless Radio Buttons
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Default Headless Radio Buttons" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Group {{
    selected: selected.clone(),
    onchange: |value| selected.set(value),
    Radio {{ value: "mac", label: "Mac", input_style: "" }}
    Radio {{ value: "windows", label: "Windows", input_style: "" }}
    Radio {{ value: "linux", label: "Linux", input_style: "" }}
}}"##
                    }
                    Group {
                        selected: os_selected.clone(),
                        onchange: onchange(os_selected.clone()),
                        Radio {
                            value: "mac",
                            label: "Mac",
                            input_style: ""
                        }
                        Radio {
                            value: "windows",
                            label: "Windows",
                            input_style: ""
                        }
                        Radio {
                            value: "linux",
                            label: "Linux",
                            input_style: ""
                        }
                    }
                }

                // Horizontal Headless Radio Buttons
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Horizontal Headless Radio Buttons" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Group {{
    selected: os_selected.clone(),
    onchange: |value| os_selected.set(value),
    orientation: Orientation::Horizontal,
    Radio {{ value: "mac", label: "Mac", input_style: "" }}
    Radio {{ value: "windows", label: "Windows", input_style: "" }}
    Radio {{ value: "linux", label: "Linux", input_style: "" }}
}}"##
                    }
                    Group {
                        selected: os_selected.clone(),
                        onchange: onchange(os_selected.clone()),
                        orientation: Orientation::Horizontal,
                        Radio {
                            value: "mac",
                            label: "Mac",
                            input_style: ""
                        }
                        Radio {
                            value: "windows",
                            label: "Windows",
                            input_style: ""
                        }
                        Radio {
                            value: "linux",
                            label: "Linux",
                            input_style: ""
                        }
                    }
                }

                // Disabled Radio Buttons
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Disabled Radio Buttons" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Group {{
    selected: browser_selected.clone(),
    onchange: |value| browser_selected.set(value),
    Radio {{ value: "mac", label: "Mac", input_style: "", disabled: true }}
    Radio {{ value: "windows", label: "Windows", input_style: "", disabled: true }}
    Radio {{ value: "linux", label: "Linux", input_style: "", disabled: true }}
}}"##
                    }
                    Group {
                        selected: browser_selected.clone(),
                        onchange: onchange(browser_selected.clone()),
                        Radio {
                            value: "mac",
                            label: "Mac",
                            input_style: "",
                            disabled: true
                        }
                        Radio {
                            value: "windows",
                            label: "Windows",
                            input_style: "",
                            disabled: true
                        }
                        Radio {
                            value: "linux",
                            label: "Linux",
                            input_style: "",
                            disabled: true
                        }
                    }
                }

                // Horizontal Radio Buttons With Images
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Horizontal Radio Buttons With Images" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Group {{
    selected: os_selected.clone(),
    onchange: |value| os_selected.set(value),
    orientation: Orientation::Horizontal,
    Radio {{
        value: "mac",
        label: "Mac",
        src: "assets/apple.png",
        class: "p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md",
        selected_class: "border-blue-500 bg-blue-100 text-blue-800",
        image_class: "w-16",
    }}
    Radio {{
        value: "windows",
        label: "Windows",
        src: "assets/windows.png",
        class: "p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md",
        selected_class: "border-blue-500 bg-blue-100 text-blue-800",
        image_class: "w-16",
    }}
    Radio {{
        value: "linux",
        label: "Linux",
        src: "assets/linux.png",
        class: "p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md",
        selected_class: "border-blue-500 bg-blue-100 text-blue-800",
        image_class: "w-16",
    }}
}}"##
                    }
                    Group {
                        selected: os_selected.clone(),
                        onchange: onchange(os_selected.clone()),
                        orientation: Orientation::Horizontal,
                        Radio {
                            value: "mac",
                            label: "Mac",
                            src: "assets/apple.png",
                            class: "p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md",
                            selected_class: "border-blue-500 bg-blue-100 text-blue-800",
                            image_class: "w-16",
                            onclick: Callback::new(|value: String| tracing::info!("{} button has been selected", value))
                        }
                        Radio {
                            value: "windows",
                            label: "Windows",
                            src: "assets/windows.png",
                            class: "p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md",
                            selected_class: "border-blue-500 bg-blue-100 text-blue-800",
                            image_class: "w-16",
                            onclick: Callback::new(|value: String| tracing::info!("{} button has been selected", value))
                        }
                        Radio {
                            value: "linux",
                            label: "Linux",
                            src: "assets/linux.png",
                            class: "p-4 border-gray-400 border items-center rounded-lg cursor-pointer hover:shadow-md",
                            selected_class: "border-blue-500 bg-blue-100 text-blue-800",
                            image_class: "w-16",
                            onclick: Callback::new(|value: String| tracing::info!("{} button has been selected", value))
                        }
                    }
                }
            }
        }
    }
}
