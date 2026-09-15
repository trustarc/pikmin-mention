const KEY_LABELS: Record<string, string> = {
  Period: '.',
  Comma: ',',
  Slash: '/',
  Semicolon: ';',
  Quote: "'",
  Backslash: '\\',
  Backquote: '`',
  Minus: '-',
  Equal: '=',
  Space: 'Space',
};

const MAC_MODIFIERS: Record<string, string> = {
  Control: '⌃',
  Alt: '⌥',
  Shift: '⇧',
  Super: '⌘',
};

const OTHER_MODIFIERS: Record<string, string> = {
  Control: 'Ctrl',
  Alt: 'Alt',
  Shift: 'Shift',
  Super: 'Win',
};

export function formatHotkey(value: string, mac: boolean) {
  const modifiers = mac ? MAC_MODIFIERS : OTHER_MODIFIERS;

  return value
    .split('+')
    .map((part) => {
      const label = modifiers[part];
      if (label) {
        return label;
      }
      return (KEY_LABELS[part] ?? part).replace(/^(Key|Digit)/, '');
    })
    .join(mac ? ' ' : '+');
}

export function toShortcut(event: KeyboardEvent) {
  if (/^(Control|Alt|Shift|Meta)/.test(event.code)) {
    return null;
  }

  const modifiers: string[] = [];
  if (event.ctrlKey) modifiers.push('Control');
  if (event.altKey) modifiers.push('Alt');
  if (event.shiftKey) modifiers.push('Shift');
  if (event.metaKey) modifiers.push('Super');

  return modifiers.length ? [...modifiers, event.code].join('+') : null;
}
