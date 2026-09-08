<script setup lang="ts">
import Button from "./Button.vue";

defineProps<{
	title: string;
	create: boolean;
	loading: boolean;
	error: string | null;
}>();

const emit = defineEmits<{
	close: [];
	save: [];
	delete: [];
}>();
</script>

<template>
	<div class="overlay" @click="emit('close')">
		<div class="edit-modal" @click.stop>
			<div class="header">
				<h2>{{ title }}</h2>
				<Button type="button" variant="primary" @click="emit('close')">
					Close
				</Button>
			</div>

			<p v-if="loading">Loading...</p>

			<p v-else-if="error">
				{{ error }}
			</p>

			<form v-else @submit.prevent="emit('save')">
				<slot />

				<div class="btn-container">
					<div class="btn-container">
						<slot name="extra-buttons" />
					</div>
					<Button
						v-if="!create"
						type="button"
						variant="danger"
						@click="emit('delete')"
					>
						Delete
					</Button>

					<Button type="submit" variant="success">Save</Button>
				</div>
			</form>
		</div>
	</div>
</template>

<style scoped>
.overlay {
	position: absolute;
	width: 100vw;
	height: 100vh;
	background: #000000bb;
	top: 0px;
	left: 0px;
	display: flex;
	align-items: center;
	justify-content: center;
	backdrop-filter: blur(5px);
	-webkit-backdrop-filter: blur(5px);
}

.edit-modal {
	color: var(--color-text);
	width: 30em;
	background: var(--color-surface-0);
	border: var(--border-width) solid var(--color-surface-1);
	border-radius: var(--radius-sm);
	position: relative;
	padding: 20px;
}

.header {
	display: flex;
	justify-content: space-between;
}

h2 {
	margin: 0;
}

.edit-modal > button {
	position: absolute;
	right: 5px;
	top: 5px;
}

form, .edit-modal {
	display: flex;
	flex-direction: column;
	gap: 10px;
}

.btn-container {
	margin-top: auto;
	display: flex;
	flex: 1;
	gap: 10px;
}

:slotted(label) {
	display: flex;
	align-items: center;
	gap: 0.5rem;
}

:slotted(input[type="text"]) {
	height: 36px;
}

:slotted(textarea) {
	height: 68px;
}

:slotted(input[type="text"]),
:slotted(textarea) {
	color: inherit;
	font-size: 1em;
	font-family: inherit;
	width: 100%;
	display: block;
	box-sizing: border-box;
	border: var(--border-width) solid var(--color-surface-2);
	border-radius: var(--radius-sm);
	background: var(--color-surface-1);
	outline: none;
	padding: 5px;
	filter: var(--shadow-sm);
	transition:
		border-color var(--transition-fast),
		filter var(--transition-slow);
}
</style>
