# GastroMan
A Vue.js gastronomy manager with a rust-backend


## WebSocketProtocol used by GastroMan
GastroMan uses a simple WebSocket protocol, seperating Session-ID (0 if not existing), Method and Payload (JSON)

The three blocks are separated by the UTF8-Char u001F (INFORMATION SEPARATOR ONE)

### Methods of GastroMan protocol
* user
    * user.login
    Request from frontend to server
        * user.login.success
        * user.login.error