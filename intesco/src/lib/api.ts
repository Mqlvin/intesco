export const IS_DEV = import.meta.env.MODE === "development";

export const API_URL: string = IS_DEV
  ? "http://localhost:4000/api"
  : "https://intesco.mqlvin.dev/api";

