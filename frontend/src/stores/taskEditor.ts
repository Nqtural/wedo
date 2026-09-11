import { defineStore } from "pinia";
import { ref } from "vue";
import { type Tag } from "@/tag";

export interface TaskForm {
	name: string;
	description: string;
	completed: boolean;
	tags: Tag[];
}

const emptyForm = (): TaskForm => ({
	name: "",
	description: "",
	completed: false,
	tags: [],
});

export const useTaskEditorStore = defineStore("taskEditor", () => {
	const taskId = ref<string | null>(null);
	const listId = ref<string | null>(null);
	const form = ref<TaskForm>(emptyForm());

	function startCreate(newListId: string) {
		taskId.value = null;
		listId.value = newListId;
		form.value = emptyForm();
	}

	function startEdit(task: {
		id: string;
		listId: string;
		name: string;
		description?: string;
		completed: boolean;
		tags: Tag[];
	}) {
		taskId.value = task.id;
		listId.value = task.listId;

		form.value = {
			name: task.name,
			description: task.description ?? "",
			completed: task.completed,
			tags: task.tags,
		};
	}

	function clear() {
		taskId.value = null;
		listId.value = null;
		form.value = emptyForm();
	}

	return {
		taskId,
		listId,
		form,
		startCreate,
		startEdit,
		clear,
	};
});
