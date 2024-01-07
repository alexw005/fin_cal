# How to host dioxus web app on gh page

## Dioxus setup

- Follow the guild on dioxus v0.4 cookbook https://dioxuslabs.com/learn/0.4/getting_started/wasm

- Then create file `Dioxus.toml` in the main directory

- use command line below to build app (ensure the release is on docs folder same as what is set in the gh page setting)

  `dx build --release`

- wait for github action to deploy the page for you, Enjoy!
