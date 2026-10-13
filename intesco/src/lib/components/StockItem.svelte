<script lang="ts">
    import { type TescoItem } from "../stock";
    import ClubcardPricing from "./ClubcardPricing.svelte";

    import { onMount } from "svelte";
    import tippy from "tippy.js";
    import "tippy.js/dist/tippy.css";
    import "tippy.js/themes/light.css";

    export let itemData: TescoItem;

    // maps stock status + quantity to indicator text + hex colour etc "#000000"
    function mapStockStatus(status: string, quantity: number): [string, string]  {
        switch(status) {
            case "IN_STOCK": { 
                return [`In stock (${quantity} left)`, "#00539f"];
            }
            case "LOW_STOCK": {
                return [`Low stock (${quantity} left)`, "#ac6b35"];
            }
            case "OUT_OF_STOCK": {
                return [`Out of Stock`, "#444"];
            }
            case "UNKNOWN": {
                return [`Unknown`, "#444"];
            }
        }
        return [`Unknown`, "#444"];
    }

    function getAisles(productLocation: any): string[] {
        if(productLocation && productLocation.length == 0) return ["Unknown"];
        return Array.from(new Set(productLocation.map((p) => { return `Aisle ${p.aisle} (${p.aisleOrientation})` })))
    }
    function getLocations(productLocation: any): string[] { // get first shelf position only
        let locations: Set<string> = new Set();

        for(const idx in productLocation) {
            const name = `Aisle ${productLocation[idx].aisle} (${productLocation[idx].aisleOrientation})`;
            locations.add(`${name} - Module ${productLocation[idx].moduleNumber}, ${productLocation[idx].productPosition} up`);
        }

        return Array.from(locations);
    }

    const stockIndicator = mapStockStatus(itemData.availability.status, itemData.availability.quantity);
    const aisles = getAisles(itemData.productLocation);
    const locations = getLocations(itemData.productLocation);

    let locationElement: HTMLElement;
    onMount(() => {
        tippy(locationElement, {
            content: `
<div class="location-popup">
    <p style="font-size: 1.2em; font-weight: 300; color: #666;" class="popup-text">${locations.length > 1 ? "All locations:" : "Location:"}</p>
    <p class="popup-text" style="font-size: 1.1em; font-weight: 600; color: #333; border-left: 2px solid #ddd; padding-left: 8px; margin-top: 8px;">
        <b>${locations.join("<br>")}</b>
    </p>
</div>
            `,
            allowHTML: true,
            placement: "top-start",
            duration: [150, 250],
            theme: "light",
            trigger: "click",
            interactive: true
        });
    });
</script>

<div id="container">
    <br>
    <img id="item-image" src={itemData.defaultImageUrl} alt="The associated Tesco product" />
    <div id="meta-container">
        <a target="_blank" id="item-link" href="https://www.tesco.com/shop/en-GB/products/{itemData.id}">{itemData.title}</a>
        <div id="item-location" bind:this={locationElement}>
            <span style="font-weight: 300;">Location{aisles.length > 1 ? "s" : ""}:</span>
            <span style="font-weight: 600;">{aisles.join(", ")}</span>

            <svg style="margin: 0; padding: 0; transform: translateY(1px);" xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
                <path d="M0 0h24v24H0z" fill="none" />
                <path fill="currentColor" d="M12 22c5.51 0 10-4.49 10-10S17.51 2 12 2S2 6.49 2 12s4.49 10 10 10M11 7h2v2h-2zm0 4h2v6h-2z" />
            </svg>
        </div>
        {#if itemData.promotions && itemData.promotions.length != 0}
            <ClubcardPricing promotionObj={itemData.promotions} originalPrice={itemData.price.actual} />
        {/if}
        <div id="bottom-meta">
            <div id="price-container">
                <p id="item-price">£{itemData.price.actual.toFixed(2)}</p>
                <p id="item-ppu">£{itemData.price.unitPrice.toFixed(2)}/{itemData.price.unitOfMeasure}</p>
            </div>
            <div id="status-container" style="background-color: {stockIndicator[1]};">
                <p id="item-availability">{stockIndicator[0]}</p>
            </div>
        </div>
    </div>
</div>

<style>
    #container {
        background-color: #fff;

        width: 300px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    #item-image {
        display: inline;
        max-height: 285px;
        max-width: 228px;

        min-height: 225px;
    }

    #meta-container {
        width: 90%;
        padding: 24px 0px 24px 0px;

        display: flex;
        flex-direction: column;

        gap: 8px;
    }

    #item-link {
        text-decoration: none;
        width: 100%;
        color: #00539f;
        flex-grow: 1;
    }
    #item-link:hover {
        text-decoration: underline;
    }

    #bottom-meta {
        display: flex;
    }

    #price-container {
        width: 40%;
    }
    #item-price {
        font-weight: 600;
        color: #444;
        margin: 0;
    }
    #item-ppu {
        color: #666;
        margin: 0;
        font-size: 0.75em;
    }

    #status-container {
        width: 60%;
        display: flex;
        align-items: center;
        justify-content: center;

        border-radius: 100px;
        user-select: none;

        box-shadow: 0px 4px 6px -1px rgba(0, 0, 0, 0.05);
    }
    #item-availability {
        margin: 0;
        text-align: center;
        color: white;

        font-weight: 600;
        line-height: 10px;
    }

    #item-location {
        font-size: 0.9em;
        opacity: 0.7;
        cursor: pointer;

        padding-top: 4px;
        padding-bottom: 4px;
    }

    :global(.location-popup) {
        max-height: 200px;
        overflow-y: auto;

        padding: 12px;
    }

    :global(p.popup-text) {
        margin: 0;
        padding: 0;
    }

</style>
