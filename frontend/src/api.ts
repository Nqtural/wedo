import router from "@/router";
import { TOKEN_KEY } from "@/auth";

export async function apiFetch<T>(
	path: string,
	init?: RequestInit,
): Promise<T> {
	const headers = new Headers(init?.headers);

	if (init?.body && !headers.has("Content-Type")) {
		headers.set("Content-Type", "application/json");
	}

	const token = localStorage.getItem(TOKEN_KEY);

	if (token) {
		headers.set("Authorization", token);
	}

	const response = await fetch(`${import.meta.env.VITE_API_URL}${path}`, {
		...init,
		credentials: "include",
		headers,
	});

	if (response.status === 401) {
		localStorage.removeItem(TOKEN_KEY);

		if (router.currentRoute.value.path !== "/login") {
			await router.push({
				path: "/login",
				query: {
					redirect: router.currentRoute.value.fullPath,
				},
			});
		}

		throw new Error("Unauthorized");
	}

	if (!response.ok) {
		throw new Error(`HTTP error: ${response.status}`);
	}

	if (response.status === 204) {
		return undefined as T;
	}

	const text = await response.text();

	if (!text) {
		return undefined as T;
	}

	return JSON.parse(text) as T;
}
