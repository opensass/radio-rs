# 🧬 Radio RS Dioxus Usage

Adding Radio RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the Radio component to your dependencies by including it in your `Cargo.toml` file.

   ```sh
   cargo add radiors --features=dio
   ```

1. Import the `Radio` components into your Dioxus component and start using it in your app.

## 🛠️ Usage

Incorporating the Accordion component into your application is easy. Follow these steps:

1. Import the Radio components into your Dioxus project:

   ```rust
   use dioxus::prelude::*;
   use radiors::dioxus::{Radio, Group};
   use radiors::{Size, Orientation};
   ```

1. Use the `Radio` components within your Dioxus application:

   ```rust
   use dioxus::prelude::*;
   use radiors::dioxus::{Radio, Group};
   use radiors::{Size, Orientation};

   #[component]
   pub fn app() -> Element {
       let mut selected_value = use_signal(|| "Option1".to_string());

       rsx! {
           Group {
               selected: selected_value(),
               onchange: move |new_value| {
                   selected_value.set(new_value);
               },
               orientation: Orientation::Vertical,
               class: "radio-group",
               Radio {
                   label: "Option 1",
                   value: "Option1",
                   class: "radio-button"
               }
               Radio {
                   label: "Option 2",
                   value: "Option2",
                   class: "radio-button"
               }
           }
       }
   }
   ```

## 🔧 Props

### `Group` Props

#### Main Props

| Property   | Type      | Description                                          | Default |
| ---------- | --------- | ---------------------------------------------------- | ------- |
| `selected` | `String`  | The currently selected value of the radio group.     | `""`    |
| `children` | `Element` | Child `Radio` components to render within the group. | `""`    |

#### Styling Props

```sh
+-----------------------------------------------------------+
|                  [Group Container]                        |  <-- `class` & `style`
|                                                           |
|   +-----------------------------------------------+       |  <-- `orientation`
|   |              [Child Radio Buttons]            |       |  <-- `children`
|   +-----------------------------------------------+       |
|                                                           |
+-----------------------------------------------------------+
```

| Property      | Type           | Description                                                  | Default                   |
| ------------- | -------------- | ------------------------------------------------------------ | ------------------------- |
| `style`       | `&'static str` | Inline styles for the radio group container.                 | `""`                      |
| `class`       | `&'static str` | CSS class for the radio group container.                     | `""`                      |
| `orientation` | `Orientation`  | Orientation of the radio group (`Horizontal` or `Vertical`). | `Orientation::Horizontal` |

#### Behavioral Props

| Property   | Type               | Description                                         | Default |
| ---------- | ------------------ | --------------------------------------------------- | ------- |
| `onchange` | `Callback<String>` | Callback triggered when the selected value changes. | No-op   |

### `Radio` Props

#### Main Props

| Property   | Type           | Description                                             | Default |
| ---------- | -------------- | ------------------------------------------------------- | ------- |
| `label`    | `&'static str` | Text label displayed alongside the radio button.        | `""`    |
| `value`    | `&'static str` | Unique value for the radio button.                      | `""`    |
| `src`      | `&'static str` | Optional image URL to display next to the radio button. | `""`    |
| `selected` | `bool`         | Indicates whether this radio button is selected.        | `false` |
| `disabled` | `bool`         | Disables the radio button when `true`.                  | `false` |

#### Styling Props

```sh
+-----------------------------------------------------------+
|                   [Radio Container]                       |  <-- `class` & `style`
|                                                           |
|   +-------------------------------------------------+     |  <-- `selected_class` & `selected_style` (when selected)
|   |               [Radio Button]                    |     |  <-- `disabled_class` & `disabled_style` (when disabled)
|   |                                                 |     |
|   |   +---------------------------------------+     |     |
|   |   |           [Hidden Input]              |     |     |  <-- `input_class` & `input_style`
|   |   +---------------------------------------+     |     |
|   |                                                 |     |
|   |   +---------------------------------------+     |     |
|   |   |         [Optional Image]              |     |     |  <-- `image_class` & `image_style`
|   |   +---------------------------------------+     |     |
|   |                                                 |     |
|   |   +---------------------------------------+     |     |
|   |   |          [Radio Label]                |     |     |  <-- `label_class` & `label_style`
|   |   +---------------------------------------+     |     |
|   +-------------------------------------------------+     |
|                                                           |
+-----------------------------------------------------------+
```

| Property          | Type           | Description                                                      | Default              |
| ----------------- | -------------- | ---------------------------------------------------------------- | -------------------- |
| `style`           | `&'static str` | Custom inline styles for the radio container.                    | `""`                 |
| `class`           | `&'static str` | CSS class for the radio container.                               | `""`                 |
| `label_style`     | `&'static str` | Inline styles for the radio label.                               | `""`                 |
| `label_class`     | `&'static str` | CSS class for the radio label.                                   | `""`                 |
| `image_style`     | `&'static str` | Inline styles for the image (if `src` is provided).              | `""`                 |
| `image_class`     | `&'static str` | CSS class for the image (if `src` is provided).                  | `""`                 |
| `size`            | `Size`         | Size of the radio button (`Small`, `Medium`, `Large`).           | `Size::XSmall`       |
| `type`            | `Type`         | Styling type of the radio button (e.g., `Primary`, `Secondary`). | `Type::None`         |
| `selected_style`  | `&'static str` | Inline styles for the selected state of the radio button.        | `""`                 |
| `selected_class`  | `&'static str` | CSS class for the selected state of the radio button.            | `""`                 |
| `disabled_style`  | `&'static str` | Inline styles for the disabled state of the radio button.        | `""`                 |
| `disabled_class`  | `&'static str` | CSS class for the disabled state of the radio button.            | `""`                 |
| `animation_style` | `&'static str` | Inline styles for animations applied to the radio button.        | `""`                 |
| `animation_class` | `&'static str` | CSS class for animations applied to the radio button.            | `""`                 |
| `input_style`     | `&'static str` | Inline styles for the hidden `<input>` element.                  | `HIDDEN_INPUT_STYLE` |
| `input_class`     | `&'static str` | CSS class for the hidden `<input>` element.                      | `""`                 |

#### Behavioral Props

| Property  | Type               | Description                                          | Default |
| --------- | ------------------ | ---------------------------------------------------- | ------- |
| `onclick` | `Callback<String>` | Callback triggered when the radio button is clicked. | No-op   |

## 💡 Notes

- Use the `Group` component to manage state for multiple `Radio` components.
- Callback props like `onchange` and `onclick` allow you to handle user interactions effectively.
- Make sure that the `value` for each `Radio` is unique within the `Group` to avoid conflicts.
- Customize the layout with the `orientation` prop (`Horizontal` or `Vertical`).
- Enhance the appearance by applying custom classes and styles or using any CSS framework.
