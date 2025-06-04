# XChat Clone

A secure messaging application with vanishing messages, file sharing, and audio/video calling, built in Rust with Bitcoin-style encryption.

## Features
- **Vanishing Messages**: Messages auto-delete after a specified time.
- **File Sharing**: Upload and share any file type.
- **Audio/Video Calling**: WebRTC-based peer-to-peer calls.
- **Encryption**: Uses secp256k1 for key pairs and AES-GCM for end-to-end encryption.
- **Hosting**: Can be run locally or deployed online for free.

## Logo

   ____ _          _ _       
  / ___| |__   ___| | | ___  
 | |   | '_ \ / __| | |/ _ \ 
 | |___| | | | (__| | |  __/
  \____|_| |_|____|_|_|\___|


  
## Setup

**Install Rust**:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

**Install Dependencies**
`cargo build`

**Create Uploads Directory**
`mkdir uploads`

**Run Locally**
`cargo run`

## Local Hosting with Docker

**Install Docker**
`sudo apt update
sudo apt install docker.io
sudo systemctl start docker
sudo systemctl enable docker`

**Build and Run**
`docker build -t xchat-clone .
docker run -p 8080:8080 xchat-clone`

## Online Hosting with Fly.io

**Install Flyctl**
`curl -L https://fly.io/install.sh | sh`

## Deploy
`fly auth signup
fly deploy`


## Persistent Storage:

**Create a volume for SQLite**
`fly volumes create xchat_data --region <your-region> --size 1`


## Scalability: For production, switch to PostgreSQL and add a reverse proxy (e.g., Nginx) for load balancing.










