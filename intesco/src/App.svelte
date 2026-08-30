<script lang="ts">
    import { onMount } from "svelte";
    import { cnameStore, getStore } from "./lib/cookies";

    import StockViewer from "./routes/StockViewer.svelte";
    import StoreSelector from "./routes/StoreSelector.svelte";
    let page: "index" | "store" | "selector" = "index";

    onMount(() => {
        let store = getStore()
        if(store != undefined) {
            sessionStorage.setItem(cnameStore, store)
            page = "store";
            return;
        }

        sessionStorage.setItem(cnameStore, "undefined")
        page = "selector";
    })
</script>

<div id="pageContainer">
    {#if page == "store"}
        <StockViewer cookie={sessionStorage.getItem(cnameStore)!!}/>
    {:else if page == "selector"}
        <StoreSelector />
    {:else if page == "index"}
        <p>Redirecting...</p>
    {/if}
</div>
