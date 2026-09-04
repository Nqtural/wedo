<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
const router = useRouter();

import Task from "../components/Task.vue";

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
	const response = await fetch(`${import.meta.env.VITE_API_URL}/lists/${listId}/tasks`);

	if (!response.ok) {
		throw new Error(`HTTP error: ${response.status}`);
	}

	taskList.value = await response.json();
}

onMounted(async () => {
	const response = await fetch(`${import.meta.env.VITE_API_URL}/lists/${listId}`);

	if (!response.ok) {
		throw new Error(`HTTP error: ${response.status}`);
	}

	listName.value = (await response.json()).name;

 	getTasks();
});

async function toggleFinished(task: Task) {
	const response = await fetch(
		`${import.meta.env.VITE_API_URL}/tasks/${task.id}/completed`,
		{
			method: "PUT",
			body: JSON.stringify({ completed: !task.completed }),
			headers: {
				"Content-Type": "application/json",
			},
		},
	);

	task.completed = (await response.json()).completed;
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

function closeTask() {
	selectedTaskId.value = null;
	creatingTask.value = false;

	getTasks();
}
</script>

<template>
	<h1>{{listName}}</h1>

	<router-link :to="{ name: 'Lists' }">Back</router-link>

	<ul>
		<li v-for="task in taskList" :key="task.id">
			<div @click="selectedTaskId = task.id">
				{{ task.name }}

				<input
					type="checkbox"
					:checked="task.completed"
					@click.stop
					@change="toggleFinished(task)"
				/>
			</div>
		</li>

		<li>
			<button @click="newTask">Add task</button>
		</li>
	</ul>

	<Task
		v-if="selectedTaskId || creatingTask"
		:task-id="selectedTaskId"
		:list-id="String(listId)"
		:create="creatingTask"
		@close="closeTask"
	/>
</template>
