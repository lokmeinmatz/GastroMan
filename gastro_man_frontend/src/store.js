import Vue from 'vue'
import Vuex from 'vuex'
import Cookie from 'js-cookie'
import router from './router'
import socket from './socket'
import perm from './tiles/permissions.json'

const permissions = perm
Vue.use(Vuex)


export default new Vuex.Store({
  state: {
    userdata: undefined,
    filteredPermissions: [],
    fullscreen: false,
    showSettings: false,
    admin: {
      allUsers: []
    }
  },
  getters: {
    isLoggedIn: state => state.userdata != undefined,

    validTiles(state) {
      let res = []
      if (state.userdata == undefined) return res
      
      // tiles get managed from ./tiles/permissions.json
      // add admin tiles
      if (state.userdata.permissions.includes("admin") && state.filteredPermissions.includes("admin")) {
        res = res.concat(permissions.admin)
      }

      // add manager tiles
      if (state.userdata.permissions.includes("manager") && state.filteredPermissions.includes("manager")) {
        res = res.concat(permissions.manager)
      }

      // add cook tiles
      if (state.userdata.permissions.includes("cook") && state.filteredPermissions.includes("cook")) {
        res = res.concat(permissions.cook)
      }

      // add waiter tiles
      if (state.userdata.permissions.includes("waiter") && state.filteredPermissions.includes("waiter")) {
        res = res.concat(permissions.waiter)
      }

      return res
    }
  },
  mutations: {
    setLoggedIn(state, login_data) {
      state.userdata = login_data
      Cookie.set('user', login_data.username)
      Cookie.set('sid', login_data.sessionID)

      state.filteredPermissions = login_data.permissions
    },
    showSettings(state, show) {
      state.showSettings = show
    },

    setFilteredPermissions(state, perm) {
      perm = perm.filter(v => state.userdata.permissions.includes(v))
      state.filteredPermissions = perm
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
    },
    updateAdminAllUsers(state, ul) {
      state.admin.allUsers = ul
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
    },
    getUserList(context) {
      socket.addListenerConstant('admin.getusers.ret', (ul) => context.commit('updateAdminAllUsers', ul))
      socket.sendRequest('admin.getusers', '')
    },
    updateUserPermission(context, {user_name, permissions}) {
      return new Promise((resolve) => {
        socket.sendRequest('admin.user.update.permission', {user_name: user_name, permissions: permissions})
        socket.addListenerOnce('admin.user.update.permission.ret', () => resolve())
      })
      
    }
  }
})
