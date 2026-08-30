<script lang="ts">
    export let size: string = "4em";
</script>

<div style="display: flex; flex-direction: column; align-items: center;">
    <br>
    <br>
    <div style="display: flex;">
        <h1 class="title" style="font-size: {size}; color: #00539f;">in.</h1>
        <h1 class="title" style="font-size: {size}; color: #e81c2d;">tesco?</h1>
    </div>
    <h3 class="url">Check store stock @ intesco.mqlvin.dev</h3>
    <br>
</div>

<style>
    .title {
        font-family: "Tesco Modern", serif;
        font-weight: 600;
        font-style: normal;
        text-align: center;
    }

    .url {
        font-weight: 600;
        opacity: 0.5;
        text-align: center;
        font-size: 1em;
        transform: translateY(-5px);
    }

    h1, h3 {
        margin: 0;
    }
</style>
