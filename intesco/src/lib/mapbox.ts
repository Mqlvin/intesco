export interface MapboxResult {
    relevance: number,
    place_name: string,
    center: [number, number],
}

export async function getRelevantLocations(searchTerm: string): Promise<MapboxResult[] | string> {
    const params = new URLSearchParams({
        access_token: "pk.eyJ1IjoieWV4dCIsImEiOiJqNzVybUhnIn0.hTOO5A1yqfpN42-_z_GuLw",
        language: "en",
        limit: "5",
        country: "GB",
        types: "place,region,district,locality",
    });

    const url = `https://api.mapbox.com/geocoding/v5/mapbox.places/${encodeURIComponent(searchTerm)}.json?${params}`;

    try {
        const response = await fetch(url);

        if(!response.ok) {
            return `Mapbox HTTP request failed, code ${response.status}`;
        }

        const data = await response.json();
        const relevantData = (data["features"] as any[]).map((d) => {
            return {
                "relevance": d["relevance"],
                "place_name": d["place_name"],
                "center": [d["center"][0], d["center"][1]],
            } as MapboxResult
        });
        return relevantData;
    } catch(err) {
        return err as string;
    }
}
