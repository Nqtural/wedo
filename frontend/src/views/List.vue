<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { apiFetch } from "@/api";

const router = useRouter();

import Checkbox from "../components/Checkbox.vue";
import EditTask from "../components/EditTask.vue";
import Header from "../components/Header.vue";
import Button from "../components/Button.vue";
import ListPage from "../components/ListPage.vue";
import ListItem from "../components/ListItem.vue";
import ListItemActions from "../components/ListItemActions.vue";

const route = useRoute();
const listId = route.params.id;

interface Task {
	id: string;
	name: string;
	completed: boolean;
}

const taskList = ref<Task[]>([]);
const listName = ref<string>("");
const selectedTaskId = ref<string | null>(null);
const creatingTask = ref(false);

async function getTasks() {
	taskList.value = await apiFetch<Task[]>(`/lists/${listId}/tasks`);
}

onMounted(async () => {
	listName.value = (
		await apiFetch<{ id: string; name: string }>(`/lists/${listId}`)
	).name;
	await getTasks();
});

async function toggleCompleted(task: Task) {
	task.completed = (
		await apiFetch<{ completed: boolean }>(`/tasks/${task.id}/completed`, {
			method: "PUT",
			body: JSON.stringify({ completed: task.completed }),
		})
	).completed;
}

function updateTaskInList(updatedTask: {
	id: string;
	name: string;
	completed: boolean;
}) {
	const existingTask = taskList.value.find(
		(task) => task.id === updatedTask.id,
	);

	if (existingTask) {
		existingTask.name = updatedTask.name;
		existingTask.completed = updatedTask.completed;
	}
}

function newTask() {
	creatingTask.value = true;
}

async function closeTask() {
	selectedTaskId.value = null;
	creatingTask.value = false;

	await getTasks();
}
</script>

<template>
	<ListPage :title="listName">
		<template #back>
			<Button variant="primary" :to="{ name: 'Lists' }">Back</Button>
		</template>

		<ListItem v-for="task in taskList" :key="task.id">
			<div class="task" :class="{ completed: task.completed }">
				{{ task.name }}
			</div>

			<ListItemActions>
				<template #hiding>
					<Button
						type="button"
						variant="primary"
						@click.stop="selectedTaskId = task.id"
					>
						Edit
					</Button>
				</template>

				<template #visible>
					<Checkbox
						v-model="task.completed"
						@update:modelValue="toggleCompleted(task)"
						@click.stop
					/>
				</template>
			</ListItemActions>
		</ListItem>

		<template #actions>
			<Button variant="success" @click="newTask">Add</Button>
		</template>
	</ListPage>

	<Transition name="overlay">
		<EditTask
			v-if="selectedTaskId || creatingTask"
			:task-id="selectedTaskId"
			:list-id="String(listId)"
			:create="creatingTask"
			@close="closeTask"
		/>
	</Transition>
</template>

<style scoped>
.task {
	display: flex;
	align-items: center;
	flex: 1;
	text-decoration: none;
	color: var(--color-text);
	padding: 10px;
	border-radius: var(--radius-sm);
	transition: background var(--transition-fast);
}

li:has(.completed) {
	border-color: var(--color-success);
	text-decoration: line-through;
}

li:hover .btn-container > * {
	opacity: 1;
}
</style>
