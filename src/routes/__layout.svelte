<div class="content">
    <slot></slot>
</div>

<style>
    :root {
        --header-height: 100px;
        background-image: url('/images/home_bkg.jpg');
        background-attachment: fixed;
        background-size: cover;
        height: 100%;
        font-family: Helvetica, Arial, sans-serif;
        color: #ccc;
    }
    :global(a:link, a:visited) {
        text-decoration: none;
    }
    :global(a:hover, a:active) {
        text-decoration: underline;
    }
    :global(a:hover) {
        color: rgb(176, 224, 224);
    }
    :global(a:link) {
        color: rgb(102, 213, 228);
    }
    :global(a:visited) {
        color: rgb(102, 130, 206);
    }
    .content {
        background-color: rgba(17, 17, 17, 0.8);
        max-width: 800px;
        margin: auto;
        padding: 2%;
        padding-top: var(--header-height);
    }
</style>