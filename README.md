# WeDo

Get things done together

---

## Features

WeDo is a self-hostable, open source, todo-list designed around collaboration.

## Installation

```sh
# clone the repository
git clone https://github.com/Nqtural/wedo.git

# build frontend
cd wedo/frontend
bun run build

# install frontend
sudo rm /path/to/frontend/directory/ && cp -r dist/* /path/to/frontend/directory/

# frontend can be served using something like nginx

# build backend
cd ../backend
cargo build --release

# install backend
sudo cp target/release/wedo-backend .env /path/to/server/directory/

# start backend (must be executed in the directory with .env file)
cd /path/to/server/directory/
./wedo-backend
```

## Usage

Home page is presented by navigating to /lists. Here you can create and manage
task lists, which are collections of tasks. Clicking one takes you to the task
list. Here you can see the tasks in the list and their completed-status, as
well as manage them.
