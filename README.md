# URL shortener

Built purely in rust, no infrastructure, thanks to [shuttle.rs](https://shuttle.rs/).

## How to use it

You can use this URL shortener directly from your terminal. Just copy and paste this command to your terminal and replace `<URL>` with the URL that you want to shorten

```bash
curl -X POST -d '<URL>' https://kurz.shuttleapp.rs
```

like this

```bash
curl -X POST -d 'https://twitter.com/_arihantbansal' https://kurz.shuttleapp.rs
```

You will get the shortened URL back. Example: `https://kurz.shuttleapp.rs/MWG1Ur`
