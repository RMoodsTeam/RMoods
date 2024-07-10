import Link from "next/link";
import React from "react";

const Login = () => {
  return (
    <div>
      <input type="email" name="email" id="email" autoFocus />
      <input type="password" name="password" id="password" />
      <Link href="/dashboard">
        <button>Login</button>
      </Link>
    </div>
  );
};

export default Login;
