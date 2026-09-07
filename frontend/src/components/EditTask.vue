<script setup lang="ts">
import { ref, onMounted } from "vue";
import { apiFetch } from "@/api";

import Checkbox from "../components/Checkbox.vue";
import EditModal from "../components/EditModal.vue";

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
		task.value = await apiFetch<TaskDetails>(`/tasks/${props.taskId}`);
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

	await apiFetch(`/lists/${props.listId}/tasks`, {
		method: "POST",
		body: JSON.stringify({
			name: task.value.state.name,
			description: task.value.state.description,
			completed: task.value.state.completed,
		}),
	});

	emit("close");
}

async function updateTask() {
	if (!task.value) return;

	await apiFetch(`/tasks/${props.taskId}`, {
		method: "PUT",
		body: JSON.stringify({
			name: task.value.state.name,
			description: task.value.state.description,
			completed: task.value.state.completed,
		}),
	});

	emit("close");
}

async function deleteTask() {
	await apiFetch(`/tasks/${props.taskId}`, {
		method: "DELETE",
	});

	emit("close");
}
</script>

<template>
	<EditModal
		:create="create"
		:loading="loading"
		:error="error"
		@close="emit('close')"
		@save="saveTask"
		@delete="deleteTask"
	>
		<label>Name</label>
		<input v-if="task" v-model="task.state.name" type="text" />

		<label>Description</label>
		<textarea v-if="task" v-model="task.state.description"></textarea>

		<label>
			<Checkbox
				v-if="task"
				class="checkbox"
				v-model="task.state.completed"
			/>
			Completed
		</label>
	</EditModal>
</template>

<style scoped>
.checkbox {
	height: 35px;
	aspect-ratio: 1/1;
	display: inline-flex;
}
</style>
