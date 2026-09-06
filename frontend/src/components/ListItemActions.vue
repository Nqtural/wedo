<script setup lang="ts">
import { useSlots } from "vue";

const slots = useSlots();
</script>

<template>
	<div class="btn-container">
		<div v-if="slots.hiding" class="hiding">
			<slot name="hiding" />
		</div>

		<div v-if="slots.visible" class="visible">
			<slot name="visible" />
		</div>
	</div>
</template>

<style scoped>
.btn-container {
	position: absolute;
	right: 0;
	top: 0;
	height: 100%;
	width: fit-content;
	display: flex;
	pointer-events: none;
}

.btn-container > * {
	height: 100%;
	aspect-ratio: 1;
	display: grid;
	place-items: center;
	opacity: 0;
	transition: opacity var(--transition-fast);
}

.btn-container > .visible {
	opacity: 1;
}

.btn-container > * {
	pointer-events: auto;
}
</style>
