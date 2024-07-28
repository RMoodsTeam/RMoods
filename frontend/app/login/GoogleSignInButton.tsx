export default function GoogleSignInButton(params: { onClick: any }) {
  return (
    <button
      className="flex items-center justify-center w-full px-4 py-2 text-white bg-red-600 rounded-md"
      onClick={params.onClick}
    >
      Sign in with Google
    </button>
  );
}
