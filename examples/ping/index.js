import { Server } from "../../pkg";

let input = document.createElement("input");
let button = document.createElement("button");

button.innerText = "ping";

document.body.append(input);
document.body.append(button);

button.addEventListener("click", (_) => {
  let server = new Server(input.value);
  console.log(server.ping);
  server.ping.then((msg) => {
    console.log(msg);
  });
});
