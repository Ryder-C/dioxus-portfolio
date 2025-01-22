const keyframes = [
  {
    opacity: 0,
    backgroundColor: "clear",
  },
  {
    opacity: 1,
    backgroundColor: "var(--text-color)",
    color: "var(--primary-background)",
  },
  {
    backgroundColor: "clear",
  },
];

function entrance() {
  let elements = Array.from(document.querySelectorAll(".animate")).flatMap(
    (container) => Array.from(container.children),
  );

  elements
    .filter((element) => {
      const { top, bottom } = element.getBoundingClientRect();
      return bottom > 0 && top < window.innerHeight;
    })
    .forEach((element, i) => {
      element.animate(keyframes, {
        duration: 100,
        easing: "steps(2, start)",
        delay: 100 * i + 400,
        fill: "both",
      });
    });
}

setTimeout(entrance, 100);
