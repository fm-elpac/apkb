import { createRouter, createWebHashHistory } from "vue-router";

import Layout1 from "@/c/layout/Layout1.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/ui/home",
    },
    {
      path: "/ui",
      component: Layout1,
      children: [
        {
          path: "/ui/home",
          component: () => import("@/views/home/index.vue"),
          meta: {
            title: "App",
          },
        },
      ],
    },
  ],
});

export default router;
