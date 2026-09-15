import type { Snippet } from '@/types';
import { Star, X } from 'lucide-react';
import { useEffect, useRef } from 'react';

type Props = {
  snippet: Snippet;
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

export default function SnippetRow({
  snippet,
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
      ref.current?.scrollIntoView({ block: 'nearest' });
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
      className={`group flex cursor-pointer items-center gap-1.5 rounded-md py-1.5 pr-2 pl-1 ${
        selected
          ? 'bg-white/12 ring-1 ring-white/20 ring-inset'
          : 'hover:bg-white/5'
      } ${dragging ? 'opacity-30' : ''} ${dropTarget ? 'border-t border-white/40' : 'border-t border-transparent'}`}
    >
      <button
        type="button"
        aria-label={pinned ? 'Unpin' : 'Pin'}
        onClick={(event) => {
          event.stopPropagation();
          onTogglePin();
        }}
        className={`-my-1 flex size-8 shrink-0 items-center justify-center rounded-md hover:bg-white/10 ${
          pinned ? 'text-amber-300' : 'text-white/25 hover:text-white/60'
        }`}
      >
        <Star className="size-4" fill={pinned ? 'currentColor' : 'none'} />
      </button>

      <span className="flex-1 truncate text-sm text-white/80">
        {snippet.label}
      </span>

      {accelerator ? (
        <kbd className="border-hairline shrink-0 rounded border bg-white/5 px-1.5 py-0.5 font-mono text-xs text-white/40">
          ⌘{accelerator}
        </kbd>
      ) : null}

      {snippet.custom ? (
        <button
          type="button"
          aria-label="Delete"
          onClick={(event) => {
            event.stopPropagation();
            onRemove();
          }}
          className="hover:text-destructive -my-1 flex size-8 shrink-0 items-center justify-center rounded-md text-white/25 opacity-0 group-hover:opacity-100 hover:bg-white/10"
        >
          <X className="size-4" />
        </button>
      ) : null}
    </li>
  );
}
