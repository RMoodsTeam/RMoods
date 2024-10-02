// "use server";

// import { redirect } from "next/navigation";

// // Server action to get the JWT from our backend
// export async function postGoogleCode(codeResponse: { code: string }) {
//   // for test purposes it will stay at this URL for now
//   const url = "http://localhost:8001/auth/login";
//   const response = await fetch(url, {
//     method: "POST",
//     headers: { "Content-Type": "application/json" },
//     body: JSON.stringify({ code: codeResponse.code }),
//   });
//   const answer: { jwt: string; user_info: Object } = await response.json();
//   console.log(answer);
//   return answer;
// }

// // I really hope we need this garbage only here and I won't need to extract it upwards.
// // This is a hack to make the server redirect to the dashboard after the login.
// // `redirect` is not available in client components and `useRouter` seems unreliable, so we call `redirect` in this server action.
// export async function serverRedirect(path: string) {
//   redirect(path);
// }
