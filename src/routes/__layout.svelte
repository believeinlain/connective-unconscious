
<header class="header">
    <h1>Connective Unconscious</h1>
    <div class="navbar">
        <a href="/">Home</a>
        <a href="/gallery/nihonnoyume">日本の夢</a>
        <a href="/gallery/jikannoyume">時間の夢</a>
        <a href="/planetgen">🪐</a>
    </div>
</header>

<div class="content">
<!-- Pages will be injected here -->
    <slot></slot>
</div>

<br/>

<footer class="footer">
    <p>Website by Stephanie Aelmore. Source available
        <a href="https://github.com/believeinlain/connective-unconscious">here</a>
        under the MIT License.
    </p>
</footer>

<style>
    :root {
        --header-height: 90px;
        background-image: url('/images/home_bkg.jpg');
        background-attachment: fixed;
        background-size: cover;
        height: 100%;
        font-family: Helvetica, Arial, sans-serif;
        color: #ccc;
    }
    :global(h1) {
        font-size: 30px;
    }
    :global(a:link) {
        text-decoration: none;
        background-color: initial;
        color: rgb(102, 213, 228);
    }
    :global(a:visited) {
        text-decoration: none;
        color: rgb(102, 130, 206);
    }
    :global(a:hover) {
        text-decoration: underline;
        color: rgb(176, 224, 224);
    }
    :global(a:active) {
        text-decoration: underline;
    }
    .content {
        background-color: rgba(17, 17, 17, 0.8);
        max-width: 800px;
        margin: auto;
        padding: 10px;
        padding-top: var(--header-height);
        height: 100%;
    }
    .header, .footer {
        box-sizing: border-box;
        margin: 0;
        position: fixed;
        left: 0;
        right: 0;
        width: 100%;
        background-color: rgba(17, 17, 17, 0.8);
        color: #ccc;
        z-index: 2;
        line-height: 0px;
        padding: 10px;
        opacity: 0.8;
    }
    .header {
        top: 0;
        text-align: center;
        height: var(--header-height);
        display: grid;
        line-height: 0;
    }
    .navbar {
        display: flex;
        flex-direction: row;
        height: 100%;
        margin: auto;
    }
    .navbar a {
        display: block;
        float: left;
        width: fit-content;
        height: fit-content;
        padding: 15px;
        margin: 2px;
        border: 1px solid #222;
        background-color: rgba(34, 34, 34, 0.8);
        text-decoration: none;
    }
    .navbar a:link {
        border-color: #888;
        color: #eee;
    }
    .navbar a:visited {
        border-color: #555;
        color: #ddd;
    }
    .navbar a:hover {
        color: #fff;
        border-color: #fff;
    }
    .footer {
        bottom: 0;
        padding-top: 0;
        padding-bottom: 0;
        height: fit-content;
    }
</style>