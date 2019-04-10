import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import Socket from './socket'
import Buefy from 'buefy'
import VueMq from 'vue-mq'

import 'buefy/dist/buefy.css'

Vue.config.productionTip = false

Vue.use(Buefy)

Vue.use(VueMq, {
  breakpoints: {
    sm: 450,
    md: 1087,
    lg: Infinity,
  }
})

Socket.init()

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
