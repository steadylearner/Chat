/// An example of a chat web application server
extern crate ws;
use ws::{listen, Handler, Message, Request, Response, Result, Sender};

// This can be read from a file
static INDEX_HTML: &'static [u8] = br#"
<!doctype html>
<html>

<head>
  <title>Socket.IO chat</title>
  <!-- <link rel="stylesheet" href="/main.css"> -->
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }

    body {
      font: 13px Helvetica, Arial;
    }

    form {
      background: #000;
      padding: 3px;
      position: fixed;
      bottom: 0;
      width: 100%;
    }

    form input {
      border: 0;
      padding: 10px;
      width: 90%;
      margin-right: .5%;
    }

    form button {
      width: 9%;
      background: rgb(130, 224, 255);
      border: none;
      padding: 10px;
    }

    #messages {
      list-style-type: none;
      margin: 0;
      padding: 0;
    }

    #messages li {
      padding: 5px 10px;
    }

    #messages li:nth-child(odd) {
      background: #eee;
    }

  </style>
</head>

<body>
  <ul id="messages"></ul>
  <form id="form">
    <input type="text" id="msg">
    <button>Send</button>
  </form>
</body>

<script>
  const socket = new WebSocket("ws://" + window.location.host + "/ws");

  const form = document.getElementById("form");
  form.addEventListener('submit', function (event) {
    event.preventDefault();
    const input = document.getElementById("msg");
    socket.send(input.value);
    input.value = "";
  });

  socket.onmessage = function (event) {
    const messages = document.getElementById("messages");
    const li = document.createElement("li");
    li.append(event.data)
    messages.append(li);
  };

</script>

</html>
    "#;

// <script src="https://code.jquery.com/jquery-1.11.1.js"></script>

// Server web application handler
struct Server {
    out: Sender,
}

impl Handler for Server {
    //
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        // Using multiple handlers is better (see router example)
        match req.resource() {
            // The default trait implementation
            "/ws" => Response::from_request(req),

            // Create a custom response
            "/" => Ok(Response::new(200, "OK", INDEX_HTML.to_vec())),

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    // Handle messages recieved in the websocket (in this case, only on /ws)
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Broadcast to all connections
        self.out.broadcast(msg)
    }
}

fn main() {
    // Listen on an address and call the closure for each connection
    listen("127.0.0.1:8000", |out| Server { out }).unwrap()
}
