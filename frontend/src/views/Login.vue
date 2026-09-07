<script setup lang="ts">
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { apiFetch } from "@/api";
import { TOKEN_KEY } from "@/auth";

const route = useRoute();
const router = useRouter();

import Button from "../components/Button.vue";
import Input from "../components/Input.vue";

const username = ref("");
const password = ref("");
const message = ref("");

type InputVariant = "normal" | "danger";
const variant_username = ref<InputVariant>("normal");
const variant_password = ref<InputVariant>("normal");

async function submit() {
	if (username.value.length < 3) {
		message.value = "Username must be at least 3 characters long";
		variant_username.value = "danger";
		return;
	}

	try {
		let token = await apiFetch<string>("/auth/login", {
			method: "POST",
			body: JSON.stringify({
				username: username.value,
				password: password.value,
			}),
		});

		localStorage.setItem(TOKEN_KEY, token);

		const redirect = route.query.redirect;

		if (typeof redirect === "string") {
			await router.push(redirect);
		} else {
			await router.push("/lists");
		}
	} catch (error) {
		if (error instanceof Error && error.message === "Unauthorized") {
			message.value = "Incorrect username or password.";
			variant_username.value = "danger";
			variant_password.value = "danger";
			return;
		}

		throw error;
	}
}
</script>

<template>
	<div class="wrapper">
		<h2>Log in</h2>
		<form @submit.prevent="submit()">
			<Input
				type="text"
				@input="variant_username = 'normal'"
				:variant="variant_username"
				v-model="username"
				placeholder="Username"
			></Input>
			<Input
				type="password"
				@input="variant_password = 'normal'"
				:variant="variant_password"
				v-model="password"
				placeholder="Password"
			></Input>
			<p class="message">{{ message }}</p>
			<Button type="submit" variant="success">Log in</Button>
			<p>
				Don't have an account?
				<RouterLink to="/create-account">Create</RouterLink> one!
			</p>
		</form>
	</div>
</template>

<style scoped>
.wrapper {
	color: var(--color-text);
	border: var(--border-width) solid var(--color-surface-2);
	background: var(--color-surface-0);
	padding: 20px;
	border-radius: var(--radius-sm);
	display: flex;
	flex-direction: column;
	gap: 20px;
	width: 16em;
}

h2 {
	margin: 0;
}

form {
	display: flex;
	flex-direction: column;
	gap: 10px;
	align-items: center;
}

p {
	max-width: 100%;
	font-size: 0.8em;
	margin: 0;
}

.message {
	color: var(--color-danger);
	margin: 0.5em 0;
	font-size: 0.8em;
	line-height: 0.8em;
	height: 0.8em;
}

button {
	width: 100%;
}

a {
	color: var(--color-primary);
	text-decoration: none;

	&:hover {
		text-decoration: underline;
	}
}
</style>
