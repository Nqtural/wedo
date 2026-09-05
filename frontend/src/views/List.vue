<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
const router = useRouter();

import Task from "../components/Task.vue";
import Header from "../components/Header.vue";

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

async function toggleCompleted(task: Task) {
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
	<Header />

	<div class="task-list-container">
		<div class="header">
			<router-link :to="{ name: 'Lists' }">Back</router-link>
			<h2>{{listName}}</h2>
		</div>
		<hr>
		<ul>
			<TransitionGroup name="list-transition">
				<li v-for="task in taskList" :key="task.id">
					<div class="task" :class="{ completed: task.completed }" @click="selectedTaskId = task.id">
						{{ task.name }}

						<div class="checkbox-wrapper" @click.stop @click="toggleCompleted(task)">
							<div class="checkbox" :class="{ completed: task.completed }">
								<span>✓</span>
							</div>
						</div>
					</div>
				</li>
			</TransitionGroup>
			<button @click="newTask">Add task</button>
		</ul>
	</div>


	<Transition name="overlay">
		<Task
			v-if="selectedTaskId || creatingTask"
			:task-id="selectedTaskId"
			:list-id="String(listId)"
			:create="creatingTask"
			@close="closeTask"
		/>
	</Transition>
</template>

<style scoped>
.list-transition-enter-active,
.list-transition-leave-active {
	transition: opacity 0.3s, transform 0.3s;
}

.list-transition-enter-from,
.list-transition-leave-to {
	opacity: 0;
}

.list-transition-enter-to,
.list-transition-leave-from {
	opacity: 1;
}

.task-list-container {
	display: flex;
	flex-direction: column;
	width: min(50em, calc(100vw - 5em));
	background: dimgray;
	border-radius: 6px;
	color: black;
	padding: 20px;
	max-height: 450px;

	/* push content further up */
	margin-bottom: 10em;
}

.header {
	display: flex;
	gap: 20px;
}

a {
	display: flex;
	justify-content: center;
	align-items: center;
	color: black;
	text-decoration: none;
	border: 2px solid black;
	width: 30px;
	height: 30px;
	border-radius: 50%;
	background: lightgray;
}

h2 {
	display: inline-block;
	float: left;
	margin: 0;
}

hr {
	height: 1.5px;
	background: black;
	width: 100%;
	border: none;
	margin: 20px 0px;
}

ul {
	margin: 0;
	padding: 0;
	list-style: none;
	display: flex;
	gap: 10px;
	flex-direction: column;
	flex: 1;
	flex-shrink: 0;
	justify-content: center;
	align-items: center;
	height: fit-content;
	overflow: hidden;
}

li {
	background: darkgray;
	border-radius: 6px;
	--li-height: 50px;
	height: var(--li-height);
	display: flex;
	position: relative;
	width: 100%;
	padding: 10px;
	box-sizing: border-box;

	& .task:hover > input[type="checkbox"],
	input[type="checkbox"]:hover {
		opacity: 1;
		pointer-events: all;
	}
}

.task {
	display: flex;
	align-items: center;
	height: 100%;
	flex: 1;
	text-decoration: none;
	color: black;
	border-radius: 6px;
	transition: background 0.1s;

	& .checkbox-wrapper {
		position: absolute;
		height: 100%;
		aspect-ratio: 1/1;
		right: 0;
		display: flex;
		align-items: center;
		justify-content: center;

		&:hover {
			cursor: pointer;
		}

		.checkbox {
			height: 75%;
			aspect-ratio: 1/1;
			box-sizing: border-box;
			border: var(--border-width) solid var(--color-surface-2);
			border-radius: var(--radius-sm);
			background: var(--color-surface-1);
			transition: background-color var(--transition-fast), border-color var(--transition-fast);
			display: flex;
			align-items: center;
			justify-content: center;

			& > span {
				height: fit-content;
				width: fit-content;
				user-select: none;
				-webkit-user-select: none;
				-moz-user-select: none;
				-ms-user-select: none;
				opacity: 0;
				transition: opacity var(--transition-fast);
			}

			&.completed {
				border-color: var(--color-success);
				background-color: color-mix(in srgb, var(--color-success) 70%, black);

				& > span {
					opacity: 1;
				}
			}
		}
	}
}

.completed {
	background: lightgreen;
	text-decoration: line-through;
}


#new-list-btn {
	height: 25px;
	width: 25%;
}
</style>
