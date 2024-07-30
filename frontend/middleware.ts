import { NextResponse } from "next/server";
import { NextRequest } from "next/server";
import { jwtVerify } from "jose";

export async function middleware(request: NextRequest) {
  // ***DO NOT CHANGE*** prevent rerouting NextJS assets like CSS
  if (request.nextUrl.pathname.startsWith("/_next")) {
    return NextResponse.next();
  }

  // get pathname as we dont want to interrupt user if he wants to login
  const { pathname } = request.nextUrl;

  console.log(pathname);
  if (pathname === "/login") {
    return NextResponse.next();
  }

  // get token and then check if it's valid, if not, redirect to /login
  const token = request.cookies.get("RMOODS_JWT");

  if (!token) {
    return NextResponse.redirect(new URL("/login", request.url));
  }

  // TODO: change missing env handling
  if (!process.env.JWT_SECRET) {
    console.error("JWT_SECRET not defined in the environment.");
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 },
    );
  }

  try {
    await jwtVerify(
      token.value,
      new TextEncoder().encode(process.env.JWT_SECRET),
    );
  } catch {
    return NextResponse.redirect(new URL("/login", request.url));
  }
  return NextResponse.next();
}
