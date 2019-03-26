import Vue from 'vue'
import Router from 'vue-router'
import Home from './views/Home.vue'
import Store from './store'
import Cookie from 'js-cookie'

Vue.use(Router)

const router = new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'Home',
      component: Home
    },
    {
      path: '/login',
      name: 'Login',
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import(/* webpackChunkName: "login" */ './views/Login.vue')
    }
  ]
})

router.beforeEach((to, from, next) => {

  //check if user was logged in before
  let sid = Cookie.get('sid')
  let uname = Cookie.get("user")
  if (sid != undefined && sid != 'undefined' && uname != undefined && uname != 'undefined' && sid.length > 5 && uname.length > 1) {
    Store.commit('setLoggedIn', {username: uname, sessionID: sid})
  }

  // redirect to login page if not logged in and trying to access a restricted page
  const publicPages = ['/login'];
  const authRequired = !publicPages.includes(to.path);
  const loggedIn = Store.getters.isLoggedIn;

  if (authRequired && !loggedIn) {
    return next('/login');
  }

  next();
})

export default router

