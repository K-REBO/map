build:
	wasm-pack build --target web --out-name wasm --out-dir ./static
	
serve:
	miniserve static/ --index index.html
edit:
	vim ~/project/rs_AR_Map/justfile
css:
	cd ~/project/rs_AR_Map/css && NODE_ENV=production npx tailwindcss build style.css -o output.css && cp output.css ../static
trunk:
	trunk serve
git:
	git add -A && git commit -m "auto commit for netlify" && git push
# packed
all: build serve css
trunkAll : css trunk

# alias
b: build
s: serve
a: all
t: trunk
ta: trunkAll
allgit : css build git
