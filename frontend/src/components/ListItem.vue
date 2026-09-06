<template>
	<li>
		<slot />
	</li>
</template>

<style scoped>
li {
	background: var(--color-surface-1);
	border: var(--border-width) solid var(--color-surface-2);
	border-radius: var(--radius-sm);
	--li-height: 50px;
	height: var(--li-height);
	flex-shrink: 0;
	display: flex;
	position: relative;
	width: 100%;
	box-sizing: border-box;
	transition:
		border-color var(--transition-fast),
		filter var(--transition-slow);
	filter: var(--shadow-sm);

	&:hover {
		filter: var(--shadow-md);

		& .btn-container {
			opacity: 1;
		}
	}
}
</style>
