import SnippetRow from '@/components/SnippetRow';
import type { Snippet } from '@/types';
import { useState } from 'react';

type Props = {
  snippets: Snippet[];
  selectedId: string | null;
  pinned: string[];
  followSelection: boolean;
  onActivate: (snippet: Snippet) => void;
  onHover: (snippet: Snippet) => void;
  onTogglePin: (snippet: Snippet) => void;
  onRemove: (snippet: Snippet) => void;
  onReorder: (id: string, toIndex: number) => void;
};

export default function SnippetList({
  snippets,
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
      {snippets.map((snippet, index) => (
        <SnippetRow
          key={snippet.id}
          snippet={snippet}
          index={index}
          selected={snippet.id === selectedId}
          pinned={pinned.includes(snippet.id)}
          accelerator={index < 9 ? index + 1 : null}
          followSelection={followSelection}
          dragging={draggingId === snippet.id}
          dropTarget={
            draggingId !== null &&
            overIndex === index &&
            draggingId !== snippet.id
          }
          onActivate={() => onActivate(snippet)}
          onHover={() => onHover(snippet)}
          onTogglePin={() => onTogglePin(snippet)}
          onRemove={() => onRemove(snippet)}
          onDragStart={() => setDraggingId(snippet.id)}
          onDragOver={() => setOverIndex(index)}
          onDrop={() => {
            if (draggingId && draggingId !== snippet.id) {
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
