# Boa Checker Snek

Used to compete in the Hack Regina Rookie tournament & as an excuse for me to use rust 😊

The code should (mostly) be self-explanatory.

Read the full devlog on my blog: [SquaredX.dev - Building Boa Checker, a Rust-Based Bot for Battlesnakes](https://www.squaredx.dev/building-rust-boa-checker-battlesnake/)

## Technologies Used

This project uses [Rust](https://www.rust-lang.org/) and [Rocket](https://rocket.rs). It also comes with an optional [Dockerfile](https://docs.docker.com/engine/reference/builder/) to help with deployment.

## Run boa-checker-snek

```sh
cargo run
```

## Play a Game Locally

Install the [Battlesnake CLI](https://github.com/BattlesnakeOfficial/rules/tree/main/cli)
* You can [download compiled binaries here](https://github.com/BattlesnakeOfficial/rules/releases)
* or [install as a go package](https://github.com/BattlesnakeOfficial/rules/tree/main/cli#installation) (requires Go 1.18 or higher)

Command to run a local game

```sh
battlesnake play -W 11 -H 11 --name 'Boa Checker' --url http://localhost:8000 -g solo --browser
```

## How to deploy

Use terraform and ensure the AWS CLI is setup on your computer. Navigate to the `terraform` folder and modify `main.tf` where the comments are
- Ensure your key name, instance AMI, and private key path are specified
- I never actually got the setup script to run properly so that is an exercise left to the reader :)
