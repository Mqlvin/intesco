<script lang="ts">
    import { getItemName, getStock, type StockLookupResult } from "../lib/stock";
    import StockItem from "../lib/components/StockItem.svelte";
    import { clearStore } from "../lib/cookies";

    const LOOKUP_SIZE: number = 20; // 20 items at a time

    export let cookie: string;
    const storeName = cookie.split(":")[0];
    const storeUuid = cookie.split(":")[1];

    // "last" lookup request performed. once performed, all products will be copied to a separate list
    let lookupRequest: StockLookupResult | string = "Search for an item";
    // let lookupRequest: StockLookupResult | string = JSON.parse('{"pageInformation":{"count":5,"matchType":"exact","pageNo":1,"query":{"searchTerm":"shell pasta"},"totalCount":5},"results":[{"availability":{"quantity":23,"status":"IN_STOCK"},"baseProductId":"50869969","defaultImageUrl":"https://digitalcontent.api.tesco.com/v2/media/ghs/710ee4bb-d4ac-429b-9a7a-85df969edcd1/178866b4-aef9-47d5-8b38-6116b940525c_747777021.jpeg?h=225&w=225","price":{"actual":1.19,"unitOfMeasure":"kg","unitPrice":1.19},"productLocation":[{"aisle":"23","aisleOrientation":"L"},{"aisle":"23","aisleOrientation":"L"}],"promotions":[],"title":"Tesco Conchiglie Pasta Shells 1Kg"},{"availability":{"quantity":15,"status":"IN_STOCK"},"baseProductId":"50695707","defaultImageUrl":"https://digitalcontent.api.tesco.com/v2/media/ghs/a7468716-6452-490d-a4e5-c5e7850e9317/1987b95f-a2d8-4565-8bb9-dea7cf87475d_909637556.jpeg?h=225&w=225","price":{"actual":0.75,"unitOfMeasure":"kg","unitPrice":1.5},"productLocation":[{"aisle":"23","aisleOrientation":"L"},{"aisle":"23","aisleOrientation":"L"}],"promotions":[],"title":"Tesco Conchiglie Pasta Shells 500G"},{"availability":{"quantity":23,"status":"IN_STOCK"},"baseProductId":"50687061","defaultImageUrl":"https://digitalcontent.api.tesco.com/v2/media/ghs/e8e6aca9-1192-4003-a0ee-385cbe312632/8a98fc16-d628-46c8-ab5c-d2d979473c6d_1043096732.jpeg?h=225&w=225","price":{"actual":0.75,"unitOfMeasure":"kg","unitPrice":1.5},"productLocation":[{"aisle":"23","aisleOrientation":"L"},{"aisle":"23","aisleOrientation":"L"}],"promotions":[],"title":"Tesco Farfalle Pasta Bows 500G"},{"availability":{"quantity":35,"status":"IN_STOCK"},"baseProductId":"71330699","defaultImageUrl":"https://digitalcontent.api.tesco.com/v2/media/ghs/c932452a-ca71-42e9-8aeb-dadc586ca2c9/fe255bcb-4a35-4699-836c-8aae8090f198_2144262375.jpeg?h=225&w=225","price":{"actual":1.85,"unitOfMeasure":"kg","unitPrice":37},"productLocation":[{"aisle":"23","aisleOrientation":"F"},{"aisle":"23","aisleOrientation":"F"}],"promotions":[],"title":"Cook Italia Grated Hard Cheese 50G"},{"availability":{"quantity":12,"status":"IN_STOCK"},"baseProductId":"92251490","defaultImageUrl":"https://digitalcontent.api.tesco.com/v2/media/ghs/56a18f6b-0dd7-4150-acfe-57d765bc6aed/33fea6da-93b1-46e0-9c27-a97f04f34e52_1053571143.jpeg?h=225&w=225","price":{"actual":2.15,"unitOfMeasure":"kg","unitPrice":4.3},"productLocation":[{"aisle":"23","aisleOrientation":"L"},{"aisle":"23","aisleOrientation":"L"}],"promotions":[{"endDate":"2026-09-15T23:00:00Z","offerText":"£1.60 Save 25% Clubcard Price","price":{"afterDiscount":2.15,"beforeDiscount":null},"promotionAttributes":["CLUBCARD_PRICING"],"promotionId":"102162852","promotionType":null,"startDate":"2026-07-31T23:00:00Z","unitSellingInfo":"£3.20/kg"}],"title":"Tesco Finest Conchiglie 500G 500G"}]}');
    let productList: TescoItem[] = [];
    let loadMoreAvailable: boolean = false;
    let searchTerm: string = "";

    async function refreshNewProducts(query: string) {
        lookupRequest = "Checking stock...";
        lookupRequest = await getStock(storeUuid, query, LOOKUP_SIZE, 1);
        if(typeof lookupRequest == "string") return; // an error

        loadMoreAvailable = false;
        productList = [];
        productList = productList.concat(lookupRequest.results);
        loadMoreAvailable = isAnotherPage();

        if(productList.length == 0) {
            lookupRequest = "No results found for query";
        }
    }

    async function loadNextPage(query: string) {
        if(typeof lookupRequest == "string") return;
        if(!isAnotherPage()) {
            console.error("Didn't fetch next page of items due to limit reached, button shouldn't be visible");
            return;
        }

        const nextPage = lookupRequest.pageInformation.pageNo + 1;
        lookupRequest = await getStock(storeUuid, query, LOOKUP_SIZE, nextPage);
        productList = productList.concat(lookupRequest.results);
        loadMoreAvailable = isAnotherPage();
    }

    function isAnotherPage() {
        if(typeof lookupRequest == "string") return;
        return lookupRequest.pageInformation.pageNo * LOOKUP_SIZE < lookupRequest.pageInformation.totalCount
    }

    async function searchItem(e) {
        searchTerm = e.target.value;

        loadMoreAvailable = false;
        productList = [];

        searchTerm = searchTerm.startsWith("https://") ? searchTerm.substring(8) : searchTerm
        searchTerm = searchTerm.startsWith("www.tesco.com") ? searchTerm.substring(4) : searchTerm
        // now starts with tesco.com/dotdotdot

        if(searchTerm == "") {
            lookupRequest = "Search for an item";
        } else if(searchTerm.startsWith("tesco.com/shop/en-GB/products/")) {
            const tpnc = searchTerm.substring(30);
            lookupRequest = "Fetching item name...";
            const itemName = await getItemName(tpnc);

            if(!itemName[1]) { // if err
                lookupRequest = "Error looking up item name for link";
                return;
            }

            lookupRequest = "Searching for " + itemName;
            searchTerm = itemName[0];
            refreshNewProducts(itemName[0]);
        } else {
            refreshNewProducts(searchTerm);
        }
    }

    function changeStore() {
        clearStore();
        window.location.href = "/";
    }
</script>

<div id="titlebar">
    <input id="product-search" type="text" placeholder="Search for a product..." on:change={searchItem} />
    <div>
        <div style="display: flex; font-size: 0.8em; font-weight: 600; color: #666; text-align: left; line-height: 0em; margin-top: 1.1em;"><p style="margin: 0;">Current Store</p> &nbsp; <a on:click={changeStore}>(change)</a></div>
        <p style="font-size: 1.2em; font-weight: 400; color: #000; text-align: left; line-height: 0em; margin-right: -1px;">{storeName}</p>
    </div>
</div>

<div id="stock-container">
    {#if lookupRequest != undefined && typeof lookupRequest != "string"}
        {#each productList as tescoItem}
            <StockItem itemData={tescoItem} />
        {/each}   
    {/if}
    {#if loadMoreAvailable}
        <div id="load-more">
            <button id="load-more-button" on:click={() => { loadNextPage(searchTerm); }}>Load more</button>
        </div>
    {/if}
</div>
{#if typeof lookupRequest == "string"}
    <p style="color: #666; font-size: 0.9em;">{lookupRequest}</p> <!-- status -->
{/if}

<br><br><br>

<style>
    #stock-container {
        width: fit-content;
        max-width: 95%;

        display: inline-grid;
        grid-template-columns: repeat(auto-fit, 300px);

        background: #ccc;
        gap: 1px;
        border: 1px solid #ccc;
        border-radius: 3px;

        box-shadow:
  0px 4px 6px -1px rgba(0, 0, 0, 0.1);
    }

    #load-more {
        background-color: #fff;
        width: 300px;
        min-height: 390px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }
    #load-more-button {
        background-color: #00539f;
        border: none;
        border-radius: 100px;
        width: 50%;
        height: 42px;

        color: white;
        font-weight: 600;
        font-family: "Tesco Modern";
        font-size: 1em;
        line-height: 10px;

        transition: 0.1s;

        box-shadow:
            0 0 0 2px white,
            0 0 0 3px white;
    }
    #load-more-button:hover {
        box-shadow:
            0 0 0 2px white,
            0 0 0 5px #00539f;
        cursor: pointer;
    }


    #titlebar {
        width: 80%;
        margin: 32px;

        display: flex;
        align-items: center;
        justify-content: center;

        gap: 48px;
    }
    @media (max-width: 1200px) {
        #titlebar {
            flex-direction: column-reverse;
            gap: 12px;
        }
        #product-search {
            width: 80% !important;
        }
    }

    #product-search {
        border: 3px solid #00539f;
        border-radius: 100px;
        background-color: white;

        width: 50%;
        max-width: 340px;
        height: 40px;

        font-size: 1em;
        font-family: "Tesco Modern";
        text-align: left;
        padding-left: 40px;

        box-shadow:
            0px 4px 6px -1px rgba(0, 0, 0, 0.1);

       background: white url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' class='bi bi-search' viewBox='0 0 16 16'%3E%3Cpath d='M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z'%3E%3C/path%3E%3C/svg%3E") no-repeat 14px center;
    }

    a {
        text-decoration: underline;
        cursor: pointer;
    }
</style>
