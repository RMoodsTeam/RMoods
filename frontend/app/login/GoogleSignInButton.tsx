export default function GoogleSignInButton(params: { onClick: any }) {
  return (
    <button
      onClick={params.onClick}
      className="px-4 py-2 flex gap-2 rounded-full border"
    >
      <img
        className="w-6 h-6"
        src="https://www.svgrepo.com/show/475656/google-color.svg"
        loading="lazy"
        alt="google logo"
      />
      <span>Login with Google</span>
    </button>
  );
}
