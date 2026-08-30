export const cnameStore = "currentStore";

export function getStore(): string | undefined {
    let decodedCookie = decodeURIComponent(document.cookie);

    if(decodedCookie.length == 0) {
        return undefined;
    }

    let cookies = decodedCookie.split(";");
    console.log(cookies)
    for(const c in cookies) {
        if(cookies[c].startsWith(cnameStore)) {
            return cookies[c].substring(cnameStore.length + 1); // + 1 removes =
        }
    }

    return undefined;
}

export function setStore(name: string, uuid: string) {
    let d = new Date();
    d.setTime(d.getTime() + (7 * 86400000)); // expire in 7 days
    let cexpires = `expires=${d.toUTCString()}`;
    let cvalue = `${name}:${uuid}`;

    document.cookie = `${cnameStore}=${cvalue};${cexpires}`;

    console.log(`Selected ${name} with UUID:${uuid}`);
}

export function clearStore() {
    document.cookie = `${cnameStore}=; expires=Thu, 01 Jan 1970 00:00:00 UTC;`;
}
