<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { apiFetch } from "@/api";
import { type Tag, tagColor, tagColors } from "@/tag";

import Button from "../components/Button.vue";
import Checkbox from "../components/Checkbox.vue";
import EditModal from "../components/EditModal.vue";
import TagPill from "../components/TagPill.vue";

const router = useRouter();

const selectedTaskId = ref<string | null>(null);

interface TaskDetails {
	id: string;
	state: {
		name: string;
		description: string;
		completed: boolean;
		tags: Tag[];
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

const availableTags = ref<Tag[]>([]);
const addingTag = ref(false);
const tagInput = ref("");

const filteredTags = computed(() => {
	const query = tagInput.value.trim().toLowerCase();
	const appliedTagIds = new Set(
		task.value?.state.tags.map((tag) => tag.id) ?? [],
	);

	return (availableTags.value ?? [])
		.filter((tag) => !appliedTagIds.has(tag.id))
		.filter((tag) => tag.state.name.toLowerCase().includes(query))
		.slice(0, 5);
});

const task = ref<TaskDetails>();
const loading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
	if (props.create) {
		task.value = {
			id: "",
			state: {
				name: "",
				description: "",
				completed: false,
				tags: [],
			},
		};

		try {
			await updateTags();
		} catch (e) {
			error.value = e instanceof Error ? e.message : "Unknown error";
		} finally {
			loading.value = false;
		}

		return;
	}

	try {
		task.value = await apiFetch<TaskDetails>(`/tasks/${props.taskId}`);
		await updateTags();
	} catch (e) {
		error.value = e instanceof Error ? e.message : "Unknown error";
	} finally {
		loading.value = false;
	}
});

async function updateTags() {
	availableTags.value = (
		await apiFetch<Tag[]>(`/lists/${props.listId}/tags`)
	).sort((a, b) => a.state.name.localeCompare(b.state.name));
}

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
		body: JSON.stringify(task.value.state),
	});

	emit("close");
}

async function updateTask() {
	if (!task.value) return;

	await apiFetch(`/tasks/${props.taskId}`, {
		method: "PUT",
		body: JSON.stringify(task.value.state),
	});

	emit("close");
}

async function deleteTask() {
	await apiFetch(`/tasks/${props.taskId}`, {
		method: "DELETE",
	});

	emit("close");
}

function removeTag(tag_id: string) {
	if (!task.value) {
		return;
	}

	task.value.state.tags = task.value?.state.tags.filter(
		(tag) => tag.id !== tag_id,
	);
}

function toggleTag(tag: Tag) {
	if (tagApplied(tag.id)) {
		removeTag(tag.id);
		return;
	}

	task.value?.state.tags.push(tag);
}

function tagApplied(tag_id: string) {
	if (task.value) {
		return task.value.state.tags.some((tag) => tag.id === tag_id);
	}

	return false;
}

function manageTags() {
	router.push({
		name: "Tags",
		params: {
			id: props.listId,
		},
		query: props.taskId ? { edit: props.taskId } : { create: "true" },
	});
}
</script>

<template>
	<EditModal
		:create="create"
		:loading="loading"
		:error="error"
		title="Create task"
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

		<h3>Tags</h3>
		<div class="tagging">
			<div class="tags">
				<TagPill
					v-for="tag in availableTags"
					:key="tag.id"
					:color="tagColor(tag.state.color_key)"
					:name="tag.state.name"
					:applied="tagApplied(tag.id)"
					@click="toggleTag(tag)"
				/>
			</div>
			<RouterLink :to="`/lists/${listId}/tags`">Manage tags</RouterLink>
		</div>
	</EditModal>
</template>

<style scoped>
h3 {
	margin: 0;
}

.tagging {
	display: flex;
	flex-direction: column;
	gap: 10px;

	.tags {
		display: flex;
		flex-wrap: wrap;
		gap: 5px;

		& > * {
			cursor: pointer;
		}
	}
}

a {
	color: var(--color-primary);
	text-decoration: none;
	width: fit-content;

	&:hover {
		text-decoration: underline;
	}
}
</style>
