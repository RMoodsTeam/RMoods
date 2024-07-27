import Link from "next/link";
import React from "react";

/**
 * Login component for user to input their credentials
 */
const Login = () => {
  return (
    <div className="flex flex-col border-solid border-4 border-accent-purple m-auto h-32 w-64 content-evenly flex-wrap">
      <input type="email" name="email" id="email" autoFocus />
      <input type="password" name="password" id="password" />
      <Link href="/dashboard">
        <button>Login</button>
      </Link>
    </div>
  );
};

export default Login;
