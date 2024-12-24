# 🌱 Leptos Radio RS Usage

Adding Radio RS to your Leptos project is simple:

1. Make sure your project is set up with Leptos. Refer to their [Getting Started Guide](https://book.leptos.dev/getting_started/index.html) for setup instructions.

1. Add `radiors` to your dependencies:

   ```sh
   cargo add radiors --features=lep
   ```

1. Import the `Radio` components into your Leptos component and start using it in your app.

## 🛠️ Usage

Incorporating the Radio components into your Leptos application is easy. Follow these steps:

1. Import the Radio components into your Leptos project:

   ```rust
   use leptos::prelude::*;
   use radiors::leptos::{Radio, Group};
   use radiors::{Size, Orientation};
   ```

1. Use the `Radio` components within your Leptos application:

   ```rust
   use leptos::prelude::*;
   use radiors::leptos::{Radio, Group};
   use radiors::{Size, Orientation};

   #[component]
   pub fn app() -> impl IntoView {
       let selected_value = signal("Option1".to_string());

       view! {
           <Group
               selected={selected_value.0.get()}
               onchange=Callback::from(move |new_value: String| {
                   selected_value.1.set(new_value);
               })
               orientation=Orientation::Vertical
               class="radio-group"
           >
               <Radio
                   label="Option 1"
                   value="Option1"
                   class="radio-button"
               />
               <Radio
                   label="Option 2"
                   value="Option2"
                   class="radio-button"
               />
           </Group>
       }
   }
   ```

## 🔧 Props

### `Group` Props

#### Main Props

| Property   | Type      | Description                                          | Default |
| ---------- | --------- | ---------------------------------------------------- | ------- |
| `selected` | `String`  | The currently selected value of the radio group.     | `""`    |
| `children` | `ChildrenFragment` | Child `Radio` components to render within the group. | `""`    |

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
