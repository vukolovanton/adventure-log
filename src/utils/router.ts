import { createRouter, createWebHashHistory } from "vue-router";

import Settings from "../components/Settings.vue";
import Editor from "../components/Editor.vue";

const routes = [
  {
    path: "/",
    redirect: "/editor/",
  },
  {
    path: "/settings",
    name: "settings",
    component: Settings,
  },
  {
    path: "/editor/:id?",
    name: "editor",
    sensitive: true,
    component: Editor,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
