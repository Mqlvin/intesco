<script lang="ts">
    export let promotionObj;
    export let originalPrice;

    let clubcardPromotions = promotionObj.filter((obj) => obj.promotionAttributes && obj.promotionAttributes[0] == "CLUBCARD_PRICING");

    const promotion: any | undefined = clubcardPromotions.length > 0 ? clubcardPromotions[0] : undefined;

    function formatEndDate(isoString: string): string {
        if(isoString == undefined || isoString == "") return "unknown";

        const date = new Date(isoString);
        const day = date.getUTCDate().toString();
        const month = date.getUTCMonth().toString();
        const year = date.getUTCFullYear().toString();
        return `${day.length == 1 ? "0" + day : day}/${month.length == 1 ? "0" + month : month}/${year.substring(year.length - 2)}`; // this code won't be valid in year 3000
    }
</script>

{#if promotion != undefined}
    <div id="container">
        <div id="clubcard-text">
            Clubcard<br>Price
        </div>
        <div id="clubcard-info">
            {#if promotion.price == null || originalPrice == promotion.price.afterDiscount}
                <p id="new-price" style="line-height: 1.25em; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; text-overflow: ellipsis; display: -webkit-box;">{promotion.offerText}</p>
            {:else}
                <p id="new-price"><b>£{promotion.price.afterDiscount.toFixed(2)} until {formatEndDate(promotion.endDate)}</p>
                <p id="new-unit-price">({promotion.unitSellingInfo})</p>
            {/if}
        </div>
    </div>
    <div id="deal-ends">
        <p>Offer valid until {formatEndDate(promotion.endDate)}</p>
    </div>
{/if}

<style>
    p { margin: 0; }

    #container {
        display: flex;
        gap: 4px;

        width: 100%;
        height: 56px;
        
        border-radius: 12px;
        overflow: hidden;

        margin: 10px 0px 0px 0px;
    }

    #clubcard-text {
        background-color: #00539f;
        min-width: 56px;
        
        color: white;
        font-size: 0.60em;
        text-align: center;
        font-weight: 600;
        line-height: 1em;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    #clubcard-info {
        background-color: #fcd700;
        flex-grow: 1;

        padding: 8px 12px 8px 8px;

        display: flex;
        align-items: center;
    }

    #new-price {
        font-weight: 600;
    }

    #new-unit-price {
        font-size: 0.75em;
        color: #333;
        line-height: 1em;
    }

    #deal-ends {
        font-size: 0.75em;
        color: #333;
        text-align: left;
        width: 100%;

        margin: -6px 0px 7px 0px;
    }
</style>
