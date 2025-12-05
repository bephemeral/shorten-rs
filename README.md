# shorten-rs

A simple link shortener written in Rust with Actix Web 

## Features

- /create route for creating a shortened link
- /{id} route for accessing shortened links
- 4 character random alphanumeric IDs
- Performant code written in Actix Web

### (maybe) Coming soon

- Database integration for persistently storing links
- Frontend

Please open a pull request if you are willing to help implement these changes or others

## Deployment

Deploy with [shuttle.dev](https://shuttle.dev)

```
shuttle deploy
```

### Run locally

You can also run the project locally to test before deploying

```
shuttle run
```

## License

This project is licensed under the [MIT License](LICENSE)