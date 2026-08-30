<script lang="ts">
    import "../app.css";
    import Title from "../lib/components/Title.svelte";
    import StoreListing from "../lib/components/StoreListing.svelte";
    import { type MapboxResult, getRelevantLocations } from "../lib/mapbox";
    import { type TescoStore, getStoresAtCoordinates } from "../lib/storelocator";
    import { setStore } from "../lib/cookies";

    let mapboxResult: MapboxResult[] | string = ""//[{"relevance":1,"place_name":"Coventry, West Midlands, England, United Kingdom","center":[-1.510477,52.408181]},{"relevance":1,"place_name":"Coverack, Cornwall, England, United Kingdom","center":[-5.097118,50.023136]},{"relevance":1,"place_name":"Covehithe, Suffolk, England, United Kingdom","center":[1.706762,52.376545]},{"relevance":1,"place_name":"Cove, Scottish Borders, Scotland, United Kingdom","center":[-2.353621,55.93836]},{"relevance":1,"place_name":"Coven, Staffordshire, England, United Kingdom","center":[-2.135652,52.658344]}];
    let tescoStores: TescoStore[] | undefined = undefined//JSON.parse('[{"address":{"city":"Coventry","countryCode":"GB","extraDescription":null,"line1":"29 High Street","line2":null,"line3":null,"postalCode":"CV1 5RE","region":null,"sublocality":null},"distance":{"distanceKilometers":0.11977374106187992,"distanceMiles":0.07442395228234604,"id":"46d1d1d6-1bc7-426a-9324-4866fd604d2c"},"name":"Coventry High St Express"},{"address":{"city":"Coventry","countryCode":"GB","extraDescription":null,"line1":"23 Shelton Square","line2":null,"line3":null,"postalCode":"CV1 1DG","region":null,"sublocality":null},"distance":{"distanceKilometers":0.26193168162840813,"distanceMiles":0.16275680129817374,"id":"d4be4181-5b95-401a-9283-9da20433191b"},"name":"Coventry Shelton Square Express"},{"address":{"city":"Coventry","countryCode":"GB","extraDescription":null,"line1":"Unit 1, Bishopsgate","line2":"Tower Street","line3":null,"postalCode":"CV1 1JN","region":null,"sublocality":null},"distance":{"distanceKilometers":0.4315738178476105,"distanceMiles":0.26816753773438773,"id":"77386f6f-dcf9-4e41-9073-f8331c334dc0"},"name":"Coventry Bishopsgate Express"},{"address":{"city":"Coventry","countryCode":"GB","extraDescription":null,"line1":"255-259 Walsgrave Rd","line2":null,"line3":null,"postalCode":"CV2 4BA","region":null,"sublocality":null},"distance":{"distanceKilometers":1.9944999896997908,"distanceMiles":1.2393248365171092,"id":"c52260c9-4e34-4f0b-9d48-e1323741e05d"},"name":"Ball Hill Walsgrave Rd Express"},{"address":{"city":"Coventry","countryCode":"GB","extraDescription":null,"line1":"489 Holyhead Rd","line2":null,"line3":null,"postalCode":"CV5 8HU","region":null,"sublocality":null},"distance":{"distanceKilometers":2.5856436658222437,"distanceMiles":1.6066444873328782,"id":"1d54a931-b87a-4925-9508-7b380045c2ff"},"name":"Coventry Holyhead Express"},{"address":{"city":"Coventry","countryCode":"GB","extraDescription":null,"line1":"Jubilee Crescent","line2":null,"line3":null,"postalCode":"CV6 3EX","region":null,"sublocality":null},"distance":{"distanceKilometers":2.6508039119641067,"distanceMiles":1.6471331871645258,"id":"c4ac6168-66b9-4469-9396-1391a20c05b1"},"name":"Coventry Jubilee Crescent Express"},{"address":{"city":"Coventry","countryCode":"GB","extraDescription":null,"line1":"22 Cannon Park Centre","line2":"Lynchgate Road","line3":null,"postalCode":"CV4 7EH","region":null,"sublocality":null},"distance":{"distanceKilometers":3.824820325001655,"distanceMiles":2.376633165439866,"id":"d8a9603c-c21a-4ee9-b866-854dc95b6038"},"name":"Cannon Park Centre Superstore"},{"address":{"city":"Coventry","countryCode":"GB","extraDescription":null,"line1":"Arena Retail Park","line2":"Classic Drive","line3":null,"postalCode":"CV6 6AS","region":null,"sublocality":null},"distance":{"distanceKilometers":4.196085690485467,"distanceMiles":2.6073267682269714,"id":"4f5e9f15-b13d-4dea-bc3a-eec1431fa0f9"},"name":"Coventry Arena Extra"},{"address":{"city":"Coventry","countryCode":"GB","extraDescription":null,"line1":"Clifford Bridge Road","line2":null,"line3":null,"postalCode":"CV2 2TS","region":null,"sublocality":null},"distance":{"distanceKilometers":4.74896872702668,"distanceMiles":2.9508723598103823,"id":"913637f9-022a-4802-89cd-0e48ebeb0034"},"name":"Coventry Walsgrave Superstore"},{"address":{"city":"Coventry","countryCode":"GB","extraDescription":null,"line1":"Olivier Way","line2":"Cross Point Business Park","line3":null,"postalCode":"CV2 2SH","region":null,"sublocality":null},"distance":{"distanceKilometers":5.988465537832144,"distanceMiles":3.7210599709149466,"id":"275c3018-da44-49d5-8a75-60b5ab9e70b6"},"name":"Coventry Cross Point Extra"}]') as TescoStore[];
    let coordinates: [number, number] | undefined = undefined;

    async function textChanged(e) {
        if(e.target.value.length >= 3) {
            const result: MapboxResult[] | string = await getRelevantLocations(e.target.value);
            if(typeof result == "string") {
                mapboxResult = result;
            } else {
                mapboxResult = result as MapboxResult[];
            }
        } else if(e.target.value.length == 0 && !(mapboxResult instanceof String)) {
            mapboxResult = "Enter a postcode...";
        }
    }

    async function selectCoordinates(locationResult: MapboxResult) {
        coordinates = locationResult.center;
        console.log("New coordinates: " + coordinates);

        mapboxResult = "Fetching stores...";
        tescoStores = undefined;

        let storeSearchResult = await getStoresAtCoordinates(coordinates[0], coordinates[1]);
        if(typeof storeSearchResult == "string") { // error
            tescoStores = undefined;
        } else {
            tescoStores = storeSearchResult;
        }

        document.getElementById("location-input").textContent = ""
        mapboxResult = "";
    }

    function selectStore(storeListingObj: TescoStore) {
        setStore(storeListingObj.name, storeListingObj.distance["id"]);
        window.location.href = "/";
    }
</script>



<Title size="3em" />

<input id="location-input" type="text" on:input={textChanged} placeholder="Enter a place..." autocomplete="off" />
{#if typeof mapboxResult == "string" && mapboxResult != ""}
    <p>{mapboxResult}</p>
{:else if mapboxResult.length == 0 && typeof mapboxResult != "string"}
    <div id="mapbox-result-container">
        <p>No results</p>
    </div>
{:else if typeof mapboxResult != "string" && mapboxResult.length > 0 && mapboxResult[0].length != 1}
    <div id="mapbox-result-container">
        {#each mapboxResult as locationResult}
            <button class="mapbox-result" on:click={async () => { await selectCoordinates(locationResult) }}>{locationResult.place_name}</button>
        {/each}
    </div>
{/if}

<br>
{#if tescoStores != undefined}
    {#each tescoStores as store}
        <StoreListing storeListingObj={store} onClickCallback={() => {selectStore(store)}} />
    {/each}
{/if}

<br>



<style>
    #location-input {
        border: 3px solid #00539f;
        border-radius: 100px;
        background-color: white;

        width: 90%;
        max-width: 340px;
        height: 40px;

        font-size: 1em;
        font-family: "Tesco Modern";
        text-align: center;

        box-shadow:
            0px 4px 6px -1px rgba(0, 0, 0, 0.1);
    }

    #mapbox-result-container {
        width: 90%;
        max-width: 340px;

        margin-top: 4px;

        font-size: 1em;
        font-family: "Tesco Modern";
        text-align: center;

        overflow: hidden;
        border-radius: 3px;

        box-shadow:
            0px 4px 6px -1px rgba(0, 0, 0, 0.1);
        border: 1px solid #ddd;
    }

    .mapbox-result {
        font-size: 1em;
        font-family: "Tesco Modern";
        text-align: center;

        width: 100%;

        background-color: #fff;
        border: none;
        border-bottom: 1px solid #ddd;

        padding: 10px 8px 10px 8px;
    }
    .mapbox-result:last-child {
        border-bottom: none;
    }
    .mapbox-result:hover {
        cursor: pointer;
        background-color: #eee;
    }

</style>
