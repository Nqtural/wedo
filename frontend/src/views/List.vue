<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { apiFetch } from "@/api";

const router = useRouter();

import Button from "../components/Button.vue";
import Checkbox from "../components/Checkbox.vue";
import EditTask from "../components/EditTask.vue";
import Header from "../components/Header.vue";
import ListPage from "../components/ListPage.vue";
import ListItem from "../components/ListItem.vue";
import ListItemActions from "../components/ListItemActions.vue";
import Tag from "../components/Tag.vue";

const route = useRoute();
const listId = route.params.id;

interface TagState {
	name: string;
	color_key: string;
}

interface Tag {
	id: string;
	state: TagState;
}

interface Task {
	id: string;
	name: string;
	description?: string;
	completed: boolean;
	tags: Tag[];
}

const tagColors: Record<string, string> = {
	red: "--red",
	orange: "--orange",
	yellow: "--yellow",
	green: "--green",
	teal: "--teal",
	blue: "--blue",
	purple: "--purple",
	pink: "--pink",
};

const taskList = ref<Task[]>([]);
const listName = ref<string>("");
const selectedTaskId = ref<string | null>(null);
const creatingTask = ref(false);
const expandedTaskId = ref<string | null>(null);

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

function updateTaskInList(updatedTask: Task) {
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

async function toggleExpandTask(task: Task) {
	if (expandedTaskId.value === task.id) {
		expandedTaskId.value = null;
		return;
	}

	if (!task.description) {
		task.description = (
			await apiFetch<{
				id: string;
				state: {
					name: string;
					description: string;
					completed: boolean;
				};
			}>(`/tasks/${task.id}`)
		).state.description;
	}

	expandedTaskId.value = task.id;
}

function tagColor(color_key: string) {
	return {
		"--tag-color": `var(${tagColors[color_key] ?? "--color-primary"})`,
	};
}
</script>

<template>
	<ListPage :title="listName">
		<template #back>
			<Button variant="primary" :to="{ name: 'Lists' }">Back</Button>
		</template>

		<ListItem
			@click.stop="toggleExpandTask(task)"
			v-for="task in taskList"
			:key="task.id"
			:expanded="task.id === expandedTaskId"
		>
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
			<Transition name="fade">
				<div v-if="task.id !== expandedTaskId" class="tags">
					<span
						v-for="tag in task.tags"
						:style="tagColor(tag.state.color_key)"
					></span>
				</div>
			</Transition>

			<template #expanded>
				<div class="expanded-wrapper">
					<p v-if="task.description" class="task__description">
						{{ task.description }}
					</p>
					<p
						v-else
						class="task__description task__description--empty"
					>
						No description
					</p>
					<div class="tags-expanded">
						<Tag
							v-for="tag in task.tags"
							:name="tag.state.name"
							:color="tagColor(tag.state.color_key)"
							:applied="true"
						/>
					</div>
				</div>
			</template>
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
.fade-enter-active,
.fade-leave-active {
	transition: opacity var(--transition-fast);
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
}

.fade-enter-to,
.fade-leave-from {
	opacity: 1;
}

.tags {
	position: absolute;
	bottom: 10px;
	left: 10px;
	font-size: 12px;
	display: flex;
	gap: 3px;

	& span {
		display: inline-block;
		height: 3px;
		width: 10px;
		border-radius: 9999px;
		background-color: var(--tag-color);
	}
}

.tags-expanded {
	display: flex;
	flex-wrap: wrap;
	gap: 5px;
	margin-top: auto;
}

.expanded-wrapper {
	display: flex;
	flex-direction: column;
}

.task {
	display: flex;
	align-items: center;
	flex: 1;
	text-decoration: none;
	color: var(--color-text);
	border-radius: var(--radius-sm);
	transition: background var(--transition-fast);
}

.task__description {
	font-size: 0.9em;
	color: var(--color-text-muted);
	margin: 0;
}

.task__description--empty {
	color: var(--color-text-subtle);
}

li {
	&:has(.completed) {
		border-color: var(--color-success);
	}

	.completed {
		text-decoration: line-through;
	}
}

li:hover .btn-container > * {
	opacity: 1;
}
</style>
