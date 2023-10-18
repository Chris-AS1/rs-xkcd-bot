Telegram bot built with teloxide that retrieves a random XKCD on command. Applies a cooldown to avoid abuse.

## Docker Installation
Before building the container a recipe file used from cargo-chef to cache build layers must be present.

 ```bash
 cargo chef prepare --recipe-path recipe.json
 ```

After that the container image can be built, for simplicity let's use docker compose:
```bash
docker compose build
```

To run the whole stack use:
```bash
docker compose up
```
