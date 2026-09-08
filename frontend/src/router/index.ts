import { createRouter, createWebHistory } from "vue-router";
import { TOKEN_KEY } from "@/auth";

import Login from "../views/Login.vue";
import CreateAccount from "../views/CreateAccount.vue";
import Lists from "../views/Lists.vue";
import List from "../views/List.vue";
import Join from "../views/Join.vue";

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes: [
		{
			path: "/create-account",
			name: "CreateAccount",
			component: CreateAccount,
		},
		{
			path: "/join/:code",
			name: "Join",
			component: Join,
			meta: { requiresAuth: true },
		},
		{
			path: "/login",
			name: "Login",
			component: Login,
		},
		{
			path: "/lists",
			name: "Lists",
			component: Lists,
			meta: { requiresAuth: true },
		},
		{
			path: "/lists/:id",
			name: "List",
			component: List,
			meta: { requiresAuth: true },
		},

		{
			path: "/:pathMatch(.*)*",
			redirect: "/lists",
		},
	],
});

router.beforeEach((to) => {
	const token = localStorage.getItem(TOKEN_KEY);

	if (to.meta.requiresAuth && !token) {
		return {
			path: "/login",
			query: {
				redirect: to.fullPath,
			},
		};
	}
});

export default router;
