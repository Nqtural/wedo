<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { apiFetch } from "@/api";
import { type Tag, tagColor, tagColors } from "@/tag";
import { type TaskForm, useTaskEditorStore } from "@/stores/taskEditor";

import Button from "../components/Button.vue";
import Checkbox from "../components/Checkbox.vue";
import EditModal from "../components/EditModal.vue";
import TagPill from "../components/TagPill.vue";

const router = useRouter();
const editor = useTaskEditorStore();

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

const loading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
	await updateTags();
	loading.value = false;
});

async function updateTags() {
	availableTags.value = (
		await apiFetch<Tag[]>(`/lists/${props.listId}/tags`)
	).sort((a, b) => a.state.name.localeCompare(b.state.name));
}

function close() {
	editor.clear();
	emit("close");
}

async function saveTask() {
	if (!editor.form) return;

	if (props.create) {
		await createTask(editor.form);
		return;
	}

	await updateTask(editor.form);
}

async function createTask(taskState: TaskForm) {
	await apiFetch(`/lists/${props.listId}/tasks`, {
		method: "POST",
		body: JSON.stringify(taskState),
	});

	close();
}

async function updateTask(taskState: TaskForm) {
	await apiFetch(`/lists/${props.listId}/tasks/${props.taskId}`, {
		method: "PUT",
		body: JSON.stringify(taskState),
	});

	close();
}

async function deleteTask() {
	await apiFetch(`/lists/${props.listId}/tasks/${props.taskId}`, {
		method: "DELETE",
	});

	close();
}

function toggleTag(tag: Tag) {
	const tags = editor.form.tags;

	if (tags.some((t) => t.id === tag.id)) {
		editor.form.tags = tags.filter((t) => t.id !== tag.id);
	} else {
		editor.form.tags = [...tags, tag];
	}
}

function tagApplied(tagId: string) {
	return editor.form.tags.some((tag) => tag.id === tagId);
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
		@close="close()"
		@save="saveTask"
		@delete="deleteTask"
	>
		<label>Name</label>
		<input v-model="editor.form.name" type="text" />

		<label>Description</label>
		<textarea v-model="editor.form.description"></textarea>

		<label>
			<Checkbox
				class="checkbox"
				v-model="editor.form.completed"
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
