# The Room

A small API project that serves a chat room with 0 authenticating and log in.

Build with Rust using Actix and Diesel.

# Setup

<!-- Not out yet -->

# API Endpoints

## Get all messages

You can get all the messages in the chat room by sending a GET request to the `/messages` endpoint.

```http
GET /messages
```

You typically will use the last query parameter to specify the number of messages you want to get as shown below.

## Get the last X amount of messages

You can get the last X amount of messages by sending a GET request to the `/messages` endpoint with the last query parameter.

```
GET /messages?last=10
```

## Send a message

This is a very simple POST request to send a message to the chat room.

```http
POST /messages
Content-Type: application/json

{
    "username": "John Doe",
    "message": "Hello, World!"
}
```
