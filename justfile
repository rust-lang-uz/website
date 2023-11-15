dev:
    trunk serve
    
css:
    pnpx tailwindcss -i ./style/input.css -o ./style/output.css --watch

build:
    pnpx tailwindcss -i ./style/input.css -o ./style/output.css
    trunk build --release