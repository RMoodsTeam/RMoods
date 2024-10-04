// import { jwtDecrypt, jwtVerify } from "jose";
// import { cookies } from "next/headers";

// export async function userInfoFromJWT(): Promise<any> {
//   // TODO maybe use caching wisely to avoid calling it every time we render the dashboard
//   const token = cookies().get("RMOODS_JWT")?.value;
//   if (!token) {
//     console.error("No JWT token found");
//     throw new Error("No JWT token found");
//   }

//   // TODO: change missing env handling
//   if (!process.env.JWT_SECRET) {
//     console.error("JWT_SECRET not defined in the environment.");
//     throw new Error("JWT_SECRET not defined in the environment.");
//   }

//   const key = new TextEncoder().encode(process.env.JWT_SECRET);
//   try {
//     const { payload } = await jwtVerify(token, key);
//     return payload.user_info as any;
//   } catch (error) {
//     console.error("JWT token could not be decoded.");
//     throw new Error("JWT token could not be decoded.");
//   }
// }
