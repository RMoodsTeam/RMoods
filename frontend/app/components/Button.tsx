export default function Button({ children, ...props }: any) {
  return (
    <button
      className="bg-accent-purple hover:bg-accent-green text-white font-bold py-2 px-4 rounded"
      {...props}
    >
      {children}
    </button>
  );
}
