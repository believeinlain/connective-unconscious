<script>
	export let type;
</script>

<div class={type}>
	<slot />
</div>

<style>
	.header,
	.footer {
		box-sizing: border-box;
		margin: 0;
		position: fixed;
		left: 0;
		right: 0;
		width: 100%;
		background-color: rgba(17, 17, 17, 0.8);
		line-height: 0;
		padding: 10px;
		opacity: 0.8;
		z-index: 2;
	}
	.footer {
		bottom: 0;
		padding-top: 0;
		padding-bottom: 0;
	}
	.header {
		top: 0;
		text-align: center;
		display: grid;
		height: var(--header-height);
	}
</style>
