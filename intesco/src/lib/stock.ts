import { API_URL } from "./api"
import { RequestResult } from "./result"

export interface TescoItem {
    availability: {
        quantity: number,
        status: string
    },
    id: number,
    defaultImageUrl: string,
    price: {
        actual: number,
        unitOfMeasure: string,
        unitPrice: number
    },
    productLocation: {
        aisle: string,
        aisleOrientation: string
    }[],
    promotions: {
        endDate: string,
        offerText: string,
        price: {
            afterDiscount: number,
        },
        promotionAttributes: string[],
        unitSellingInfo: string
    }[],
    title: string,
}

export interface StockLookupResult {
    pageInformation: {
        count: number,
        pageNo: number,
        totalCount: number,
    },
    results: TescoItem[],
}


export async function getStock(storeUuid: string, query: string, maxResults: number, page: number): Promise<StockLookupResult | string> {
    const url = new URL(`${API_URL}/stock`);

    try {
        const response = await fetch(url, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ store_uuid: storeUuid, query, max_results: maxResults, page: page }),
        });

        if(!response.ok) {
            return `Tesco stock search HTTP failed, code ${response.status}`;
        }

        const resultObj = new RequestResult<any>(await response.text());
        if(!resultObj.success) {
            return resultObj.error_reason as string;
        }

        return resultObj.data["search"] as StockLookupResult;
    } catch(error) {
        const message = error instanceof Error ? error.message : String(error);
        return message;
    }
}   

export async function getItemName(tpnc: string): Promise<[string, boolean]> {
    const url = new URL(`${API_URL}/name`);

    try {
        const response = await fetch(url, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ tpnc }),
        });

        if(!response.ok) {
            return [`Tesco item name request HTTP failed, code ${response.status}`, false];
        }

        const resultObj = new RequestResult<any>(await response.text());
        if(!resultObj.success) {
            return [resultObj.error_reason as string, false];
        }

        return [resultObj.data as string, true];
    } catch(error) {
        const message = error instanceof Error ? error.message : String(error);
        return [message, false];
    }

}
