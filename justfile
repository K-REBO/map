build:
	wasm-pack build --target web --out-name wasm --out-dir ./static && cp css/output.css static
serve:
	miniserve static/ --index index.html
css:
	cd ~/project/map && windicss src/*.rs src/modules/*.rs src/components/*.rs -o css/output.css -tm && cd css && cat style.css >> output.css
trunk:
	trunk serve

ar:
	cd ~/project/map/static/ && miniserve ar/ --index index.html

deno:
	deno run --allow-net serve.js &



alias c := css
alias b := build
alias s := serve
alias t := trunk
