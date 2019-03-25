import Vue from 'vue'
import Vuex from 'vuex'
import Cookie from 'js-cookie'
import router from './router'
import socket from './socket'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    userdata: undefined,
    fullscreen: false,
    showSettings: false,
  },
  getters: {
    isLoggedIn: state => state.userdata != undefined
  },
  mutations: {
    setLoggedIn(state, login_data) {
      state.logged_in = login_data != undefined
      state.userdata = login_data
      Cookie.set('user', login_data.username)
      Cookie.set('sid', login_data.sessionID)
    },
    showSettings(state, show) {
      state.showSettings = show
    },

    logOut(state) {
      Cookie.remove('user')
      Cookie.remove('sid')
      state.userdata = undefined
      router.push('/login')
      // eslint-disable-next-line
      console.log('Logged out')
    },

    setFullscreen(state, full) {
      state.fullscreen = full

      const element = document.documentElement
      if (state.fullscreen) {
        if (element.requestFullscreen) {
          element.requestFullscreen()
        } else if (element.webkitRequestFullscreen) {
          element.webkitRequestFullscreen()
        } else if (element.mozRequestFullScreen) {
          element.mozRequestFullScreen()
        } else if (element.msRequestFullscreen) {
          element.msRequestFullscreen()
        } else {
          // eslint-disable-next-line
          console.log('Fullscreen API is not supported.')
        }
      }
      else {
        if (document.exitFullscreen) {
          document.exitFullscreen()
        } else if (document.webkitExitFullscreen) {
          document.webkitExitFullscreen()
        } else if (document.mozCancelFullScreen) {
          document.mozCancelFullScreen()
        } else if (document.msExitFullscreen) {
          document.msExitFullscreen()
        } else {
          // eslint-disable-next-line
          console.log('Fullscreen API is not supported.')
        }
      }
    }
  },
  actions: {  
    tryLoginAsync(context, {username, password}) {
      return new Promise((resolve, reject) => {
        // eslint-disable-next-line
        socket.addListenerOnce('user.login.success', (r) => {
          context.commit('setLoggedIn', r)
          resolve(r)
        })
        socket.addListenerOnce('user.login.error', (e) => {reject(e)})
        socket.sendLoginRequest(username, password)
      })
    }
  }
})
