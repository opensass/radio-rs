use crate::common::{Orientation, Size, Type, HIDDEN_INPUT_STYLE};
use dioxus::prelude::*;
use dioxus_core::AttributeValue;

/// Properties for configuring the `Group` component.
///
/// The `Group` component allows you to create a group of radio buttons with customizable
/// styles, orientation, and behavior. This is useful for creating user-selectable options
/// presented as a single group.
///
/// It supports horizontal or vertical layouts, CSS customizations, and child components of type `Radio` only (TODO).
#[derive(Props, PartialEq, Clone)]
pub struct GroupProps {
    /// The selected value of the radio group.
    ///
    /// This represents the current value selected in the group. It can be bound to a state
    /// to reflect changes dynamically. Defaults to an empty string if not provided.
    #[props(default)]
    pub selected: String,

    /// Callback for when the selected value changes.
    ///
    /// This callback is triggered whenever the user selects a different radio button. It
    /// provides the new selected value as a string. Defaults to a no-op.
    #[props(default)]
    pub onchange: Callback<String>,

    /// Orientation of the radio buttons group (horizontal or vertical).
    ///
    /// Determines the layout of the radio buttons. The available options are:
    /// - `Orientation::Horizontal`: Displays the radio buttons side by side.
    /// - `Orientation::Vertical`: Stacks the radio buttons vertically.
    ///   Defaults to `Orientation::Horizontal`.
    #[props(default)]
    pub orientation: Orientation,

    /// Additional inline styles for the container.
    ///
    /// Allows for custom inline styles to be applied directly to the group container.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub style: &'static str,

    /// Additional CSS classes for the container.
    ///
    /// Enables adding custom CSS classes to the group container for further styling or
    /// overriding default styles. Defaults to an empty string if not provided.
    #[props(default = "")]
    pub class: &'static str,

    /// Child components for the group.
    ///
    /// This property allows you to pass one or more `Radio` components as children of the
    /// `Group` component. The children will be arranged based on the specified `orientation`.
    /// Defaults to an empty list of children if not provided.
    ///
    /// TODO: Restrict children to `Radio` type and not any Element.
    pub children: Element,
}

// TODO: Fix this 9000 IQ HACK
fn process_attrs(
    mut attrs: Box<[Box<[Attribute]>]>,
    selected: &str,
    onchange: Callback<String>,
) -> Box<[Box<[Attribute]>]> {
    for attribute in &mut attrs {
        let attr = &mut (*attribute)[0];

        if attr.name == "value" {
            let is_selected = match &attr.value {
                AttributeValue::Text(value) => value == selected,
                _ => false,
            };

            if is_selected {
                attr.value = AttributeValue::Bool(true);
            } else {
                attr.value = AttributeValue::Bool(false);
            }
        }

        if attr.name == "on_click" {
            let onclick = {
                let value = match &attr.value {
                    AttributeValue::Text(val) => val.clone(),
                    _ => String::new(),
                };
                Callback::new(move |_| {
                    onchange.call(value.clone());
                })
            };
            attr.value = AttributeValue::Listener(onclick);
        }
    }
    attrs
}

/// Group Component
///
/// A Dioxus component for creating a group of radio buttons with customizable styles,
/// orientation, and behavior. The `Group` component arranges child `Radio` components
/// horizontally or vertically and handles the selection state.
///
/// # Properties
/// The `Group` component uses the `GroupProps` struct for its properties. Key properties include:
///
/// - **selected**: The currently selected value of the group (`String`).
///   Represents the value of the selected radio button.
/// - **onchange**: Callback function that is triggered when the selected value changes (`Callback<String>`).
/// - **orientation**: Defines the layout direction of the group. Can be horizontal or vertical
///   (`Orientation`). Default: `Orientation::Horizontal`.
/// - **style**: Custom inline styles applied to the group container (`String`). Default: `""`.
/// - **class**: CSS class names for the group container (`String`). Default: `""`.
/// - **children**: A collection of `Radio` components to be rendered inside the `Group`. Each child should be a `Radio` component.
///
/// # Features
/// - Automatically manages the state of the selected radio button.
/// - Triggers the provided `onchange` callback with the updated value when the selection changes.
/// - Supports horizontal or vertical orientation through the `Orientation` enum.
/// - Highly customizable with inline styles and additional CSS classes.
/// - Ensures only valid `Radio` components are accepted as children.
///
/// # Examples
///
/// ## Basic Usage
/// This example demonstrates how to create a simple horizontal radio group:
///
/// ```rust
/// use dioxus::prelude::*;
/// use radiors::dioxus::{Group, Radio};
///
/// #[component]
/// fn App() -> Element {
///     let mut selected = use_signal(|| "Option1".to_string());
///     let onchange = {
///         move |new_value: String| selected.set(new_value)
///     };
///
///     rsx! {
///         Group {
///             selected: selected(),
///             onchange: onchange,
///             Radio { value: "Option1", label: "Option 1" }
///             Radio { value: "Option2", label: "Option 2" }
///             Radio { value: "Option3", label: "Option 3" }
///         }
///     }
/// }
/// ```
///
/// ## Horizontal Orientation
/// Use the `orientation` property to arrange the group horizontally:
///
/// ```rust
/// use dioxus::prelude::*;
/// use radiors::dioxus::{Group, Radio};
/// use radiors::Orientation;
///
/// #[component]
/// fn App() -> Element {
///     let mut selected = use_signal(|| "Option1".to_string());
///     let onchange = {
///         move |new_value: String| selected.set(new_value)
///     };
///
///     rsx! {
///         Group {
///             selected: selected(),
///             onchange: onchange,
///             orientation: Orientation::Horizontal,
///             style: "margin: 20px;",
///             Radio { value: "Option1", label: "Option 1" }
///             Radio { value: "Option2", label: "Option 2" }
///             Radio { value: "Option3", label: "Option 3" }
///         }
///     }
/// }
/// ```
///
/// ## Custom Styling
/// Customize the appearance of the group container using `style` and `class`:
///
/// ```rust
/// use dioxus::prelude::*;
/// use radiors::dioxus::{Group, Radio};
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Group {
///             selected: "Option1",
///             style: "border: 1px solid black; padding: 10px;",
///             class: "custom-radio-group",
///             Radio { value: "Option1", label: "Option 1" }
///             Radio { value: "Option2", label: "Option 2" }
///         }
///     }
/// }
/// ```
///
/// # Behavior
/// - Clicking a `Radio` component updates the `selected` value in the `Group`.
/// - The `onchange` callback is triggered with the `value` of the selected `Radio` as a `String`.
/// - When `orientation` is set to `Orientation::Vertical`, the child components are stacked vertically.
/// - Inline styles and CSS classes allow fine-grained control of the component's appearance.
///
/// # Notes
/// - TODO: The `children` property must contain only `Radio` components; other elements will cause runtime errors.
/// - The `selected` property must match one of the `value` attributes in the `Radio` components for proper behavior.
#[component]
pub fn Group(props: GroupProps) -> Element {
    rsx! {
        div {
            class: "{props.class}",
            style: "{props.orientation.to_style()} {props.style}",
            for child in props.children.iter() {
                {
                    VNode::new(
                        child.key.clone(),
                        child.template,
                        child.dynamic_nodes.clone(),
                        process_attrs(
                            child.dynamic_attrs.clone(),
                            &props.selected,
                            props.onchange
                        )
                    )
                }
            }
        }
    }
}

/// Properties for configuring the `Radio` component.
///
/// The `Radio` component allows the creation of individual, customizable radio buttons.
/// It supports various configurations for appearance, behavior, and styles.
#[derive(Props, PartialEq, Clone)]
pub struct RadioProps {
    /// The label for the radio button.
    ///
    /// Defines the text displayed next to the radio button.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub label: &'static str,

    /// The value for the radio button.
    ///
    /// This value represents the data associated with the radio button, used to identify
    /// it in the `Group`'s selection context.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub value: &'static str,

    /// Optional image URL for the radio button.
    ///
    /// If provided, the radio button can display an image along with its label.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub src: &'static str,

    /// Inline styles for the container.
    ///
    /// Allows applying custom inline CSS styles directly to the radio button's container.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub style: &'static str,

    /// CSS class for the container.
    ///
    /// Adds custom CSS classes to style the radio button's container.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub class: &'static str,

    /// Inline styles for the label.
    ///
    /// Enables the addition of custom inline CSS styles to the label text.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub label_style: &'static str,

    /// CSS class for the label.
    ///
    /// Adds custom CSS classes to style the radio button's label.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub label_class: &'static str,

    /// Inline styles for the image.
    ///
    /// Allows applying custom inline CSS styles to the image associated with the radio button.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub image_style: &'static str,

    /// CSS class for the image.
    ///
    /// Adds custom CSS classes to style the image associated with the radio button.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub image_class: &'static str,

    /// The size of the radio button.
    ///
    /// Specifies the size of the radio button. Possible values are:
    /// - `Size::XSmall`
    /// - `Size::Small`
    /// - `Size::Medium`
    /// - etc
    ///   Defaults to `Size::XSmall` if not provided.
    #[props(default)]
    pub size: Size,

    /// The styling type of the radio button.
    ///
    /// Determines the visual theme of the radio button, such as Primary or Secondary styles.
    /// Defaults to `Type::None` if not provided.
    #[props(default)]
    pub r#type: Type,

    /// Whether this radio button is selected.
    ///
    /// Indicates if the radio button is currently selected. Managed internally in the `Group`
    /// component context. Defaults to `false`.
    #[props(default = false)]
    pub selected: bool,

    /// Whether this radio button is disabled.
    ///
    /// If `true`, the radio button is not clickable and appears disabled.
    /// Defaults to `false`.
    #[props(default = false)]
    pub disabled: bool,

    /// Inline styles applied when the radio button is selected.
    ///
    /// Provides custom styles for the radio button in the selected state.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub selected_style: &'static str,

    /// CSS class applied when the radio button is selected.
    ///
    /// Adds custom CSS classes to style the radio button in the selected state.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub selected_class: &'static str,

    /// Inline styles applied when the radio button is disabled.
    ///
    /// Provides custom styles for the radio button in the disabled state.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub disabled_style: &'static str,

    /// CSS class applied when the radio button is disabled.
    ///
    /// Adds custom CSS classes to style the radio button in the disabled state.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub disabled_class: &'static str,

    /// Inline styles for animations applied to the radio button.
    ///
    /// Enables applying custom styles to animations or transitions for the radio button.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub animation_style: &'static str,

    /// CSS class for animations applied to the radio button.
    ///
    /// Adds custom CSS classes to handle animations or transitions for the radio button.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub animation_class: &'static str,

    /// Inline styles for the hidden input element.
    ///
    /// Provides custom styles for the hidden `<input>` element used for the radio button.
    /// Defaults to `HIDDEN_INPUT_STYLE`.
    #[props(default = HIDDEN_INPUT_STYLE)]
    pub input_style: &'static str,

    /// CSS class for the hidden input element.
    ///
    /// Adds custom CSS classes for the hidden `<input>` element.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub input_class: &'static str,

    /// Callback for when the radio button is clicked.
    ///
    /// Triggered whenever the user clicks on the radio button. It provides the `value` of the
    /// radio button as a string to the callback.
    /// Defaults to a no-op.
    #[props(default)]
    pub onclick: Callback<String>,

    /// Internal callback triggered when the radio button is clicked.
    ///
    /// This property is intended for use by the parent `Group` component to manage
    /// the state of the radio group. It is not exposed for direct use by end users.
    ///
    /// The callback receives the `value` of the clicked radio button as a `String`.
    #[props(default)]
    pub on_click: Callback<String>,
}

/// Radio Component
///
/// A Dioxus component for individual, customizable radio buttons. The `Radio` component is designed
/// to work seamlessly with the `Group` component, providing an interactive and customizable
/// selection experience.
///
/// # Properties
/// The `Radio` component uses the `RadioProps` struct for its properties. Key properties include:
///
/// - **label**: The text displayed next to the radio button (`String`). Default: `""`.
/// - **value**: The value associated with the radio button (`String`). Default: `""`.
/// - **selected**: Indicates if the radio button is selected (`bool`). Default: `false`.
/// - **disabled**: Whether the radio button is disabled (`bool`). Default: `false`.
/// - **on_click**: A callback triggered when the radio button is clicked (`Callback<String>`). Default: no-op.
/// - **onclick**: A secondary callback triggered when the radio button is clicked (`Callback<String>`). Default: no-op.
/// - **src**: URL of an optional image displayed alongside the radio button (`String`). Default: `""`.
/// - **style**: Custom inline styles for the container (`String`). Default: `""`.
/// - **class**: CSS class for the container (`String`). Default: `""`.
/// - **input_style**: Inline styles for the `<input>` element (`String`). Default: `""`.
/// - **input_class**: CSS class for the `<input>` element (`String`). Default: `""`.
/// - **selected_style**: Custom styles applied when the radio button is selected (`String`). Default: `""`.
/// - **selected_class**: CSS class applied when the radio button is selected (`String`). Default: `""`.
/// - **disabled_style**: Custom styles applied when the radio button is disabled (`String`). Default: `""`.
/// - **disabled_class**: CSS class applied when the radio button is disabled (`String`). Default: `""`.
/// - **animation_style**: Inline styles for animations (`String`). Default: `""`.
/// - **image_style**: Inline styles for the optional image (`String`). Default: `""`.
/// - **image_class**: CSS class for the optional image (`String`). Default: `""`.
/// - **label_style**: Inline styles for the label text (`String`). Default: `""`.
/// - **label_class**: CSS class for the label text (`String`). Default: `""`.
/// - **size**: Determines the size of the radio button. Uses the `Size` enum (`Size::Small`, `Size::Medium`, `Size::Large`). Default: `Size::XSmall`.
/// - **r#type**: Determines the type/style of the radio button. Uses the `Type` enum (e.g., `Type::Primary`, `Type::Secondary`). Default: `Type::None`.
///
/// # Features
/// - Provides extensive customization for styles and classes.
/// - Optional image display alongside the label.
/// - Supports selected and disabled states.
/// - Works with callbacks for handling user interactions.
/// - Designed to integrate into the `Group` component for managing grouped radio buttons.
///
/// # Examples
///
/// ## Basic Radio Button
/// ```rust
/// use dioxus::prelude::*;
/// use radiors::dioxus::Radio;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Radio {
///             label: "Option 1",
///             value: "Option1",
///         }
///     }
/// }
/// ```
///
/// ## Radio Button with an Image
/// ```rust
/// use dioxus::prelude::*;
/// use radiors::dioxus::Radio;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Radio {
///             label: "Option 1",
///             value: "Option1",
///             src: "assets/radio.png",
///             image_style: "width: 20px; height: 20px;",
///         }
///     }
/// }
/// ```
///
/// ## Custom Styling
/// ```rust
/// use dioxus::prelude::*;
/// use radiors::dioxus::Radio;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Radio {
///             label: "Custom Radio",
///             value: "Custom",
///             style: "padding: 10px; border: 2px solid blue;",
///             class: "custom-radio-class",
///         }
///     }
/// }
/// ```
///
/// ## Disabled Radio Button
/// ```rust
/// use dioxus::prelude::*;
/// use radiors::dioxus::Radio;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Radio {
///             label: "Disabled Radio",
///             value: "Disabled",
///             disabled: true,
///         }
///     }
/// }
/// ```
///
/// # Behavior
/// - The `onclick` callback is triggered when the radio button is clicked, passing its `value`.
/// - If the `disabled` property is set to `true`, the button will not respond to clicks or emit callbacks.
/// - When selected, the radio button applies the `selected_style` and `selected_class`.
/// - Similarly, when disabled, the button applies the `disabled_style` and `disabled_class`.
///
/// # Notes
/// - The `selected` and `on_click` properties are typically controlled by the parent `Group` component.
/// - If an image is provided via the `src` property, it will be rendered next to the label with optional custom styles and classes.
/// - The component uses the `Size` and `Type` enums for additional flexibility in appearance and behavior.
///
/// TODO: Add animations
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let onclick = {
        let value = props.value.to_string();
        move |e: MouseEvent| {
            e.stop_propagation();
            if !props.disabled {
                props.on_click.call(value.clone());
                props.onclick.call(value.clone());
            }
        }
    };

    let selected_style = if props.selected {
        props.selected_style
    } else {
        ""
    };
    let disabled_style = if props.disabled {
        props.disabled_style
    } else {
        ""
    };
    let selected_class = if props.selected {
        props.selected_class
    } else {
        ""
    };
    let disabled_class = if props.disabled {
        props.disabled_class
    } else {
        ""
    };

    rsx! {
        div {
            class: "{selected_class} {disabled_class} {props.class}",
            style: "{selected_style} {disabled_style} {props.style} {props.animation_style} {props.r#type.to_style()} {props.size.to_style()}",
            onclick: onclick,
            input {
                r#type: "radio",
                name: "radio",
                value: "{props.value}",
                checked: "{props.selected}",
                disabled: "{props.disabled}",
                style: "{props.input_style}",
                class: "{props.input_class}",
            },
            if !props.src.is_empty() {
                img {
                    src: "{props.src}",
                    alt: "{props.label}",
                    style: "{props.image_style}",
                    class: "{props.image_class}",
                }
            },
            span {
                style: "{props.label_style}",
                class: "{props.label_class}",
                "{props.label}"
            }
        }
    }
}
