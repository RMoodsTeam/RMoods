// TODO!
// import React from "react";

// export default function ThemeSwitch() {
//   const { mode, setMode } = useColorScheme();
//   const [mounted, setMounted] = React.useState(false);

//   // necessary for server-side rendering
//   // because mode is undefined on the server
//   React.useEffect(() => {
//     setMounted(true);
//   }, []);
//   if (!mounted) {
//     return null;
//   }

//   return (
//     <Select
//       value={mode}
//       onChange={(_, newMode) => {
//         setMode(newMode);
//       }}
//       sx={{ width: "max-content" }}
//     >
//       <Option value="system">System</Option>
//       <Option value="light">Light</Option>
//       <Option value="dark">Dark</Option>
//     </Select>
//   );
// }
