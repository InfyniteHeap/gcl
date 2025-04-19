export function disableFunctionKeys() {
  onkeydown = (event) => {
    if (/^(F[1-9]|F1[0-2])$/.test(event.key)) {
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
