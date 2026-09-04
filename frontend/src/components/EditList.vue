<script setup lang="ts">
import { ref, onMounted } from "vue";

interface List {
	id: string;
	name: string;
}

const props = defineProps<{
	listId: string | null;
	create: boolean;
}>();

const emit = defineEmits<{
	close: [];
}>();

const list = ref<List | null>(
	props.create
		? {
			id: "",
			name: "",
		}
		: null,
);
const loading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
	if (props.create) {
		loading.value = false;
		return;
	}

	try {
		const response = await fetch(
			`${import.meta.env.VITE_API_URL}/lists/${props.listId}`,
		);

		if (!response.ok) {
			throw new Error(`HTTP error: ${response.status}`);
		}

		list.value = await response.json();
	} catch (e) {
		error.value = e instanceof Error ? e.message : "Unknown error";
	} finally {
		loading.value = false;
	}
});

async function saveList() {
	if (!list.value) return;

	if (props.create) {
		await createList();
	} else {
		await updateList();
	}
}

async function createList() {
	if (!list.value) return;

	const response = await fetch(
		`${import.meta.env.VITE_API_URL}/lists`,
		{
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				name: list.value.name,
			}),
		},
	);

	if (!response.ok) {
		throw new Error(`HTTP error: ${response.status}`);
	}

	emit("close");
}

async function updateList() {
	if (!list.value) return;

	const response = await fetch(
		`${import.meta.env.VITE_API_URL}/lists/${props.listId}`,
		{
			method: "PUT",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				name: list.value.name,
			}),
		},
	);

	if (!response.ok) {
		throw new Error(`HTTP error: ${response.status}`);
	}

	emit("close");
}

async function deleteList() {
	const response = await fetch(
		`${import.meta.env.VITE_API_URL}/lists/${props.listId}`,
		{
			method: "DELETE",
		},
	);

	if (response.status !== 204) {
		throw new Error(`HTTP error: ${response.status}`);
	}

	emit("close");
}
</script>

<template>
	<div class="overlay" @click="emit('close')">
		<div class="list" @click.stop>
			<button @click="emit('close')">Close</button>

			<p v-if="loading">Loading...</p>

			<p v-else-if="error">
				{{ error }}
			</p>

			<form v-else-if="list" @submit.prevent="saveList">
				<label>
					Name
					<input v-model="list.name" type="text" />
				</label>

				<button v-if="!create" type="button" @click="deleteList">Delete</button>

				<button type="submit">Save</button>
			</form>
		</div>
	</div>
</template>
