import { createRouter, createWebHashHistory } from "vue-router";

import Layout1 from "@/c/layout/Layout1.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/ui/welcome",
    },
    {
      path: "/ui",
      component: Layout1,
      children: [
        {
          path: "/ui/welcome",
          component: () => import("@/views/welcome/index.vue"),
          meta: {
            title: "welcome",
          },
        },
        {
          path: "/ui/app",
          component: () => import("@/views/app/index.vue"),
          meta: {
            title: "app",
          },
        },
        {
          path: "/ui/backup",
          component: () => import("@/views/backup/index.vue"),
          meta: {
            title: "backup",
          },
        },
        {
          path: "/ui/host",
          component: () => import("@/views/host/index.vue"),
          meta: {
            title: "host",
          },
        },
        {
          path: "/ui/share",
          component: () => import("@/views/share/index.vue"),
          meta: {
            title: "share",
          },
        },
        {
          path: "/ui/setting",
          component: () => import("@/views/setting/index.vue"),
          meta: {
            title: "setting",
          },
        },
        {
          path: "/ui/about",
          component: () => import("@/views/about/index.vue"),
          meta: {
            title: "about",
          },
        },
      ],
    },
  ],
});

export default router;
