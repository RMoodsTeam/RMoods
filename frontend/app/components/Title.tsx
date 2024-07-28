export default function Title(props: { text: string }) {
  return (
    <h1 className="text-3xl font-bold text-gray-900 dark:text-gray-100">
      {props.text}
    </h1>
  );
}
