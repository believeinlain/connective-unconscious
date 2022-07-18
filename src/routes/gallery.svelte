
<script context="module">
    export async function load({ params, fetch, session, stuff }) {
        const url = `./gallery/${params}`;
        const response = await fetch(url);

        console.log(params);

        return {
            status: response.status,
            props: {
                article: response.ok && (await response.json())
            }
        };
    }
    function serve_photo_album(album_name, display_name) {
        // read the list of captions
        var captions = JSON.parse(fs.readFileSync(`./public/${album_name}/captions.json`, 'utf-8'));
        var photo_array = Object.getOwnPropertyNames(captions);
        
        router.get(`/${album_name}/`, function(req, res, next) {
            var image = req.query['image'];
            if (!image) {
            res.redirect(`/${album_name}/?image=0`);
            return;
            }
            var photo_id = 0;
            if ((image >= 0) && (image < photo_array.length)) {
            photo_id = Number(image);
            }
            var last_id = (photo_id-1)>=0 ? photo_id-1 : photo_array.length-1;
            var next_id = (photo_id+1)==photo_array.length ? 0 : photo_id+1; 
            var photo_name = photo_array[photo_id].trim();
            // draw the image page
            res.render('photo_album', { 
            album: album_name,
            title: display_name,
            photo: photo_name,
            caption: captions[photo_name],
            no_caption: captions[photo_name]=='' ? ', no_caption' : '',
            photo_id: photo_id,
            last_id: last_id,
            next_id: next_id});
        });
    };
</script>

<div class="gallery">

</div>