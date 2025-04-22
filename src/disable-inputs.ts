export function disableKeyboardShortcuts() {
  onkeydown = (event) => {
    if (
      ((event.ctrlKey || event.metaKey) &&
        ["f", "g", "j", "p", "r", "u"].includes(event.key)) ||
      (event.altKey && ["ArrowLeft", "ArrowRight"].includes(event.key)) ||
      (/^(F[1-9]|F1[0-2])$/.test(event.key))
    ) {
      event.preventDefault();
    }
  };
}

export function disableMouseButtons() {
  onauxclick = (event) => {
    event.preventDefault();
  };
  oncontextmenu = (event) => {
    event.preventDefault();
  };
}
