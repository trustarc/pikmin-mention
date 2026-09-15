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

const MODIFIER_LABELS: Record<string, string> = {
  Control: '⌃',
  Alt: '⌥',
  Shift: '⇧',
  Super: '⌘',
};

export function formatHotkey(value: string) {
  return value
    .split('+')
    .map((part) => {
      const label = MODIFIER_LABELS[part];
      if (label) {
        return label;
      }
      return (KEY_LABELS[part] ?? part).replace(/^(Key|Digit)/, '');
    })
    .join(' ');
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
