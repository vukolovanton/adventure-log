import { createRouter, createWebHashHistory } from "vue-router";

import Settings from "../components/Settings.vue";
import Editor from "../components/Editor.vue";
import Canvas from "../components/Canvas.vue";
import Tagmap from "../components/Tagmap.vue";

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
  {
    path: "/canvas",
    name: "canvas",
    component: Canvas,
  },
  {
    path: "/tagmap",
    name: "tagmap",
    component: Tagmap,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
