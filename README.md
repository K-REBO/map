# map
<strong>[map](https://map.oberk.dev) is local map for school.</strong><br>
Using [wasm](https://webassembly.org/)
It’s help to find place like restroom. 
Has AR navigation and dijkstra search. 
We love pull request and suggestion. 
Made with [yew 🦀](https://yew.rs), AR.js  A-Frame. 
v0.2.0 is Demo version.
Coming soon v0.3.0!!

## Hosting
---
### Trunk 🧳
```bash
cargo install trunk
trunk
```
### Wasm-pack 📦✨ + miniserve
Don’t need to miniserve. 
Use server you prefer. 
``` bash
cargo install wasm-pack miniserve
wasm-pack build --target web --out-name wasm --out-dir ./static
miniserve static/ --index index.HTML
```
---
## Todo
#### v0.2.1
##### Add
- [ ] Full Marker
- [ ] Tutorial modal
- [ ] Page(About chengeLog...)
- [ ] show version
##### Change
- [ ] sidebar width
- [ ] Github Link

#### v0.3.0
##### Add
- [ ] clickable canvas
- [ ] Map Colorize
- [ ] fetch Place data

#### later
##### Add
- [ ] Auth
- [ ] sharing where you are

#### Yew Component
- [ ] Toast

``` Rust
struct Toast {
    time: u16,
    content: &str,
    position: Position,
    id: string,/*format!(“Toast{}{}{}”,
                         hash(content),
                         create_data,
                         rnd
                 );*/
}
```
- [ ] Place Dialog
- [ ] Place Card


``` Rust
struct PlaceCard {
    title: &str,
    svg: Html,
    description: &str,
    node: u16,
    distance: u8,// calculate using node
    time: u8,// calculate using node 
    style: &str,
    DataURL: &str,
/* fetched data link which is relative 
and handled static link in fetch function. 
It’s for move placeDialog to get info more. 
*/
}
```



## Contact Bag Question...
Please report issue. 
Or [email map@oberk.dev](mailto:map@oberk.dev)

## License
MIT
