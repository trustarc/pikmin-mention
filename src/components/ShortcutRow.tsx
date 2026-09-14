import { useEffect, useRef } from "react";
import type { Shortcut } from "../types";

type Props = {
  shortcut: Shortcut;
  index: number;
  selected: boolean;
  pinned: boolean;
  accelerator: number | null;
  followSelection: boolean;
  dragging: boolean;
  dropTarget: boolean;
  onActivate: () => void;
  onHover: () => void;
  onTogglePin: () => void;
  onRemove: () => void;
  onDragStart: () => void;
  onDragOver: () => void;
  onDrop: () => void;
  onDragEnd: () => void;
};

export default function ShortcutRow({
  shortcut,
  selected,
  pinned,
  accelerator,
  followSelection,
  dragging,
  dropTarget,
  onActivate,
  onHover,
  onTogglePin,
  onRemove,
  onDragStart,
  onDragOver,
  onDrop,
  onDragEnd,
}: Props) {
  const ref = useRef<HTMLLIElement>(null);

  useEffect(() => {
    if (selected && followSelection) {
      ref.current?.scrollIntoView({ block: "nearest" });
    }
  }, [selected, followSelection]);

  return (
    <li
      ref={ref}
      draggable
      onDragStart={onDragStart}
      onDragOver={(event) => {
        event.preventDefault();
        onDragOver();
      }}
      onDrop={(event) => {
        event.preventDefault();
        onDrop();
      }}
      onDragEnd={onDragEnd}
      onMouseEnter={onHover}
      onClick={onActivate}
      className={`group flex cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 ${
        selected ? "bg-white/10" : ""
      } ${dragging ? "opacity-30" : ""} ${dropTarget ? "border-t border-white/40" : "border-t border-transparent"}`}
    >
      <button
        type="button"
        aria-label={pinned ? "Unpin" : "Pin"}
        onClick={(event) => {
          event.stopPropagation();
          onTogglePin();
        }}
        className={`shrink-0 text-sm ${
          pinned ? "text-amber-300" : "text-white/20 hover:text-white/50"
        }`}
      >
        {pinned ? "★" : "☆"}
      </button>

      <span className="flex-1 truncate text-sm text-white/80">{shortcut.label}</span>

      {accelerator ? (
        <kbd className="shrink-0 rounded border border-hairline bg-white/5 px-1.5 py-0.5 font-mono text-xs text-white/40">
          ⌘{accelerator}
        </kbd>
      ) : null}

      {shortcut.custom ? (
        <button
          type="button"
          aria-label="Delete"
          onClick={(event) => {
            event.stopPropagation();
            onRemove();
          }}
          className="shrink-0 text-xs text-white/20 opacity-0 group-hover:opacity-100 hover:text-red-400"
        >
          ✕
        </button>
      ) : null}
    </li>
  );
}
