import { useState } from "react";
import ShortcutRow from "./ShortcutRow";
import type { Shortcut } from "../types";

type Props = {
  shortcuts: Shortcut[];
  selectedId: string | null;
  pinned: string[];
  followSelection: boolean;
  onActivate: (shortcut: Shortcut) => void;
  onHover: (shortcut: Shortcut) => void;
  onTogglePin: (shortcut: Shortcut) => void;
  onRemove: (shortcut: Shortcut) => void;
  onReorder: (id: string, toIndex: number) => void;
};

export default function ShortcutList({
  shortcuts,
  selectedId,
  pinned,
  followSelection,
  onActivate,
  onHover,
  onTogglePin,
  onRemove,
  onReorder,
}: Props) {
  const [draggingId, setDraggingId] = useState<string | null>(null);
  const [overIndex, setOverIndex] = useState<number | null>(null);

  return (
    <ul className="flex flex-col">
      {shortcuts.map((shortcut, index) => (
        <ShortcutRow
          key={shortcut.id}
          shortcut={shortcut}
          index={index}
          selected={shortcut.id === selectedId}
          pinned={pinned.includes(shortcut.id)}
          accelerator={index < 9 ? index + 1 : null}
          followSelection={followSelection}
          dragging={draggingId === shortcut.id}
          dropTarget={draggingId !== null && overIndex === index && draggingId !== shortcut.id}
          onActivate={() => onActivate(shortcut)}
          onHover={() => onHover(shortcut)}
          onTogglePin={() => onTogglePin(shortcut)}
          onRemove={() => onRemove(shortcut)}
          onDragStart={() => setDraggingId(shortcut.id)}
          onDragOver={() => setOverIndex(index)}
          onDrop={() => {
            if (draggingId && draggingId !== shortcut.id) {
              onReorder(draggingId, index);
            }
            setDraggingId(null);
            setOverIndex(null);
          }}
          onDragEnd={() => {
            setDraggingId(null);
            setOverIndex(null);
          }}
        />
      ))}
    </ul>
  );
}
