import Vue from 'vue'
import Cookie from 'js-cookie'

class Callback {
  constructor(callback, once) {
    this.callback = callback
    this.once = once
  }
}

export default {
  /**
   * @type {WebSocket}
   */
  rawsock: undefined,


  /**
   * @type {Map<String, Callback>}
   */
  _listeners: new Map(),

  init() {
    Vue.prototype.$socket = this
    this.try_connecting_ws()

    this.rawsock.onmessage = (msg) => {
      let [sid, method, payload] = msg.data.split('\u001f')

      if (sid == undefined || method == undefined || payload == undefined) return
      // eslint-disable-next-line
      console.log(`Received message under session-id ${sid} with method ${method} and payload ${payload}`)

      const cb = this._listeners.get(method)

      const pp = JSON.parse(payload)

      pp.sessionID = sid

      if (cb != undefined) {
        cb.callback(pp)
        if (cb.once) {
          this._listeners.delete(method)
        }
      }
    }

  },

  addListenerConstant(method, callback) {
    this._listeners.set(method, new Callback(callback, false))
  },

  addListenerOnce(method, callback) {
    this._listeners.set(method, new Callback(callback, true))
  },

  try_connecting_ws() {
    this.rawsock = new WebSocket("ws://192.168.178.30:443")
  },

  is_connected() {
    return this.rawsock.readyState == 1
  },

  /**
   * Tries to get authentification from server
   * @param {string} username 
   * @param {string} passw 
   */
  sendLoginRequest(username, passw) {
    this.sendRequest('user.login',  {user: username, password: passw})
  },

  sendRequest(method, payload) {
    let req = this.buildReq(method, payload)
    this.rawsock.send(req)
  },

  /**
   * 
   * @param {String} messsage 
   * @param {Object} data 
   */
  buildReq(messsage, data) {
    let sessionID = Cookie.get('sid')
    if (sessionID == undefined || sessionID.length < 10) {
      sessionID = 0
    } 
    return `${sessionID}\u001f${messsage}\u001f${JSON.stringify(data)}`
  }

}
