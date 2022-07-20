
<script context="module">
    export async function load({url}) {
        const image = url.searchParams.get('image');
        if (image) {
            return {status: 200}; // OK
        }
        else {
            return {
                status: 300, // multiple choices
                redirect: url.pathname + '?image=0'
            };
        }
    }
    export function get_album_photo(album, image, captions) {
        let photo_array = Object.getOwnPropertyNames(captions);

        let photo_id = 0;
        if ((image >= 0) && (image < photo_array.length)) {
            photo_id = Number(image);
        }

        let last_id = (photo_id-1)>=0 ? photo_id-1 : photo_array.length-1;
        let next_id = (photo_id+1)==photo_array.length ? 0 : photo_id+1;
        let photo_name = photo_array[photo_id].trim();
        
        return {photo_name, photo_id, last_id, next_id};
    }
</script>

<script>
    import { page } from '$app/stores';
    import captions from '$lib/nihonnoyume.json';

    const image = $page.url.searchParams.get('image');

    export let album_name = 'nihonnoyume';
    export let { photo_name, photo_id, last_id, next_id } = 
        get_album_photo(album_name, image, captions);
    export let caption = captions[photo_name];
</script>

<div class="gallery">
    <div class="content">
        <img src={`/photos/${album_name}/${photo_name}`} alt='alt-text' />
    </div>
    <a href={`/photos/${album_name}/?image=${next_id}`} class="gallery">先</a>
    <a href={`/photos/${album_name}/?image=${last_id}`} class="gallery">次</a>
    <p class='caption'>{caption}</p>
</div>

<style>
    .gallery {
        position: absolute;
        top: 0;
        left: 0;
        padding: 0;
        margin: 0;
        height: 100%;
        width: 100%;
        align-content: center;
        opacity: 1;
    }
    .gallery img {
        /* Position image centered */
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;
        margin: auto;
        /* Fill the whole division */
        max-width: 100%;
        max-height: 100%;
    }
    .gallery a {
        position: relative;
        top: var(--header-height);
        margin-left: auto;
        margin-right: auto;
        width: 50%;
        text-align: center;
        display: inline-block;
        align-self: center;
        padding: 5px;
    }
</style>