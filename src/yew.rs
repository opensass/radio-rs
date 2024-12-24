use crate::common::{Orientation, Size, Type, HIDDEN_INPUT_STYLE};
use std::rc::Rc;
use yew::prelude::*;

/// Properties for configuring the `Group` component.
///
/// The `Group` component allows you to create a group of radio buttons with customizable
/// styles, orientation, and behavior. This is useful for creating user-selectable options
/// presented as a single group.
///
/// It supports horizontal or vertical layouts, CSS customizations, and child components of type `Radio` only.
#[derive(Properties, Clone, PartialEq)]
pub struct GroupProps {
    /// The selected value of the radio group.
    ///
    /// This represents the current value selected in the group. It can be bound to a state
    /// to reflect changes dynamically. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub selected: String,

    /// Callback for when the selected value changes.
    ///
    /// This callback is triggered whenever the user selects a different radio button. It
    /// provides the new selected value as a string. Defaults to a no-op.
    #[prop_or_default]
    pub onchange: Callback<String>,

    /// Orientation of the radio buttons group (horizontal or vertical).
    ///
    /// Determines the layout of the radio buttons. The available options are:
    /// - `Orientation::Horizontal`: Displays the radio buttons side by side.
    /// - `Orientation::Vertical`: Stacks the radio buttons vertically.
    ///   Defaults to `Orientation::Horizontal`.
    #[prop_or_default]
    pub orientation: Orientation,

    /// Additional inline styles for the container.
    ///
    /// Allows for custom inline styles to be applied directly to the group container.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub style: &'static str,

    /// Additional CSS classes for the container.
    ///
    /// Enables adding custom CSS classes to the group container for further styling or
    /// overriding default styles. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub class: &'static str,

    /// Child components for the group.
    ///
    /// This property allows you to pass one or more `Radio` components as children of the
    /// `Group` component. The children will be arranged based on the specified `orientation`.
    /// Defaults to an empty list of children if not provided.
    #[prop_or_default]
    pub children: ChildrenWithProps<Radio>,
}

/// Group Component
///
/// A Yew component for creating a group of radio buttons with customizable styles,
/// orientation, and behavior. The `Group` component arranges child `Radio` components
/// horizontally or vertically and handles selection state management.
///
/// # Properties
/// The component uses the `GroupProps` struct for its properties. Key properties include:
///
/// - **selected**: The selected value of the radio group (`String`). Default: `""`.
/// - **onchange**: Callback triggered when the selected value changes (`Callback<String>`). Default: no-op.
/// - **orientation**: The layout of the radio buttons (horizontal or vertical) (`Orientation`). Default: `Orientation::Horizontal`.
/// - **style**: Custom inline styles for the container (`&'static str`). Default: `""`.
/// - **class**: Additional CSS classes for the container (`&'static str`). Default: `""`.
/// - **children**: A collection of `Radio` components as children (`ChildrenWithProps<Radio>`). Default: empty.
///
/// # Features
/// - Supports dynamic selection of radio buttons with state binding.
/// - Allows horizontal or vertical orientation via the `Orientation` enum.
/// - Customizable through inline styles and CSS classes.
/// - Only accepts `Radio` components as children.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use yew::prelude::*;
/// use radiors::yew::{Group, Radio};
/// use radiors::Orientation;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let selected = use_state(|| "Option1".to_string());
///     let on_change = {
///         let selected = selected.clone();
///         Callback::from(move |new_value: String| {
///             selected.set(new_value);
///         })
///     };
///
///     html! {
///         <Group selected={(*selected).clone()} onchange={on_change}>
///             <Radio value="Option1" label="Option 1" />
///             <Radio value="Option2" label="Option 2" />
///             <Radio value="Option3" label="Option 3" />
///         </Group>
///     }
/// }
/// ```
///
/// ## Vertical Orientation
/// ```rust
/// use yew::prelude::*;
/// use radiors::yew::{Group, Radio};
/// use radiors::Orientation;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let selected = use_state(|| "Option1".to_string());
///
///     html! {
///         <Group
///             selected={(*selected).clone()}
///             orientation={Orientation::Vertical}
///             style="margin: 20px"
///         >
///             <Radio value="Option1" label="Option 1" />
///             <Radio value="Option2" label="Option 2" />
///             <Radio value="Option3" label="Option 3" />
///         </Group>
///     }
/// }
/// ```
///
/// ## Custom Styling
/// ```rust
/// use yew::prelude::*;
/// use radiors::yew::{Group, Radio};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Group
///             selected="Option1"
///             style="border: 1px solid black; padding: 10px"
///             class="custom-radio-group"
///         >
///             <Radio value="Option1" label="Option 1" />
///             <Radio value="Option2" label="Option 2" />
///         </Group>
///     }
/// }
/// ```
///
/// # Behavior
/// - The `Group` component dynamically manages the selection of its child `Radio` components.
/// - Clicking a radio button updates the `selected` value and triggers the `onchange` callback.
/// - The `orientation` property defines whether the radio buttons are arranged horizontally or vertically.
///
/// # Notes
/// - The `children` property is required to be of type `Radio`. Passing other components will result in a compilation error.
/// - The `onchange` callback receives the `value` of the newly selected `Radio` as a `String`.
/// - Custom styles and classes can be used to enhance the layout and appearance of the group container.
#[function_component(Group)]
pub fn group(props: &GroupProps) -> Html {
    let selected = props.selected.clone();
    let onchange = props.onchange.clone();

    html! {
        <div
            class={props.class}
            style={format!(
                "{} {}",
                props.orientation.to_style(),
                props.style
            )}
        >
            { for props.children.iter().map(|mut child| {
                let props = Rc::make_mut(&mut child.props);
                let is_selected = props.value == selected;
                let onclick = {
                    let onchange = onchange.clone();
                    let value = props.value.to_string();
                    Callback::from(move |_| {
                        onchange.emit(value.clone());
                    })
                };

                props.selected = is_selected;
                props.on_click = onclick;

                child
            }) }
        </div>
    }
}

/// Properties for configuring the `Radio` component.
///
/// The `Radio` component allows the creation of individual, customizable radio buttons.
/// It supports various configurations for appearance, behavior, and styles.
#[derive(Properties, Clone, PartialEq)]
pub struct RadioProps {
    /// The label for the radio button.
    ///
    /// Defines the text displayed next to the radio button.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub label: &'static str,

    /// The value for the radio button.
    ///
    /// This value represents the data associated with the radio button, used to identify
    /// it in the `Group`'s selection context.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub value: &'static str,

    /// Optional image URL for the radio button.
    ///
    /// If provided, the radio button can display an image along with its label.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub src: &'static str,

    /// Inline styles for the container.
    ///
    /// Allows applying custom inline CSS styles directly to the radio button's container.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub style: &'static str,

    /// CSS class for the container.
    ///
    /// Adds custom CSS classes to style the radio button's container.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline styles for the label.
    ///
    /// Enables the addition of custom inline CSS styles to the label text.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub label_style: &'static str,

    /// CSS class for the label.
    ///
    /// Adds custom CSS classes to style the radio button's label.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub label_class: &'static str,

    /// Inline styles for the image.
    ///
    /// Allows applying custom inline CSS styles to the image associated with the radio button.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub image_style: &'static str,

    /// CSS class for the image.
    ///
    /// Adds custom CSS classes to style the image associated with the radio button.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub image_class: &'static str,

    /// The size of the radio button.
    ///
    /// Specifies the size of the radio button. Possible values are:
    /// - `Size::XSmall`
    /// - `Size::Small`
    /// - `Size::Medium`
    /// - etc
    ///   Defaults to `Size::XSmall` if not provided.
    #[prop_or_default]
    pub size: Size,

    /// The styling type of the radio button.
    ///
    /// Determines the visual theme of the radio button, such as Primary or Secondary styles.
    /// Defaults to `Type::None` if not provided.
    #[prop_or_default]
    pub r#type: Type,

    /// Whether this radio button is selected.
    ///
    /// Indicates if the radio button is currently selected. Managed internally in the `Group`
    /// component context. Defaults to `false`.
    #[prop_or_default]
    pub selected: bool,

    /// Whether this radio button is disabled.
    ///
    /// If `true`, the radio button is not clickable and appears disabled.
    /// Defaults to `false`.
    #[prop_or_default]
    pub disabled: bool,

    /// Inline styles applied when the radio button is selected.
    ///
    /// Provides custom styles for the radio button in the selected state.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub selected_style: &'static str,

    /// CSS class applied when the radio button is selected.
    ///
    /// Adds custom CSS classes to style the radio button in the selected state.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub selected_class: &'static str,

    /// Inline styles applied when the radio button is disabled.
    ///
    /// Provides custom styles for the radio button in the disabled state.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub disabled_style: &'static str,

    /// CSS class applied when the radio button is disabled.
    ///
    /// Adds custom CSS classes to style the radio button in the disabled state.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub disabled_class: &'static str,

    /// Inline styles for animations applied to the radio button.
    ///
    /// Enables applying custom styles to animations or transitions for the radio button.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub animation_style: &'static str,

    /// CSS class for animations applied to the radio button.
    ///
    /// Adds custom CSS classes to handle animations or transitions for the radio button.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub animation_class: &'static str,

    /// Inline styles for the hidden input element.
    ///
    /// Provides custom styles for the hidden `<input>` element used for the radio button.
    /// Defaults to `HIDDEN_INPUT_STYLE`.
    #[prop_or(HIDDEN_INPUT_STYLE)]
    pub input_style: &'static str,

    /// CSS class for the hidden input element.
    ///
    /// Adds custom CSS classes for the hidden `<input>` element.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub input_class: &'static str,

    /// Callback for when the radio button is clicked.
    ///
    /// Triggered whenever the user clicks on the radio button. It provides the `value` of the
    /// radio button as a string to the callback.
    /// Defaults to a no-op.
    #[prop_or_default]
    pub onclick: Callback<String>,

    /// Internal callback triggered when the radio button is clicked.
    ///
    /// This property is intended for use by the parent `Group` component to manage
    /// the state of the radio group. It is not exposed for direct use by end users.
    ///
    /// The callback receives the `value` of the clicked radio button as a `String`.
    #[prop_or_default]
    on_click: Callback<String>,
}

/// Radio Component
///
/// A Yew component for individual, customizable radio buttons. The `Radio` component is designed
/// to work in conjunction with the `Group` component to create interactive radio groups.
///
/// # Properties
/// The component uses the `RadioProps` struct for its properties. Key properties include:
///
/// - **label**: The text displayed next to the radio button (`&'static str`). Default: `""`.
/// - **value**: The value of the radio button (`&'static str`). Default: `""`.
/// - **src**: Optional image URL to display next to the radio button (`&'static str`). Default: `""`.
/// - **style**: Custom inline styles for the radio button container (`&'static str`). Default: `""`.
/// - **class**: CSS class for the radio button container (`&'static str`). Default: `""`.
/// - **selected**: Whether the radio button is currently selected (`bool`). Default: `false`.
/// - **disabled**: Whether the radio button is disabled (`bool`). Default: `false`.
/// - **onclick**: Callback triggered when the radio button is clicked (`Callback<String>`). Default: no-op.
///
/// # Features
/// - Customizable label text and optional images.
/// - Inline styles and CSS classes for styling.
/// - Supports disabled and selected states.
/// - Works seamlessly within the `Group` component.
///
/// # Examples
///
/// ## Basic Radio Button
/// ```rust
/// use yew::prelude::*;
/// use radiors::yew::Radio;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Radio label="Option 1" value="Option1" />
///     }
/// }
/// ```
///
/// ## Radio Button with Image
/// ```rust
/// use yew::prelude::*;
/// use radiors::yew::Radio;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Radio
///             label="Option 1"
///             value="Option1"
///             src="assets/img.png"
///         />
///     }
/// }
/// ```
///
/// ## Custom Styles
/// ```rust
/// use yew::prelude::*;
/// use radiors::yew::Radio;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Radio
///             label="Option 1"
///             value="Option1"
///             style="background-color: lightblue; padding: 5px"
///             class="custom-radio"
///         />
///     }
/// }
/// ```
///
/// # Behavior
/// - Clicking the radio button emits the `onclick` callback with its `value`.
/// - `on_click` is managed internally by the `Group` component for handling selection state.
/// - The `disabled` property prevents interaction when set to `true`.
///
/// # Notes
/// - The `selected` and `on_click` properties are typically managed by the parent `Group` component.
#[function_component(Radio)]
pub fn radio(props: &RadioProps) -> Html {
    let onclick = {
        let on_click = props.on_click.clone();
        let onclick = props.onclick.clone();
        let value = props.value.to_string();
        let disabled = props.disabled;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if !disabled {
                on_click.emit(value.clone());
                onclick.emit(value.clone());
            }
        })
    };
    let selected = props.selected;
    let disabled = props.disabled;

    let selected_style = props.selected_style;
    let disabled_style = props.disabled_style;
    let style = props.style;
    let animation_style = props.animation_style;

    let selected_class = props.selected_class;
    let disabled_class = props.disabled_class;
    let class = props.class;
    let animation_class = props.animation_class;

    let size = props.size.to_style();
    let style_type = props.r#type.to_style();

    html! {
        <div
            class={format!(
                "{} {} {}",
                if props.selected { props.selected_class } else { "" },
                if props.disabled { props.disabled_class } else { "" },
                props.class,
            )}
            style={format!(
                "{} {} {} {} {} {}",
                if props.selected { props.selected_style } else { "" },
                if props.disabled { props.disabled_style } else { "" },
                props.style,
                props.animation_style,
                style_type,
                size,
            )}
            onclick={onclick}
            onmouseover={let size = size.clone();
                let style_type = style_type.clone();
                Callback::from(move |e: MouseEvent| {
                let target = e.target_dyn_into::<web_sys::HtmlElement>().unwrap();
                if target.tag_name() == "DIV" {
                    target.set_attribute("style", &format!(
                        "{} {} {} {} {} {}",
                        if selected { selected_style } else { "" },
                        if disabled { disabled_style } else { "" },
                        style,
                        animation_style,
                        size,
                        style_type
                    )).unwrap();
                    target.set_attribute("class", &format!(
                        "{} {} {} {}",
                        if selected { selected_class } else { "" },
                        if disabled { disabled_class } else { "" },
                        class,
                        animation_class
                    )).unwrap();
                }
            })}
            onmouseleave={let size = size.clone();
                let style_type = style_type.clone();
                Callback::from(move |e: MouseEvent| {
                let target = e.target_dyn_into::<web_sys::HtmlElement>().unwrap();
                if target.tag_name() == "DIV" {
                    target.set_attribute("style", &format!(
                        "{} {} {} {} {}",
                        if selected { selected_style } else { "" },
                        if disabled { disabled_style } else { "" },
                        style,
                        size,
                        style_type
                    )).unwrap();
                    target.set_attribute("class", &format!(
                        "{} {} {}",
                        if selected { selected_class } else { "" },
                        if disabled { disabled_class } else { "" },
                        class,
                    )).unwrap();
                }
            })}
        >
            <input
                type="radio"
                name="radio"
                value={props.value}
                checked={props.selected}
                disabled={props.disabled}
                style={props.input_style}
                class={props.input_class}
            />
            { if !props.src.is_empty() {
                html! {
                    <img
                        src={props.src}
                        alt={props.label}
                        style={props.image_style}
                        class={props.image_class}
                    />
                }
            } else {
                html! {}
            } }
            <span style={props.label_style} class={props.label_class}>{ props.label }</span>
        </div>
    }
}
