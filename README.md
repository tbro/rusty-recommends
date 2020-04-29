# rusty-recommends

A simple cli to help you discover new music. Based on artist and track name that you provide,
it will recommend another song you may enjoy.

    cargo run -- -a Bob -t "Rolling Stone"

## credentials

It uses Spotify API under the hood, so you need an API key and secret if you don't want
it to ignore you. You can set those in the environment or place a `.env` file in the root
directory of the code. [Follow these instructions](https://developer.spotify.com/documentation/web-api/quick-start) to setup your credentials.

## about

This is in part to get some practice coding with **rust**. Its very simple but there are a lot 
of potential rabbit holes around tweaking the recommendations. By the same token, a lot can be 
done around twerking them, but we'll leave that up to you. There are probably rough edges.
