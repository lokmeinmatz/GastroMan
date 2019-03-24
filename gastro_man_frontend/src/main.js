import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import Socket from './socket'

Vue.config.productionTip = false


Socket.init()

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
