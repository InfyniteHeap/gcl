export function disableKeyboardShortcuts() {
  addEventListener("keydown", (event) => {
    if (
      ((event.ctrlKey || event.metaKey) &&
        ["f", "g", "j", "p", "r", "u"].includes(event.key)) ||
      (event.altKey && ["ArrowLeft", "ArrowRight"].includes(event.key))
    ) {
      event.preventDefault();
    }
  });
}

export function disableFunctionKeys() {
  addEventListener("keydown", (event) => {
    if (/^(F[1-9]|F1[0-2])$/.test(event.key)) {
      event.preventDefault();
    }
  });
}

export function disableMouseButtons() {
  addEventListener("auxclick", (event) => {
    event.preventDefault();
  });
  addEventListener("contextmenu", (event) => {
    event.preventDefault();
  });
}
