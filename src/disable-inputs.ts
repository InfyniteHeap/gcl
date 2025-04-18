export function disableFunctionKeys() {
  globalThis.addEventListener("keydown", (event) => {
    if (/^(F[1-9]|F1[0-2])$/.test(event.key)) {
      event.preventDefault();
    }
  });
}

export function disableMouseButtons() {
  globalThis.addEventListener("auxclick", (event) => {
    event.preventDefault();
  });
  globalThis.addEventListener("contextmenu", (event) => {
    event.preventDefault();
  });
}
