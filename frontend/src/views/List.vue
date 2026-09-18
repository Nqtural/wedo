<script setup lang="ts">
import { computed, ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { apiFetch } from "@/api";
import { type Tag, tagColor } from "@/tag";
import { useTaskEditorStore } from "@/stores/taskEditor";

import Button from "../components/Button.vue";
import Checkbox from "../components/Checkbox.vue";
import EditTask from "../components/EditTask.vue";
import Header from "../components/Header.vue";
import Input from "../components/Input.vue";
import ListPage from "../components/ListPage.vue";
import ListItem from "../components/ListItem.vue";
import ListItemActions from "../components/ListItemActions.vue";
import TagPill from "../components/TagPill.vue";

const route = useRoute();
const router = useRouter();
const listId = route.params.id;
const editor = useTaskEditorStore();

const selectedTaskId = computed(() => {
	return typeof route.query.edit === "string" ? route.query.edit : null;
});

const creatingTask = computed(() => {
	return route.query.create === "true";
});

interface Task {
	id: string;
	name: string;
	description?: string;
	completed: boolean;
	tags: Tag[];
}

interface Filter {
	active: boolean;
	tags: Tag[];
	search: string;
}

const taskList = ref<Task[]>([]);
const listName = ref<string>("");
const expandedTaskId = ref<string | null>(null);
const tags = ref<Tag[]>();
const filter = ref<Filter>({
	active: false,
	tags: [],
	search: "",
});

const filteredTasks = computed(() => {
	let tasks = taskList.value;

	if (!filter.value.active) {
		return tasks;
	}

	if (filter.value.tags.length > 0) {
		tasks = tasks.filter((task) =>
			task.tags.some((tag) =>
				filter.value.tags.some(
					(filteredTag) => filteredTag.id === tag.id,
				),
			),
		);
	}

	if (filter.value.search) {
		const search = filter.value.search.toLowerCase();

		tasks = tasks.filter((task) =>
			task.name.toLowerCase().includes(search),
		);
	}

	return tasks;
});

async function getTasks() {
	taskList.value = await apiFetch<Task[]>(`/lists/${listId}/tasks`);
	if (expandedTaskId.value) {
		const task = taskList.value.find(
			(task) => task.id === expandedTaskId.value,
		);

		if (task) {
			task.description = await getTaskDescription(expandedTaskId.value);
		}
	}
	tags.value = await apiFetch<Tag[]>(`/lists/${listId}/tags`);
}

async function getTaskDescription(taskId: string) {
	return (
		await apiFetch<{
			id: string;
			state: {
				name: string;
				description: string;
				completed: boolean;
			};
		}>(`/lists/${listId}/tasks/${taskId}`)
	).state.description;
}

onMounted(async () => {
	listName.value = (
		await apiFetch<{ id: string; name: string }>(`/lists/${listId}`)
	).name;
	await getTasks();
});

async function toggleCompleted(task: Task) {
	task.completed = (
		await apiFetch<{ completed: boolean }>(
			`/lists/${listId}/tasks/${task.id}/completed`,
			{
				method: "PUT",
				body: JSON.stringify({ completed: task.completed }),
			},
		)
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

async function editTask(task: Task) {
	editor.startEdit({
		id: task.id,
		listId: String(listId),
		name: task.name,
		description: await getTaskDescription(task.id),
		completed: task.completed,
		tags: task.tags,
	});

	router.push({
		name: "List",
		params: {
			id: listId,
		},
		query: {
			edit: task.id,
		},
	});
}

function newTask() {
	editor.startCreate(String(listId));

	router.push({
		name: "List",
		params: {
			id: listId,
		},
		query: {
			create: "true",
		},
	});
}

async function closeTask() {
	router.replace({
		name: "List",
		params: {
			id: listId,
		},
		query: {},
	});

	await getTasks();
}

async function toggleExpandTask(task: Task) {
	if (expandedTaskId.value === task.id) {
		expandedTaskId.value = null;
		return;
	}

	if (!task.description) {
		task.description = await getTaskDescription(task.id);
	}

	expandedTaskId.value = task.id;
}

function filterTagApplied(tagId: string) {
	return filter.value.tags.some((filteredTag) => filteredTag.id === tagId);
}

function filterToggleTag(tag: Tag) {
	if (filterTagApplied(tag.id)) {
		filter.value.tags = filter.value.tags.filter(
			(filteredTag) => filteredTag.id !== tag.id,
		);
		return;
	}

	filter.value.tags.push(tag);
}
</script>

<template>
	<ListPage :title="listName">
		<template #back>
			<Button variant="primary" :to="{ name: 'Lists' }">Back</Button>
		</template>

		<template #filter-button>
			<Button @click="filter.active = !filter.active" variant="primary">
				{{ filter.active ? "Stop filtering" : "Show filter" }}
			</Button>
		</template>

		<template #filter v-if="filter.active">
			<Input v-model="filter.search" placeholder="Search tasks..." />
			<div class="tags-expanded">
				<TagPill
					v-for="tag in tags"
					:key="tag.id"
					:name="tag.state.name"
					:color="tagColor(tag.state.color_key)"
					:applied="filterTagApplied(tag.id)"
					@click="filterToggleTag(tag)"
				/>
			</div>
		</template>

		<ListItem
			@click.stop="toggleExpandTask(task)"
			v-for="task in filteredTasks"
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
						@click.stop="editTask(task)"
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
						:key="tag.id"
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
						<TagPill
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
