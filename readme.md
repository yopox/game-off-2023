# Bevy game template

## Try it online

https://yopox.github.io/game-off-2023/

## How to run

- Desktop build:
  
```sh
cargo build
```

- Web build:
```sh
trunk build --release
sed "s@'/@'./@g; s@\"/@\"./@g" ./dist/index.html > ./dist/temp.html
mv ./dist/temp.html ./dist/index.html
```
