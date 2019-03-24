import Vue from 'vue'
import Cookie from 'js-cookie'

export default {
  /**
   * @type {WebSocket}
   */
  rawsock: undefined,

  init() {
    Vue.prototype.$socket = this
    this.try_connecting_ws()
  },

  try_connecting_ws() {
    this.rawsock = new WebSocket("ws://localhost:443")
  },

  is_connected() {
    return this.rawsock.readyState == 1
  },

  /**
   * Tries to get authentification from server
   * @param {string} username 
   * @param {string} passw 
   */
  try_login(username, passw) {
    let req = this.build_req('user.login', {user: username, password: passw})

    this.rawsock.send(req)
  },

  /**
   * 
   * @param {String} messsage 
   * @param {Object} data 
   */
  build_req(messsage, data) {
    let sessionID = Cookie.get('sid')
    if (sessionID == undefined || sessionID.length < 10) {
      sessionID = 0
    } 
    return `${sessionID}\u001f${messsage}\u001f${JSON.stringify(data)}`
  }

}
