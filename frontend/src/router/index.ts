import { createRouter, createWebHistory } from "vue-router";
import Lists from "../views/Lists.vue";
import List from "../views/List.vue";

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes: [
		{
			path: "/lists",
			name: "Lists",
			component: Lists,
		},
		{
			path: "/lists/:id",
			name: "List",
			component: List,
		},

		{
			path: '/:pathMatch(.*)*',
			redirect: '/links',
		},
	],
});

export default router;
