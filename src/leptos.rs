#![allow(unused)]

use crate::common::{Orientation, Size, Type, HIDDEN_INPUT_STYLE};
use leptos::{ev::MouseEvent, prelude::*};

/// Group Component
///
/// A Leptos component that represents a group of radio buttons, allowing you to select one option
/// from a set of predefined choices. This `Group` component supports customizing the appearance, orientation,
/// and behavior of the radio button group. It also provides a way to track and react to changes in selection.
///
/// # Properties
///
/// - **selected**: A string representing the currently selected value in the group. The default value is an empty string.
/// - **onchange**: A callback triggered whenever the selection changes. The callback receives the selected value (of type `String`).
/// - **orientation**: Defines the layout of the radio buttons within the group. It can be either `Horizontal` or `Vertical` (default: `Horizontal`).
/// - **style**: Inline styles applied to the group container (`&'static str`). Default: `""`.
/// - **class**: CSS class for the group container (`&'static str`). Default: `""`.
/// - **children**: Child `Radio` components nested inside the `Group`. These components will be rendered as part of the group.
///
/// # Features
/// - Supports both horizontal and vertical orientations for the radio buttons.
/// - Customizable inline styles and CSS classes.
/// - Callback mechanism for reacting to selection changes.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use leptos::prelude::*;
/// use radiors::leptos::{Group, Radio};
/// use leptos::logging::log;
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let selected = signal(String::new());
///     let onchange = Callback::from(move |value: String| {
///         log!("Selected value changed to: {}", value);
///     });
///
///     view! {
///         <Group selected={selected.0.get()} onchange={onchange}>
///             <Radio value="option1" label="Option 1" />
///             <Radio value="option2" label="Option 2" />
///             <Radio value="option3" label="Option 3" />
///         </Group>
///     }
/// }
/// ```
///
/// ## Group with Vertical Orientation
/// ```rust
/// use leptos::prelude::*;
/// use radiors::leptos::{Group, Radio};
/// use radiors::Orientation;
/// use leptos::logging::log;
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let selected = signal(String::new());
///     let onchange = Callback::from(move |value: String| {
///         log!("Selected value changed to: {}", value);
///     });
///
///     view! {
///         <Group selected={selected.0.get()} onchange={onchange} orientation={Orientation::Vertical}>
///             <Radio value="option1" label="Option 1" />
///             <Radio value="option2" label="Option 2" />
///             <Radio value="option3" label="Option 3" />
///         </Group>
///     }
/// }
/// ```
///
/// ## Group with Custom Styles and Class
/// ```rust
/// use leptos::prelude::*;
/// use radiors::leptos::{Group, Radio};
/// use leptos::logging::log;
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let selected = signal(String::new());
///     let onchange = Callback::from(move |value: String| {
///         log!("Selected value changed to: {}", value);
///     });
///
///     view! {
///         <Group
///             selected={selected.0.get()}
///             onchange={onchange}
///             style="border: 1px solid black; padding: 10px;"
///             class="radio-group"
///         >
///             <Radio value="option1" label="Option 1" />
///             <Radio value="option2" label="Option 2" />
///             <Radio value="option3" label="Option 3" />
///         </Group>
///     }
/// }
/// ```
///
/// # Behavior
/// - The `selected` property tracks the value of the selected radio button.
/// - The `onchange` callback is triggered whenever the selected value changes.
/// - The `orientation` property determines the layout of the radio buttons. By default, it is horizontal, but it can be set to vertical.
/// - Child `Radio` components are rendered as part of the `Group`. Each `Radio` component should have a unique `value` to distinguish between them.
/// - Custom inline styles and CSS classes can be used for detailed customization of the group's appearance.
///
/// # Notes
/// - The `selected` value should match one of the `Radio` component values in the group, or it will default to an empty string.
/// - The `onchange` callback provides a way to react to user selection. It passes the new selected value as a `String`.
/// - The `orientation` property allows for switching between a horizontal or vertical layout for the radio buttons.
/// - Custom inline styles and classes allow for further customization of the group’s appearance and behavior.
#[component]
pub fn Group(
    /// Selected value in the group.
    ///
    /// This represents the value that is currently selected within the group of
    /// radio buttons. The default value is an empty string.
    #[prop(default = String::new())]
    selected: String,

    /// Callback for when the selection changes.
    ///
    /// This callback is triggered whenever the selection changes. It passes the
    /// selected value (of type `String`) to the callback function. This allows
    /// the parent component to react to changes in selection.
    #[prop(default = Callback::from(|value: String| {}))]
    onchange: Callback<(String,), ()>,

    /// Orientation of the group (horizontal or vertical).
    ///
    /// Specifies the layout of the radio buttons within the group. The `Orientation`
    /// enum allows for either `Horizontal` or `Vertical` layouts. The default is `Horizontal`.
    #[prop(default = Orientation::Horizontal)]
    orientation: Orientation,

    /// Custom inline styles.
    ///
    /// This applies custom inline styles to the group container. It is a string
    /// value that will be included in the `style` attribute of the container.
    /// Defaults to an empty string.
    #[prop(default = "")]
    style: &'static str,

    /// Custom CSS class.
    ///
    /// This applies a custom CSS class to the group container. It is a string value
    /// that will be added to the `class` attribute of the container. Defaults to an
    /// empty string.
    #[prop(default = "")]
    class: &'static str,

    /// Child `Radio` components.
    ///
    /// These are the `Radio` components nested inside the `Group` component.
    /// They will be rendered as part of the group. This is typically used to pass
    /// a fragment of children elements to be displayed inside the group.
    children: ChildrenFragment,
) -> impl IntoView {
    let (selected, set_selected) = signal(selected);

    view! {
        <div
            class=class
            style=format!(
                "{} {}",
                orientation.to_style(),
                style
            )
        >
            {children().nodes.into_iter().map(|child| {
                // TODO:
                // Extract props from AnyView
                // let props = child;
                // let is_selected = props.value == selected.get();
                let on_click = {
                    let onchange = onchange.clone();
                    // let value = props.value.clone();
                    // move || {
                    //     set_selected.set(value.clone());
                    //     onchange.emit(value.clone());
                    // }
                };

                // TODO:
                // Update selected and on_click
                child
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// Radio Component
///
/// A Leptos component that represents a single radio button. This `Radio` component allows you to select
/// an option within a `Group` of radio buttons. It supports a variety of customization options, such as custom styles,
/// classes, images, and more. It also provides a callback mechanism for handling click events.
///
/// # Properties
///
/// - **label**: A string label associated with the radio button. It provides a descriptive text next to the radio button.
///   The default is an empty string.
/// - **value**: A string value representing the value of the radio button when selected. It is used to identify the selected option.
///   The default is an empty string.
/// - **src**: An optional image source to display alongside the radio button. If not specified, no image is shown.
///   The default is an empty string.
/// - **style**: Inline styles applied to the container element of the radio button. The default is an empty string.
/// - **class**: CSS class applied to the container element. The default is an empty string.
/// - **label_style**: Inline styles for the label element. The default is an empty string.
/// - **label_class**: CSS class applied to the label element. The default is an empty string.
/// - **image_style**: Inline styles applied to the image element. The default is an empty string.
/// - **image_class**: CSS class applied to the image element. The default is an empty string.
/// - **size**: Defines the size of the radio button, based on the `Size` enum. Possible values include `XSmall`, `Small`, `Medium`, and `Large`.
///   The default is `Size::XSmall`.
/// - **r#type**: Defines the type of the radio button. This is based on the `Type` enum and can define different behaviors or styles. The default is `Type::None`.
/// - **selected**: Whether the radio button is selected by default. The default value is `false`.
/// - **disabled**: Whether the radio button is disabled, preventing user interaction. The default value is `false`.
/// - **selected_style**: Inline styles applied when the radio button is selected. The default is an empty string.
/// - **selected_class**: CSS class applied when the radio button is selected. The default is an empty string.
/// - **disabled_style**: Inline styles applied when the radio button is disabled. The default is an empty string.
/// - **disabled_class**: CSS class applied when the radio button is disabled. The default is an empty string.
/// - **animation_style**: Inline styles applied for animations (e.g., hover effects). The default is an empty string.
/// - **animation_class**: CSS class applied for animations. The default is an empty string.
/// - **input_style**: Inline styles applied to the hidden input element associated with the radio button. The default is `HIDDEN_INPUT_STYLE`.
/// - **input_class**: CSS class applied to the hidden input element. The default is an empty string.
/// - **on_click**: A callback triggered when the radio button is clicked. It passes the `value` of the radio button as a `String` to the callback function.
///   The default is an empty callback.
///
/// # Features
/// - Supports custom labels, images, styles, and classes.
/// - Provides support for multiple sizes of radio buttons.
/// - Allows for radio buttons to be selected, disabled, or animated.
/// - Customizable styles when selected or disabled.
/// - Callback mechanism for click events.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use leptos::prelude::*;
/// use radiors::leptos::{Radio};
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     view! {
///         <Radio value="option1" label="Option 1" />
///     }
/// }
/// ```
///
/// ## Radio with Image and Custom Styles
/// ```rust
/// use leptos::prelude::*;
/// use radiors::leptos::{Radio};
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     view! {
///         <Radio
///             value="option1"
///             label="Option 1"
///             src="path/to/image.png"
///             style="border: 2px solid black; padding: 5px;"
///             label_style="color: blue;"
///         />
///     }
/// }
/// ```
///
/// ## Disabled Radio Button
/// ```rust
/// use leptos::prelude::*;
/// use radiors::leptos::{Radio};
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     view! {
///         <Radio
///             value="option1"
///             label="Option 1"
///             disabled=true
///             disabled_style="opacity: 0.5;"
///         />
///     }
/// }
/// ```
///
/// # Behavior
/// - The `selected` property determines whether the radio button is selected by default. If set to `true`, the radio button is selected on render.
/// - The `disabled` property disables the radio button, preventing any user interaction. It also changes the appearance of the radio button based
///    on the `disabled_style` and `disabled_class` properties.
/// - The `on_click` callback is triggered when the radio button is clicked. It receives the `value` of the radio button as a parameter.
/// - The `value` property is used to track which option is selected within a group of radio buttons.
///
/// # Notes
/// - The radio button is often used as part of a `Group` of radio buttons, where only one option can be selected at a time.
/// - Custom inline styles and CSS classes provide flexibility in the visual presentation of the radio button.
/// - The `size`, `type`, and `selected` properties allow for a customizable user experience.
/// - The `animation_style` and `animation_class` properties enable animations like hover or transition effects for enhanced interaction.
#[component]
pub fn Radio(
    /// Label for the radio button.
    ///
    /// This is the text label associated with the radio button. It is displayed
    /// next to the radio button to indicate its meaning. The default value is an
    /// empty string, meaning no label will be displayed unless specified.
    #[prop(default = "")]
    label: &'static str,

    /// Value of the radio button.
    ///
    /// The value that this radio button represents when it is selected. It can
    /// be used to identify the selected option in a group of radio buttons.
    /// Defaults to an empty string.
    #[prop(default = "")]
    value: &'static str,

    /// Image source (optional).
    ///
    /// An optional image that can be displayed alongside the radio button. If
    /// no image is provided, this field defaults to an empty string, meaning
    /// no image will be shown.
    #[prop(default = "")]
    src: &'static str,

    /// Inline styles for the container.
    ///
    /// These are the inline styles applied to the container element of the radio button.
    /// This allows custom styling of the radio button's wrapper element. Defaults to an empty string.
    #[prop(default = "")]
    style: &'static str,

    /// CSS class for the container.
    ///
    /// This applies a CSS class to the container element, allowing for custom styling.
    /// The default value is an empty string, meaning no class is applied unless specified.
    #[prop(default = "")]
    class: &'static str,

    /// Inline styles for the label.
    ///
    /// Inline styles that apply specifically to the label text of the radio button.
    /// Defaults to an empty string.
    #[prop(default = "")]
    label_style: &'static str,

    /// CSS class for the label.
    ///
    /// The CSS class applied to the label element, allowing custom styling of the label.
    /// Defaults to an empty string.
    #[prop(default = "")]
    label_class: &'static str,

    /// Inline styles for the image.
    ///
    /// These styles are applied specifically to the image element, if an image is used
    /// for the radio button. Defaults to an empty string.
    #[prop(default = "")]
    image_style: &'static str,

    /// CSS class for the image.
    ///
    /// The CSS class applied to the image element, if an image is provided for the radio button.
    /// Defaults to an empty string.
    #[prop(default = "")]
    image_class: &'static str,

    /// Size of the radio button.
    ///
    /// This defines the size of the radio button. It is based on the `Size` enum, where
    /// possible values include small, medium, and large. The default value is `Size::XSmall`.
    #[prop(default = Size::XSmall)]
    size: Size,

    /// Type of the radio button.
    ///
    /// This defines the type of radio button. It is based on the `Type` enum, which includes
    /// various types for different radio button styles or behavior. The default is `Type::None`.
    #[prop(default = Type::None)]
    r#type: Type,

    /// Whether the radio button is selected.
    ///
    /// This determines whether the radio button is selected by default. If set to `true`,
    /// the radio button is selected when rendered. Defaults to `false`.
    #[prop(default = false)]
    selected: bool,

    /// Whether the radio button is disabled.
    ///
    /// This disables the radio button, preventing user interaction. If `true`, the radio button
    /// cannot be selected. Defaults to `false`.
    #[prop(default = false)]
    disabled: bool,

    /// Inline styles when selected.
    ///
    /// Custom inline styles that are applied when the radio button is selected. This allows
    /// styling changes to visually indicate that the radio button is in a selected state.
    /// Defaults to an empty string.
    #[prop(default = "")]
    selected_style: &'static str,

    /// CSS class when selected.
    ///
    /// The CSS class applied to the radio button when it is selected. This allows for
    /// styling changes when the radio button is in a selected state. Defaults to an empty string.
    #[prop(default = "")]
    selected_class: &'static str,

    /// Inline styles when disabled.
    ///
    /// Custom inline styles that are applied when the radio button is disabled. This allows
    /// for styling changes to visually indicate that the radio button is disabled.
    /// Defaults to an empty string.
    #[prop(default = "")]
    disabled_style: &'static str,

    /// CSS class when disabled.
    ///
    /// The CSS class applied to the radio button when it is disabled. This allows for
    /// styling changes when the radio button is in a disabled state. Defaults to an empty string.
    #[prop(default = "")]
    disabled_class: &'static str,

    /// Inline styles for animations.
    ///
    /// These inline styles are applied when the radio button has animations, such as a hover
    /// effect or transition. Defaults to an empty string.
    #[prop(default = "")]
    animation_style: &'static str,

    /// CSS class for animations.
    ///
    /// The CSS class applied to the radio button when animations are enabled. Defaults to an empty string.
    #[prop(default = "")]
    animation_class: &'static str,

    /// Inline styles for the hidden input.
    ///
    /// Inline styles applied to the hidden input element associated with the radio button.
    /// This is useful for cases where the input element needs custom styling. Defaults to the
    /// constant value `HIDDEN_INPUT_STYLE`.
    #[prop(default = HIDDEN_INPUT_STYLE)]
    input_style: &'static str,

    /// CSS class for the hidden input.
    ///
    /// CSS class applied to the hidden input element associated with the radio button.
    /// Defaults to an empty string.
    #[prop(default = "")]
    input_class: &'static str,

    /// Callback for the click event.
    ///
    /// This callback is triggered when the radio button is clicked. It passes the `value`
    /// of the radio button as a `String` to the callback function. This allows parent components
    /// to react to clicks on individual radio buttons.
    #[prop(default = Callback::from(|value: String| {}))]
    on_click: Callback<(String,), ()>,
) -> impl IntoView {
    let onclick = move |_: MouseEvent| {
        if !disabled {
            on_click.run((value.to_string(),));
        }
    };

    view! {
        <div
            class=format!(
                "{} {} {}",
                if selected { selected_class } else { "" },
                if disabled { disabled_class } else { "" },
                class
            )
            style=format!(
                "{} {} {} {} {} {}",
                if selected { selected_style } else { "" },
                if disabled { disabled_style } else { "" },
                style,
                animation_style,
                r#type.to_style(),
                size.to_style()
            )
            on:click=onclick
        >
            <input
                r#type="radio"
                value=value
                checked=selected
                disabled=disabled
                style=input_style
                class=input_class
            />
            {if !src.is_empty() {
                Some(view! {
                    <img
                        src=src
                        alt=label
                        style=image_style
                        class=image_class
                    />
                })
            } else {
                None
            }}
            <span style=label_style class=label_class>{label}</span>
        </div>
    }
}
