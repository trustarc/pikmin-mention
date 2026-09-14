import type { Pack } from "../types";

type Props = {
  packs: Pack[];
  activeId: string | null;
  matchedId: string | null;
  onSelect: (id: string) => void;
};

export default function PackTabs({ packs, activeId, matchedId, onSelect }: Props) {
  return (
    <nav className="flex gap-1 border-b border-hairline px-3 py-2">
      {packs.map((pack) => {
        const active = pack.id === activeId;
        return (
          <button
            key={pack.id}
            type="button"
            onClick={() => onSelect(pack.id)}
            className={`flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium transition-colors ${
              active ? "bg-white/15 text-white" : "text-white/45 hover:bg-white/5 hover:text-white/70"
            }`}
          >
            {pack.name}
            {pack.id === matchedId ? <span className="size-1 rounded-full bg-emerald-400" /> : null}
          </button>
        );
      })}
    </nav>
  );
}
