## in.tesco?
in.tesco? allows you to check item stock and location within any Tesco store.

Simply enter your location, select your store, and search for your product by name or Tesco link!

<b>Note:</b><br>
If a search does not yield a specific item listing, the store does not ever stock that product. The `Out of Stock` label indicates the store may have stocked it in the past or plans to stock it, but is out of stock.<br>Hence, for example, you won't see a "loose carrots" listing in a petrol station.

---

## Screenshots


---

## Small technical analysis

Tesco internally uses GraphQL for various internal systems, including stock lookup.

Decompiling the official Tesco mobile app exposed various various API endpoints, including the `searchProduct` query which allows me to query product stock and aisle location within the store.

This endpoint is unauthenticated (how ideal) so you can indeed use it without any account.

> If anyone is interested in other Tesco API endpoints, decompile the APK with jadx and do a string search for `query ` - you can see 100+ endpoints.
