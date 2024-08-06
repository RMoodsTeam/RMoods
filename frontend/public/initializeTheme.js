const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");
darkThemeMq.addEventListener('change', (e) => {
  if (e.matches) {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
});

if (localStorage.getItem("theme") === "light") {
  document.documentElement.classList.remove("dark");
} else if (localStorage.getItem("theme") === "dark") {
  document.documentElement.classList.add("dark");
} else {
  if (darkThemeMq.matches) {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
}
