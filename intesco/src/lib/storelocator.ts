import { API_URL } from "./api";
import { RequestResult } from "./result";

export interface TescoStore {
    address: any, // todo flesh out into proper obj definition
    distance: any,
    name: string
}

export async function getStoresAtCoordinates(lat: number, long: number): Promise<TescoStore[] | string> {
    const url = new URL(`${API_URL}/storelocator/${lat}/${long}`);

    try {
        const response = await fetch(url, {
            headers: { Accept: "application/json" },
        });

        const resultObj = new RequestResult<any>(await response.text());
        if(resultObj.isErr()) {
            return resultObj.error_reason as string;
        }

        return resultObj.data["entities"] as TescoStore[];
    } catch(error) {
        const message = error instanceof Error ? error.message : String(error);
        return message;
    }
}


