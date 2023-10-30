# rs-xkcd-bot
Telegram bot built with teloxide that retrieves a random XKCD comic on command. Redis is used to apply a cooldown in order to avoid abuse.

## Docker Installation
Before building the container a recipe file from [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) must be present in order to cache the Docker layers.

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

## Note
This program is designed to get links from [https://xkcd.com/](https://xkcd.com/). Please ensure that your use of this software adheres to the owner's guidelines.
