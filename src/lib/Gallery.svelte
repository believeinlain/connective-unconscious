<script context="module">
	export async function load({ url }) {
		const image = url.searchParams.get('image');
		if (image) {
			return { status: 200 }; // OK
		} else {
			return {
				status: 300, // multiple choices
				redirect: url.pathname + '?image=0'
			};
		}
	}
	export function get_album_photo(image, captions) {
		let photo_array = Object.getOwnPropertyNames(captions);

		let photo_id = 0;
		if (image >= 0 && image < photo_array.length) {
			photo_id = Number(image);
		}

		let last_id = photo_id - 1 >= 0 ? photo_id - 1 : photo_array.length - 1;
		let next_id = photo_id + 1 == photo_array.length ? 0 : photo_id + 1;
		let photo_name = photo_array[photo_id].trim();

		return { photo_name, photo_id, last_id, next_id };
	}
</script>

<script>
	export let album_name;

	import { page } from '$app/stores';
	import captions from '$lib/captions.json';

	const album_captions = captions[album_name];

	const image = $page.url.searchParams.get('image');

	export let { photo_name, photo_id, last_id, next_id } = get_album_photo(image, album_captions);
	export let caption = album_captions[photo_name];
</script>

<div class="nav">
	<a href="/{album_name}/?image={last_id}" target="_self">先</a>
	<a href="/{album_name}/?image={next_id}" target="_self">次</a>
</div>

<div class="gallery">
	<img src="/photos/{album_name}/{photo_name}" alt={caption} title={caption} />
</div>

<style>
	.gallery {
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		opacity: 1;
		z-index: 0;
	}
	.gallery img {
		/* Position image centered */
		position: absolute;
		margin: auto;
		/* Fill the whole division */
		top: 0;
		bottom: 0;
		left: 0;
		right: 0;
		max-width: 100%;
		max-height: 100%;
	}
	.nav {
		top: var(--header-height);
		display: inline-flex;
		height: fit-content;
		width: 100%;
		position: absolute;
		left: 0;
		z-index: 3;
	}
	a {
		top: var(--header-height);
		height: fit-content;
		width: 50%;
		display: inline-block;
		text-align: center;
		border: 1px solid #222;
		padding: 5px;
		background-color: rgba(34, 34, 34, 0.8);
	}
	a:link,
	a:visited {
		border-color: #555;
		color: #ddd;
	}
	a:hover {
		color: #fff;
		border-color: #fff;
		text-decoration: none;
	}
</style>
