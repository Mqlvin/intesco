<script lang="ts">
    export let storeListingObj;
    const name = storeListingObj["name"];
    const location = storeListingObj["address"]["city"];
    const postcode = storeListingObj["address"]["postalCode"];
    const address = storeListingObj["address"]["line1"];
    const distanceKm = storeListingObj["distance"]["distanceKilometers"];

    export let onClickCallback;
</script>

<button id="container" on:click={onClickCallback}>
    <div id="left">
        <p id="name">{name}</p>
        <p id="address">{address}</p>
        <p id="location">{location}, {postcode}</p>
    </div>
    <div id="right">
        <p id="distance">{distanceKm.toFixed(1)}km</p>
    </div>
</button>

<style>
    p {
        margin: 0;
    }

    #container {
        border: 1px solid #0002;
        border-radius: 2px;

        width: 90%;
        max-width: 340px;
        display: flex;

        background-color: white;

        font-family: "Tesco Modern";
        font-size: 1em;

        padding: 10px 20px 10px 20px;
        margin-bottom: 12px;
    }

    #container:hover {
        border: 1px solid #0004;
        cursor: pointer;

        background-color: #fafafa;
    }

    #left {
        width: 80%;
        text-align: left;
    }

    #right {
        width: 20%;
        text-align: right;
    }

    #name {
        font-size: 1.2em;
        font-weight: 400;
    }
    #address {
        margin-bottom: 10px;
    }
    #location, #distance {
        font-size: 0.9em;
        font-weight: 600;
        opacity: 0.6;
        margin-bottom: 4px;
    }
</style>
