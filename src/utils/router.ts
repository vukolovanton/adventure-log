import { createRouter, createWebHashHistory } from 'vue-router';

import Greet from '../components/Greet.vue';
import Editor from '../components/Editor.vue';

const routes = [
  {
    path: '/',
    name: 'home',
    component: Greet,
  },
  {
    path: '/editor/:id?',
    name: 'editor',
    sensitive: true,
    component: Editor,
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
