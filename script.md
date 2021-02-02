``` Rust check.rs
use web_sys::console;
let st : String = "Hello".into();
console::log_1(&st.into());

yew::services::ConsoleService::log("msg");

#[wasm_bindgen(inline_js = "export function add(a,b) { return a + b;};")]
extern "C" {
    fn add(a : u32, b : u32) -> u32;
}

#[wasm_bindgen(module = "/src/js/index.js")]
extern "C" {
    fn hello();
    fn change(num : u32);
}

#[wasm_bindgen(module = "/src/js/canvas.js")]
extern "C" {
    fn drawQ();
}
```
### Comments
#### REFACT!
Not good code for rewrite
``` Rust
// REFACT! this logic is not good ~~~~~~~~
```

#### TODO!
ToDo
``` Rust
// TODO! add function fn write() -> i32;
```

#### HACK!
Need more effective or speedy code

``` Rust
// HACK!
```
