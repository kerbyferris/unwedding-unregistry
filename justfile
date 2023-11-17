run:
  cargo leptos watch
deploy:
  fly deploy
build:
  cargo leptos build --release -vv 
