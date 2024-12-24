# Y Radio RS Yew Usage

Adding Radio RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Radio RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add radiors --features=yew
   ```

1. Import the `Radio` component into your Yew component and start using it in your app.

## 🛠️ Usage

Incorporating Yew Radio RS into your application is easy. Follow these steps:

1. Import the `Group` and the `Radio` components into your Yew project:

   ```rust
   use yew::prelude::*;
   use radiors::yew::{Radio, Group};
   ```

1. Use the `Radio` component within your Yew application:

   ```rust
   use radiors::yew::{Radio, Group};
   use radiors::{Size, Type, Orientation};
   use yew::prelude::*;

   #[function_component(App)]
   pub fn app() -> Html {
       let selected_value = use_state(|| "Option1".to_string());
       let onchange = {
           let selected_value = selected_value.clone();
           Callback::from(move |new_value: String| {
               selected_value.set(new_value);
           })
       };

       html! {
           <Group
               selected={(*selected_value).clone()}
               onchange={onchange}
               orientation={Orientation::Vertical}
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

### `Group` Component Props

#### Main Props

| Property   | Type                       | Description                                          | Default |
| ---------- | -------------------------- | ---------------------------------------------------- | ------- |
| `selected` | `String`                   | The currently selected value of the radio group.     | `""`    |
| `children` | `ChildrenWithProps<Radio>` | Child `Radio` components to render within the group. | `""`    |

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

### `Radio` Component Props

#### **Main Props**

| Property   | Type           | Description                                             | Default |
| ---------- | -------------- | ------------------------------------------------------- | ------- |
| `label`    | `&'static str` | Text label displayed alongside the radio button.        | `""`    |
| `value`    | `&'static str` | Unique value for the radio button.                      | `""`    |
| `src`      | `&'static str` | Optional image URL to display next to the radio button. | `""`    |
| `selected` | `bool`         | Indicates whether this radio button is selected.        | `false` |
| `disabled` | `bool`         | Disables the radio button when `true`.                  | `false` |

#### **Styling Props**

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

- Use the `Group` component to manage state for multiple `Radio` components easily.
- Callback props like `onchange` and `onclick` allow you to handle user interactions effectively.
- Make sure the `value` for each `Radio` is unique within the `Group` to avoid conflicts.
- The `orientation` prop in the `Group` component helps align the radio buttons vertically or horizontally.
- Customize the appearance using the provided `class` and `style` props or by applying your own CSS (pure css, tailwind, bootstrap, etc).
