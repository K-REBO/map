build:
	wasm-pack build --target web --out-name wasm --out-dir ./static	
serve:
	miniserve static/ --index index.html
css:
	cd ~/project/rs_AR_Map/css && NODE_ENV=production npx tailwindcss build style.css -o output.css && cp output.css ../static
trunk:
	trunk serve
# packed
all: build serve css
trunkAll : css trunk

# alias
b: build
s: serve
a: all
t: trunk
ta: trunkAll
