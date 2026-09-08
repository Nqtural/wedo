<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { apiFetch } from "@/api";

import Button from "../components/Button.vue";

interface List {
	id: string,
	name: string,
}

interface JoinResult {
	list: List,
	joined: boolean,
}

const router = useRouter();
const route = useRoute();

const inviteCode = route.params.code;
const loading = ref(true);
const joinResult = ref<JoinResult | null>(null);
const joinError = ref<string | null>(null);

const errorTitle = ref<string | null>(null);
const errorTitles = [
	"Uh-oh...",
	"Darn...",
	":(",
	"Oops...",
	"Yikes...",
	"Oh no...",
	"Whoops...",
	"Aw, nuts...",
	"Oopsie...",
	"Uh oh...",
	"Not again...",
	"That's awkward...",
];

onMounted(async () => {
	try {
		joinResult.value = await apiFetch<JoinResult>(
			`/lists/join/${inviteCode}`,
			{
				method: "POST",
			},
		);

		if (!joinResult.value.joined) {
			await router.push(`/lists/${joinResult.value.list.id}`);
		}
	} catch (error) {
		errorTitle.value = errorTitles[Math.floor(Math.random() * errorTitles.length)]!;
		if (error instanceof Error && error.message === "HTTP error: 404") {
			joinError.value = "Invalid or expired invitation link.";
		} else if (error instanceof Error && error.message === "HTTP error: 500") {
			joinError.value = "Internal server error.";
		} else {
			joinError.value = "An unexpected error occurred.";
		}
	}
	loading.value = false;
});
</script>

<template>
	<div class="wrapper">
		<p v-if="loading">Loading...</p>
		<template v-else>
			<template v-if="joinError === null && joinResult !== null">
				<h1>Success</h1>
				<hr />
				<p>You have joined {{ joinResult.list.name }}.</p>
				<div class="btn-wrapper">
					<Button variant="primary" to="/lists">Go to your lists</Button>
					<Button variant="primary" :to="`/lists/${joinResult.list.id}`">Go to {{ joinResult.list.name }}</Button>
				</div>
			</template>
			<template v-else>
				<h1>{{ errorTitle }}</h1>
				<hr />
				<p>{{ joinError }}</p>
				<div class="btn-wrapper">
					<Button variant="primary" to="/lists">Go to your lists</Button>
				</div>
			</template>
		</template>
	</div>
</template>

<style scoped>
.wrapper {
	display: flex;
	flex-direction: column;
	gap: 20px;
	color: var(--color-text);
	background: var(--surface-0);
	border: var(--border-width) solid var(--surface-1);
	border-radius: var(--radius-sm);
	padding: 20px;
	width: 30em;
}

h1 {
	margin: 0;
	font-size: 4em;
}

hr {
	height: 1.5px;
	background: var(--color-text);
	width: 100%;
	border: none;
	margin: 0px;
}

p {
	margin: 0;
	font-size: 1.5em;
	overflow-y: auto;
}

.btn-wrapper {
	display: flex;
	justify-content: space-between;
}

.button {
	padding-inline: 5px;
	align-self: end;
	margin-top: 50px;
}
</style>
