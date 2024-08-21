import React from "react";

export default function GoogleSignInButton(params: { onClick: any }) {
  return (
    <button
      onClick={params.onClick}
      className="px-4 py-2 flex gap-2 rounded-full border bg-primary-light flex items-center justify-center text-primary-dark hover:bg-primary-dark hover:text-primary-light transition-colors"
      id="google-sign-in-button"
    >
      <img
        className="w-6 h-6"
        src="https://www.svgrepo.com/show/475656/google-color.svg"
        loading="lazy"
        alt="google logo"
      />
      <span id="google-sign-in-button-text">Sign in with Google</span>
    </button>
  );
}
