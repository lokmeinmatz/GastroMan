import Vue from 'vue'
import Vuex from 'vuex'
import Cookie from 'js-cookie'
import router from './router'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    logged_in: false,
    username: '',
    fullscreen: false
  },
  getters: {
    isLoggedIn: state => state.logged_in
  },
  mutations: {
    setLoggedIn(state, login_data) {
      state.logged_in = login_data != undefined
      state.username = login_data.username
      Cookie.set('user', login_data.username)
      Cookie.set('sid', login_data.sid)
    },

    logOut(state) {
      Cookie.remove('user')
      Cookie.remove('sid')
      state.username = ''
      state.logged_in = false
      router.push('/login')
      // eslint-disable-next-line
      console.log('Logged out')
    },

    toogleFullscreen(state) {
      state.fullscreen = !state.fullscreen

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

  }
})
