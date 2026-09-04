<script setup lang="ts">
import { ref, onMounted } from "vue";

interface TaskDetails {
	id: string;
	state: {
		name: string;
		description: string;
		completed: boolean;
	};
}

const props = defineProps<{
	taskId: string | null;
	listId: string;
	create: boolean;
}>();

const emit = defineEmits<{
	close: [];
}>();

const task = ref<TaskDetails | null>(
	props.create
		? {
				id: "",
				state: {
					name: "",
					description: "",
					completed: false,
				},
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
			`${import.meta.env.VITE_API_URL}/tasks/${props.taskId}`,
		);

		if (!response.ok) {
			throw new Error(`HTTP error: ${response.status}`);
		}

		task.value = await response.json();
	} catch (e) {
		error.value = e instanceof Error ? e.message : "Unknown error";
	} finally {
		loading.value = false;
	}
});

async function saveTask() {
	if (!task.value) return;

	if (props.create) {
		await createTask();
	} else {
		await updateTask();
	}
}

async function createTask() {
	if (!task.value) return;

	const response = await fetch(
		`${import.meta.env.VITE_API_URL}/lists/${props.listId}/tasks`,
		{
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				name: task.value.state.name,
				description: task.value.state.description,
				completed: task.value.state.completed,
			}),
		},
	);

	if (!response.ok) {
		throw new Error(`HTTP error: ${response.status}`);
	}

	emit("close");
}

async function updateTask() {
	if (!task.value) return;

	const response = await fetch(
		`${import.meta.env.VITE_API_URL}/tasks/${props.taskId}`,
		{
			method: "PUT",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				name: task.value.state.name,
				description: task.value.state.description,
				completed: task.value.state.completed,
			}),
		},
	);

	if (!response.ok) {
		throw new Error(`HTTP error: ${response.status}`);
	}

	emit("close");
}

async function deleteTask() {
	const response = await fetch(
		`${import.meta.env.VITE_API_URL}/tasks/${props.taskId}`,
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
		<div class="task" @click.stop>
			<button @click="emit('close')">Close</button>

			<p v-if="loading">Loading...</p>

			<p v-else-if="error">
				{{ error }}
			</p>

			<form v-else-if="task" @submit.prevent="saveTask">
				<label>
					Name
					<input v-model="task.state.name" type="text" />
				</label>

				<label>
					Description
					<textarea v-model="task.state.description"></textarea>
				</label>

				<label>
					<input v-model="task.state.completed" type="checkbox" />
					Completed
				</label>

				<button v-if="!create" type="button" @click="deleteTask">Delete</button>

				<button type="submit">Save</button>
			</form>
		</div>
	</div>
</template>
