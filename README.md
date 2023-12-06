# yew-datepicker
```toml
# ...
yew-datepicker = "0.2.0"
pure-rust-locales = "0.7.0"
# ...
```

```rust
use chrono::NaiveDate;
use gloo_console::log;
use pure_rust_locales::Locale;
use yew_datepicker::Datepicker;
// ...
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let on_select = move |date: NaiveDate| {
            log!(date.format("%d.%m.%Y").to_string());
        };
        let locale = Locale::en_US;
        html! {
            <div>
                <h1>{ "This is yew-datepicker usage example" }</h1>
                <Datepicker {on_select} {locale} />
            </div>
        }
    }
// ...
```
