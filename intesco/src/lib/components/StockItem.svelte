<script lang="ts">
    import { type TescoItem } from "../stock";
    import ClubcardPricing from "./ClubcardPricing.svelte";

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
        return Array.from(new Set(productLocation.map((p) => { return `${p.aisle} (${p.aisleOrientation})` })))
    }

    const stockIndicator = mapStockStatus(itemData.availability.status, itemData.availability.quantity);
    const aisles = getAisles(itemData.productLocation);

</script>

<div id="container">
    <br>
    <img id="item-image" src={itemData.defaultImageUrl} alt="The associated Tesco product" />
    <div id="meta-container">
        <a target="_blank" id="item-link" href="https://www.tesco.com/shop/en-GB/products/{itemData.id}">{itemData.title}</a>
        <div id="item-location">
            <span style="font-weight: 300;">Aisle{aisles.length > 1 ? "s" : ""}:</span>
            <span style="font-weight: 600;">{aisles.join(", ")}</span>
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
    }

</style>
